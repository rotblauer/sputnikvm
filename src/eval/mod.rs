//! VM Runtime
use bigint::{H256, M256, U256, Gas, Address};
use super::commit::{AccountState, BlockhashState};
use super::errors::{RequireError, RuntimeError, CommitError, EvalOnChainError,
                    OnChainError, NotSupportedError};
use super::{Stack, Context, HeaderParams, Patch, PC, Memory, AccountCommitment, Log};

use self::check::{check_opcode, check_support, extra_check_opcode};
use self::run::run_opcode;
use self::cost::{gas_refund, gas_stipend, gas_cost, memory_cost, memory_gas};

mod cost;
mod run;
mod check;
mod util;
mod lifecycle;

/// A VM state without PC.
pub struct State<M> {
    /// Memory of this runtime.
    pub memory: M,
    /// Stack of this runtime.
    pub stack: Stack,

    /// Context.
    pub context: Context,
    /// Block header.
    pub block: HeaderParams,

    /// The current out value.
    pub out: Vec<u8>,

    /// The current memory cost. Note that this is different from
    /// memory gas.
    pub memory_cost: Gas,
    /// Used gas excluding memory gas.
    pub used_gas: Gas,
    /// Refunded gas.
    pub refunded_gas: Gas,

    /// The current account commitment states.
    pub account_state: AccountState,
    /// The current blockhash commitment states.
    pub blockhash_state: BlockhashState,
    /// Logs appended.
    pub logs: Vec<Log>,
    /// Removed accounts using the SUICIDE opcode.
    pub removed: Vec<Address>,

    /// Depth of this runtime.
    pub depth: usize,
}

impl<M> State<M> {
    /// Memory gas, part of total used gas.
    pub fn memory_gas(&self) -> Gas {
        memory_gas(self.memory_cost)
    }

    /// Available gas at this moment.
    pub fn available_gas(&self) -> Gas {
        self.context.gas_limit - self.memory_gas() - self.used_gas
    }

    /// Total used gas including the memory gas.
    pub fn total_used_gas(&self) -> Gas {
        self.memory_gas() + self.used_gas
    }
}

/// A VM state with PC.
pub struct Machine<M, P: Patch> {
    state: State<M>,
    pc: PC<P>,
    status: MachineStatus,
}

#[derive(Debug, Clone)]
/// Represents the current runtime status.
pub enum MachineStatus {
    /// This runtime is actively running or has just been started.
    Running,
    /// This runtime has exited successfully. Calling `step` on this
    /// runtime again would panic.
    ExitedOk,
    /// This runtime has exited with errors. Calling `step` on this
    /// runtime again would panic.
    ExitedErr(OnChainError),
    ExitedNotSupported(NotSupportedError),
    /// This runtime requires execution of a sub runtime, which is a
    /// ContractCreation instruction.
    InvokeCreate(Context),
    /// This runtime requires execution of a sub runtime, which is a
    /// MessageCall instruction.
    InvokeCall(Context, (U256, U256)),
}

#[derive(Debug, Clone)]
/// Used for `check` for additional checks related to the runtime.
pub enum ControlCheck {
    Jump(M256),
}

#[derive(Debug, Clone)]
/// Used for `step` for additional operations related to the runtime.
pub enum Control {
    Stop,
    Jump(M256),
    InvokeCreate(Context),
    InvokeCall(Context, (U256, U256)),
}

impl<M: Memory + Default, P: Patch> Machine<M, P> {
    /// Create a new runtime.
    pub fn new(context: Context, block: HeaderParams, depth: usize) -> Self {
        Self::with_states(context, block, depth,
                          AccountState::default(), BlockhashState::default())
    }

    /// Create a new runtime with the given states.
    pub fn with_states(context: Context, block: HeaderParams,
                       depth: usize, account_state: AccountState,
                       blockhash_state: BlockhashState) -> Self {
        Machine {
            pc: PC::new(context.code.as_slice()),
            status: MachineStatus::Running,
            state: State {
                memory: M::default(),
                stack: Stack::default(),

                context,
                block,

                out: Vec::new(),

                memory_cost: Gas::zero(),
                used_gas: Gas::zero(),
                refunded_gas: Gas::zero(),

                account_state,
                blockhash_state,
                logs: Vec::new(),
                removed: Vec::new(),

                depth,
            },
        }
    }

    /// Derive this runtime to create a sub runtime. This will not
    /// modify the current runtime, and it will have a chance to
    /// review whether it wants to accept the result of this sub
    /// runtime afterwards.
    pub fn derive(&self, context: Context) -> Self {
        Machine {
            pc: PC::new(context.code.as_slice()),
            status: MachineStatus::Running,
            state: State {
                memory: M::default(),
                stack: Stack::default(),

                context: context,
                block: self.state.block.clone(),

                out: Vec::new(),

                memory_cost: Gas::zero(),
                used_gas: Gas::zero(),
                refunded_gas: Gas::zero(),

                account_state: self.state.account_state.clone(),
                blockhash_state: self.state.blockhash_state.clone(),
                logs: self.state.logs.clone(),
                removed: self.state.removed.clone(),

                depth: self.state.depth + 1,
            },
        }
    }

    /// Commit a new account into this runtime.
    pub fn commit_account(&mut self, commitment: AccountCommitment) -> Result<(), CommitError> {
        self.state.account_state.commit(commitment)
    }

    /// Commit a new blockhash into this runtime.
    pub fn commit_blockhash(&mut self, number: U256, hash: H256) -> Result<(), CommitError> {
        self.state.blockhash_state.commit(number, hash)
    }

    /// Step a precompiled runtime. This function returns true if the
    /// runtime is indeed a precompiled address. Otherwise return
    /// false with state unchanged.
    pub fn step_precompiled(&mut self) -> bool {
        for precompiled in P::precompileds() {
            if self.state.context.address == precompiled.0 &&
                (precompiled.1.is_none() || precompiled.1.unwrap() == self.pc.code())
            {
                let data = &self.state.context.data;
                match precompiled.2.gas_and_step(data, self.state.context.gas_limit) {
                    Err(RuntimeError::OnChain(err)) => {
                        self.state.used_gas = self.state.context.gas_limit;
                        self.status = MachineStatus::ExitedErr(err);
                    },
                    Err(RuntimeError::NotSupported(err)) => {
                        self.status = MachineStatus::ExitedNotSupported(err);
                    },
                    Ok((gas, ret)) => {
                        assert!(gas <= self.state.context.gas_limit);
                        self.state.used_gas = gas;
                        self.state.out = ret;
                        self.status = MachineStatus::ExitedOk;
                    }
                }
                return true;
            }
        }
        return false;
    }

    /// Step an instruction in the PC. The eval result is refected by
    /// the runtime status, and it will only return an error if
    /// there're accounts or blockhashes to be committed to this
    /// runtime for it to run. In that case, the state of the current
    /// runtime will not be affected.
    pub fn step(&mut self) -> Result<(), RequireError> {
        match &self.status {
            &MachineStatus::Running => (),
            _ => panic!(),
        }

        if self.step_precompiled() {
            return Ok(());
        }

        if self.pc.is_end() {
            self.status = MachineStatus::ExitedOk;
            return Ok(());
        }

        let instruction = match self.pc.peek() {
            Ok(val) => val,
            Err(err) => {
                self.status = MachineStatus::ExitedErr(err);
                return Ok(())
            },
        };

        match check_opcode(instruction, &self.state).and_then(|v| {
            match v {
                None => Ok(()),
                Some(ControlCheck::Jump(dest)) => {
                    if dest <= M256::from(usize::max_value()) && self.pc.is_valid(dest.as_usize()) {
                        Ok(())
                    } else {
                        Err(OnChainError::BadJumpDest.into())
                    }
                }
            }
        }) {
            Ok(()) => (),
            Err(EvalOnChainError::OnChain(error)) => {
                self.status = MachineStatus::ExitedErr(error);
                return Ok(());
            },
            Err(EvalOnChainError::Require(error)) => {
                return Err(error);
            },
        }

        let position = self.pc.position();
        let memory_cost = memory_cost(instruction, &self.state);
        let memory_gas = memory_gas(memory_cost);
        let gas_cost = gas_cost::<M, P>(instruction, &self.state);
        let gas_stipend = gas_stipend(instruction, &self.state);
        let gas_refund = gas_refund(instruction, &self.state);

        let all_gas_cost = memory_gas + self.state.used_gas + gas_cost;
        if self.state.context.gas_limit < all_gas_cost {
            self.status = MachineStatus::ExitedErr(OnChainError::EmptyGas);
            return Ok(());
        }

        match check_support(instruction, &self.state) {
            Ok(()) => (),
            Err(err) => {
                self.status = MachineStatus::ExitedNotSupported(err);
                return Ok(());
            },
        };

        let after_gas = self.state.context.gas_limit - all_gas_cost;

        match extra_check_opcode::<M, P>(instruction, &self.state, gas_stipend, after_gas) {
            Ok(()) => (),
            Err(err) => {
                self.status = MachineStatus::ExitedErr(err);
                return Ok(());
            },
        }

        let instruction = self.pc.read().unwrap();
        let result = run_opcode::<M, P>((instruction, position),
                                        &mut self.state, gas_stipend, after_gas);

        self.state.used_gas = self.state.used_gas + gas_cost - gas_stipend;
        self.state.memory_cost = memory_cost;
        self.state.refunded_gas = self.state.refunded_gas + gas_refund;

        match result {
            None => Ok(()),
            Some(Control::Jump(dest)) => {
                self.pc.jump(dest.as_usize()).unwrap();
                Ok(())
            },
            Some(Control::InvokeCall(context, (from, len))) => {
                self.status = MachineStatus::InvokeCall(context, (from, len));
                Ok(())
            },
            Some(Control::InvokeCreate(context)) => {
                self.status = MachineStatus::InvokeCreate(context);
                Ok(())
            },
            Some(Control::Stop) => {
                self.status = MachineStatus::ExitedOk;
                Ok(())
            },
        }
    }

    /// Get the runtime state.
    pub fn state(&self) -> &State<M> {
        &self.state
    }

    /// Get the runtime PC.
    pub fn pc(&self) -> &PC<P> {
        &self.pc
    }

    /// Get the current runtime status.
    pub fn status(&self) -> MachineStatus {
        self.status.clone()
    }
}
