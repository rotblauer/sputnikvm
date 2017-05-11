extern crate jsontests;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

use serde_json::Value;
use jsontests::test_transaction;

lazy_static! {
    static ref TESTS: Value =
        serde_json::from_str(include_str!("files/vmIOandFlowOperationsTest.json")).unwrap();
}

#[test] fn BlockNumberDynamicJump0_AfterJumpdest() { assert_eq!(test_transaction("BlockNumberDynamicJump0_AfterJumpdest", &TESTS["BlockNumberDynamicJump0_AfterJumpdest"], true), true); }
#[test] fn BlockNumberDynamicJump0_AfterJumpdest3() { assert_eq!(test_transaction("BlockNumberDynamicJump0_AfterJumpdest3", &TESTS["BlockNumberDynamicJump0_AfterJumpdest3"], true), true); }
#[test] fn BlockNumberDynamicJump0_foreverOutOfGas() { assert_eq!(test_transaction("BlockNumberDynamicJump0_foreverOutOfGas", &TESTS["BlockNumberDynamicJump0_foreverOutOfGas"], true), true); }
#[test] fn BlockNumberDynamicJump0_jumpdest0() { assert_eq!(test_transaction("BlockNumberDynamicJump0_jumpdest0", &TESTS["BlockNumberDynamicJump0_jumpdest0"], true), true); }
#[test] fn BlockNumberDynamicJump0_jumpdest2() { assert_eq!(test_transaction("BlockNumberDynamicJump0_jumpdest2", &TESTS["BlockNumberDynamicJump0_jumpdest2"], true), true); }
#[test] fn BlockNumberDynamicJump0_withoutJumpdest() { assert_eq!(test_transaction("BlockNumberDynamicJump0_withoutJumpdest", &TESTS["BlockNumberDynamicJump0_withoutJumpdest"], true), true); }
#[test] fn BlockNumberDynamicJump1() { assert_eq!(test_transaction("BlockNumberDynamicJump1", &TESTS["BlockNumberDynamicJump1"], true), true); }
#[test] fn BlockNumberDynamicJumpInsidePushWithJumpDest() { assert_eq!(test_transaction("BlockNumberDynamicJumpInsidePushWithJumpDest", &TESTS["BlockNumberDynamicJumpInsidePushWithJumpDest"], true), true); }
#[test] fn BlockNumberDynamicJumpInsidePushWithoutJumpDest() { assert_eq!(test_transaction("BlockNumberDynamicJumpInsidePushWithoutJumpDest", &TESTS["BlockNumberDynamicJumpInsidePushWithoutJumpDest"], true), true); }
#[test] fn BlockNumberDynamicJumpi0() { assert_eq!(test_transaction("BlockNumberDynamicJumpi0", &TESTS["BlockNumberDynamicJumpi0"], true), true); }
#[test] fn BlockNumberDynamicJumpi1() { assert_eq!(test_transaction("BlockNumberDynamicJumpi1", &TESTS["BlockNumberDynamicJumpi1"], true), true); }
#[test] fn BlockNumberDynamicJumpi1_jumpdest() { assert_eq!(test_transaction("BlockNumberDynamicJumpi1_jumpdest", &TESTS["BlockNumberDynamicJumpi1_jumpdest"], true), true); }
#[test] fn BlockNumberDynamicJumpiAfterStop() { assert_eq!(test_transaction("BlockNumberDynamicJumpiAfterStop", &TESTS["BlockNumberDynamicJumpiAfterStop"], true), true); }
#[test] fn BlockNumberDynamicJumpiOutsideBoundary() { assert_eq!(test_transaction("BlockNumberDynamicJumpiOutsideBoundary", &TESTS["BlockNumberDynamicJumpiOutsideBoundary"], true), true); }
#[test] fn BlockNumberDynamicJumpifInsidePushWithJumpDest() { assert_eq!(test_transaction("BlockNumberDynamicJumpifInsidePushWithJumpDest", &TESTS["BlockNumberDynamicJumpifInsidePushWithJumpDest"], true), true); }
#[test] fn BlockNumberDynamicJumpifInsidePushWithoutJumpDest() { assert_eq!(test_transaction("BlockNumberDynamicJumpifInsidePushWithoutJumpDest", &TESTS["BlockNumberDynamicJumpifInsidePushWithoutJumpDest"], true), true); }
#[test] fn DyanmicJump0_outOfBoundary() { assert_eq!(test_transaction("DyanmicJump0_outOfBoundary", &TESTS["DyanmicJump0_outOfBoundary"], true), true); }
#[test] fn DynamicJump0_AfterJumpdest() { assert_eq!(test_transaction("DynamicJump0_AfterJumpdest", &TESTS["DynamicJump0_AfterJumpdest"], true), true); }
#[test] fn DynamicJump0_AfterJumpdest3() { assert_eq!(test_transaction("DynamicJump0_AfterJumpdest3", &TESTS["DynamicJump0_AfterJumpdest3"], true), true); }
#[test] fn DynamicJump0_foreverOutOfGas() { assert_eq!(test_transaction("DynamicJump0_foreverOutOfGas", &TESTS["DynamicJump0_foreverOutOfGas"], true), true); }
#[test] fn DynamicJump0_jumpdest0() { assert_eq!(test_transaction("DynamicJump0_jumpdest0", &TESTS["DynamicJump0_jumpdest0"], true), true); }
#[test] fn DynamicJump0_jumpdest2() { assert_eq!(test_transaction("DynamicJump0_jumpdest2", &TESTS["DynamicJump0_jumpdest2"], true), true); }
#[test] fn DynamicJump0_withoutJumpdest() { assert_eq!(test_transaction("DynamicJump0_withoutJumpdest", &TESTS["DynamicJump0_withoutJumpdest"], true), true); }
#[test] fn DynamicJump1() { assert_eq!(test_transaction("DynamicJump1", &TESTS["DynamicJump1"], true), true); }
#[test] fn DynamicJumpAfterStop() { assert_eq!(test_transaction("DynamicJumpAfterStop", &TESTS["DynamicJumpAfterStop"], true), true); }
#[test] fn DynamicJumpInsidePushWithJumpDest() { assert_eq!(test_transaction("DynamicJumpInsidePushWithJumpDest", &TESTS["DynamicJumpInsidePushWithJumpDest"], true), true); }
#[test] fn DynamicJumpInsidePushWithoutJumpDest() { assert_eq!(test_transaction("DynamicJumpInsidePushWithoutJumpDest", &TESTS["DynamicJumpInsidePushWithoutJumpDest"], true), true); }
#[test] fn DynamicJumpJD_DependsOnJumps0() { assert_eq!(test_transaction("DynamicJumpJD_DependsOnJumps0", &TESTS["DynamicJumpJD_DependsOnJumps0"], true), true); }
#[test] fn DynamicJumpJD_DependsOnJumps1() { assert_eq!(test_transaction("DynamicJumpJD_DependsOnJumps1", &TESTS["DynamicJumpJD_DependsOnJumps1"], true), true); }
#[test] fn DynamicJumpPathologicalTest0() { assert_eq!(test_transaction("DynamicJumpPathologicalTest0", &TESTS["DynamicJumpPathologicalTest0"], true), true); }
#[test] fn DynamicJumpPathologicalTest1() { assert_eq!(test_transaction("DynamicJumpPathologicalTest1", &TESTS["DynamicJumpPathologicalTest1"], true), true); }
#[test] fn DynamicJumpPathologicalTest2() { assert_eq!(test_transaction("DynamicJumpPathologicalTest2", &TESTS["DynamicJumpPathologicalTest2"], true), true); }
#[test] fn DynamicJumpPathologicalTest3() { assert_eq!(test_transaction("DynamicJumpPathologicalTest3", &TESTS["DynamicJumpPathologicalTest3"], true), true); }
#[test] fn DynamicJumpStartWithJumpDest() { assert_eq!(test_transaction("DynamicJumpStartWithJumpDest", &TESTS["DynamicJumpStartWithJumpDest"], true), true); }
#[test] fn DynamicJump_value1() { assert_eq!(test_transaction("DynamicJump_value1", &TESTS["DynamicJump_value1"], true), true); }
#[test] fn DynamicJump_value2() { assert_eq!(test_transaction("DynamicJump_value2", &TESTS["DynamicJump_value2"], true), true); }
#[test] fn DynamicJump_value3() { assert_eq!(test_transaction("DynamicJump_value3", &TESTS["DynamicJump_value3"], true), true); }
#[test] fn DynamicJump_valueUnderflow() { assert_eq!(test_transaction("DynamicJump_valueUnderflow", &TESTS["DynamicJump_valueUnderflow"], true), true); }
#[test] fn DynamicJumpi0() { assert_eq!(test_transaction("DynamicJumpi0", &TESTS["DynamicJumpi0"], true), true); }
#[test] fn DynamicJumpi1() { assert_eq!(test_transaction("DynamicJumpi1", &TESTS["DynamicJumpi1"], true), true); }
#[test] fn DynamicJumpi1_jumpdest() { assert_eq!(test_transaction("DynamicJumpi1_jumpdest", &TESTS["DynamicJumpi1_jumpdest"], true), true); }
#[test] fn DynamicJumpiAfterStop() { assert_eq!(test_transaction("DynamicJumpiAfterStop", &TESTS["DynamicJumpiAfterStop"], true), true); }
#[test] fn DynamicJumpiOutsideBoundary() { assert_eq!(test_transaction("DynamicJumpiOutsideBoundary", &TESTS["DynamicJumpiOutsideBoundary"], true), true); }
#[test] fn DynamicJumpifInsidePushWithJumpDest() { assert_eq!(test_transaction("DynamicJumpifInsidePushWithJumpDest", &TESTS["DynamicJumpifInsidePushWithJumpDest"], true), true); }
#[test] fn DynamicJumpifInsidePushWithoutJumpDest() { assert_eq!(test_transaction("DynamicJumpifInsidePushWithoutJumpDest", &TESTS["DynamicJumpifInsidePushWithoutJumpDest"], true), true); }
#[test] fn JDfromStorageDynamicJump0_AfterJumpdest() { assert_eq!(test_transaction("JDfromStorageDynamicJump0_AfterJumpdest", &TESTS["JDfromStorageDynamicJump0_AfterJumpdest"], true), true); }
#[test] fn JDfromStorageDynamicJump0_AfterJumpdest3() { assert_eq!(test_transaction("JDfromStorageDynamicJump0_AfterJumpdest3", &TESTS["JDfromStorageDynamicJump0_AfterJumpdest3"], true), true); }
#[test] fn JDfromStorageDynamicJump0_foreverOutOfGas() { assert_eq!(test_transaction("JDfromStorageDynamicJump0_foreverOutOfGas", &TESTS["JDfromStorageDynamicJump0_foreverOutOfGas"], true), true); }
#[test] fn JDfromStorageDynamicJump0_jumpdest0() { assert_eq!(test_transaction("JDfromStorageDynamicJump0_jumpdest0", &TESTS["JDfromStorageDynamicJump0_jumpdest0"], true), true); }
#[test] fn JDfromStorageDynamicJump0_jumpdest2() { assert_eq!(test_transaction("JDfromStorageDynamicJump0_jumpdest2", &TESTS["JDfromStorageDynamicJump0_jumpdest2"], true), true); }
#[test] fn JDfromStorageDynamicJump0_withoutJumpdest() { assert_eq!(test_transaction("JDfromStorageDynamicJump0_withoutJumpdest", &TESTS["JDfromStorageDynamicJump0_withoutJumpdest"], true), true); }
#[test] fn JDfromStorageDynamicJump1() { assert_eq!(test_transaction("JDfromStorageDynamicJump1", &TESTS["JDfromStorageDynamicJump1"], true), true); }
#[test] fn JDfromStorageDynamicJumpInsidePushWithJumpDest() { assert_eq!(test_transaction("JDfromStorageDynamicJumpInsidePushWithJumpDest", &TESTS["JDfromStorageDynamicJumpInsidePushWithJumpDest"], true), true); }
#[test] fn JDfromStorageDynamicJumpInsidePushWithoutJumpDest() { assert_eq!(test_transaction("JDfromStorageDynamicJumpInsidePushWithoutJumpDest", &TESTS["JDfromStorageDynamicJumpInsidePushWithoutJumpDest"], true), true); }
#[test] fn JDfromStorageDynamicJumpi0() { assert_eq!(test_transaction("JDfromStorageDynamicJumpi0", &TESTS["JDfromStorageDynamicJumpi0"], true), true); }
#[test] fn JDfromStorageDynamicJumpi1() { assert_eq!(test_transaction("JDfromStorageDynamicJumpi1", &TESTS["JDfromStorageDynamicJumpi1"], true), true); }
#[test] fn JDfromStorageDynamicJumpi1_jumpdest() { assert_eq!(test_transaction("JDfromStorageDynamicJumpi1_jumpdest", &TESTS["JDfromStorageDynamicJumpi1_jumpdest"], true), true); }
#[test] fn JDfromStorageDynamicJumpiAfterStop() { assert_eq!(test_transaction("JDfromStorageDynamicJumpiAfterStop", &TESTS["JDfromStorageDynamicJumpiAfterStop"], true), true); }
#[test] fn JDfromStorageDynamicJumpiOutsideBoundary() { assert_eq!(test_transaction("JDfromStorageDynamicJumpiOutsideBoundary", &TESTS["JDfromStorageDynamicJumpiOutsideBoundary"], true), true); }
#[test] fn JDfromStorageDynamicJumpifInsidePushWithJumpDest() { assert_eq!(test_transaction("JDfromStorageDynamicJumpifInsidePushWithJumpDest", &TESTS["JDfromStorageDynamicJumpifInsidePushWithJumpDest"], true), true); }
#[test] fn JDfromStorageDynamicJumpifInsidePushWithoutJumpDest() { assert_eq!(test_transaction("JDfromStorageDynamicJumpifInsidePushWithoutJumpDest", &TESTS["JDfromStorageDynamicJumpifInsidePushWithoutJumpDest"], true), true); }
#[test] fn bad_indirect_jump1() { assert_eq!(test_transaction("bad_indirect_jump1", &TESTS["bad_indirect_jump1"], true), true); }
#[test] fn bad_indirect_jump2() { assert_eq!(test_transaction("bad_indirect_jump2", &TESTS["bad_indirect_jump2"], true), true); }
#[test] fn byte1() { assert_eq!(test_transaction("byte1", &TESTS["byte1"], true), true); }
#[test] fn calldatacopyMemExp() { assert_eq!(test_transaction("calldatacopyMemExp", &TESTS["calldatacopyMemExp"], true), true); }
#[test] fn codecopyMemExp() { assert_eq!(test_transaction("codecopyMemExp", &TESTS["codecopyMemExp"], true), true); }
#[test] fn deadCode_1() { assert_eq!(test_transaction("deadCode_1", &TESTS["deadCode_1"], true), true); }
#[test] fn dupAt51becameMload() { assert_eq!(test_transaction("dupAt51becameMload", &TESTS["dupAt51becameMload"], true), true); }
#[test] fn extcodecopyMemExp() { assert_eq!(test_transaction("extcodecopyMemExp", &TESTS["extcodecopyMemExp"], true), true); }
#[test] fn for_loop1() { assert_eq!(test_transaction("for_loop1", &TESTS["for_loop1"], true), true); }
#[test] fn for_loop2() { assert_eq!(test_transaction("for_loop2", &TESTS["for_loop2"], true), true); }
#[test] fn gas0() { assert_eq!(test_transaction("gas0", &TESTS["gas0"], true), true); }
#[test] fn gas1() { assert_eq!(test_transaction("gas1", &TESTS["gas1"], true), true); }
#[test] fn gasOverFlow() { assert_eq!(test_transaction("gasOverFlow", &TESTS["gasOverFlow"], true), true); }
#[test] fn indirect_jump1() { assert_eq!(test_transaction("indirect_jump1", &TESTS["indirect_jump1"], true), true); }
#[test] fn indirect_jump2() { assert_eq!(test_transaction("indirect_jump2", &TESTS["indirect_jump2"], true), true); }
#[test] fn indirect_jump3() { assert_eq!(test_transaction("indirect_jump3", &TESTS["indirect_jump3"], true), true); }
#[test] fn indirect_jump4() { assert_eq!(test_transaction("indirect_jump4", &TESTS["indirect_jump4"], true), true); }
#[test] fn jump0_AfterJumpdest() { assert_eq!(test_transaction("jump0_AfterJumpdest", &TESTS["jump0_AfterJumpdest"], true), true); }
#[test] fn jump0_AfterJumpdest3() { assert_eq!(test_transaction("jump0_AfterJumpdest3", &TESTS["jump0_AfterJumpdest3"], true), true); }
#[test] fn jump0_foreverOutOfGas() { assert_eq!(test_transaction("jump0_foreverOutOfGas", &TESTS["jump0_foreverOutOfGas"], true), true); }
#[test] fn jump0_jumpdest0() { assert_eq!(test_transaction("jump0_jumpdest0", &TESTS["jump0_jumpdest0"], true), true); }
#[test] fn jump0_jumpdest2() { assert_eq!(test_transaction("jump0_jumpdest2", &TESTS["jump0_jumpdest2"], true), true); }
#[test] fn jump0_outOfBoundary() { assert_eq!(test_transaction("jump0_outOfBoundary", &TESTS["jump0_outOfBoundary"], true), true); }
#[test] fn jump0_withoutJumpdest() { assert_eq!(test_transaction("jump0_withoutJumpdest", &TESTS["jump0_withoutJumpdest"], true), true); }
#[test] fn jump1() { assert_eq!(test_transaction("jump1", &TESTS["jump1"], true), true); }
#[test] fn jumpAfterStop() { assert_eq!(test_transaction("jumpAfterStop", &TESTS["jumpAfterStop"], true), true); }
#[test] fn jumpDynamicJumpSameDest() { assert_eq!(test_transaction("jumpDynamicJumpSameDest", &TESTS["jumpDynamicJumpSameDest"], true), true); }
#[test] fn jumpHigh() { assert_eq!(test_transaction("jumpHigh", &TESTS["jumpHigh"], true), true); }
#[test] fn jumpInsidePushWithJumpDest() { assert_eq!(test_transaction("jumpInsidePushWithJumpDest", &TESTS["jumpInsidePushWithJumpDest"], true), true); }
#[test] fn jumpInsidePushWithoutJumpDest() { assert_eq!(test_transaction("jumpInsidePushWithoutJumpDest", &TESTS["jumpInsidePushWithoutJumpDest"], true), true); }
#[test] fn jumpOntoJump() { assert_eq!(test_transaction("jumpOntoJump", &TESTS["jumpOntoJump"], true), true); }
#[test] fn jumpTo1InstructionafterJump() { assert_eq!(test_transaction("jumpTo1InstructionafterJump", &TESTS["jumpTo1InstructionafterJump"], true), true); }
#[test] fn jumpTo1InstructionafterJump_jumpdestFirstInstruction() { assert_eq!(test_transaction("jumpTo1InstructionafterJump_jumpdestFirstInstruction", &TESTS["jumpTo1InstructionafterJump_jumpdestFirstInstruction"], true), true); }
#[test] fn jumpTo1InstructionafterJump_noJumpDest() { assert_eq!(test_transaction("jumpTo1InstructionafterJump_noJumpDest", &TESTS["jumpTo1InstructionafterJump_noJumpDest"], true), true); }
#[test] fn jumpToUint64maxPlus1() { assert_eq!(test_transaction("jumpToUint64maxPlus1", &TESTS["jumpToUint64maxPlus1"], true), true); }
#[test] fn jumpToUintmaxPlus1() { assert_eq!(test_transaction("jumpToUintmaxPlus1", &TESTS["jumpToUintmaxPlus1"], true), true); }
#[test] fn jumpdestBigList() { assert_eq!(test_transaction("jumpdestBigList", &TESTS["jumpdestBigList"], true), true); }
#[test] fn jumpi0() { assert_eq!(test_transaction("jumpi0", &TESTS["jumpi0"], true), true); }
#[test] fn jumpi1() { assert_eq!(test_transaction("jumpi1", &TESTS["jumpi1"], true), true); }
#[test] fn jumpi1_jumpdest() { assert_eq!(test_transaction("jumpi1_jumpdest", &TESTS["jumpi1_jumpdest"], true), true); }
#[test] fn jumpiAfterStop() { assert_eq!(test_transaction("jumpiAfterStop", &TESTS["jumpiAfterStop"], true), true); }
#[test] fn jumpiOutsideBoundary() { assert_eq!(test_transaction("jumpiOutsideBoundary", &TESTS["jumpiOutsideBoundary"], true), true); }
#[test] fn jumpiToUint64maxPlus1() { assert_eq!(test_transaction("jumpiToUint64maxPlus1", &TESTS["jumpiToUint64maxPlus1"], true), true); }
#[test] fn jumpiToUintmaxPlus1() { assert_eq!(test_transaction("jumpiToUintmaxPlus1", &TESTS["jumpiToUintmaxPlus1"], true), true); }
#[test] fn jumpi_at_the_end() { assert_eq!(test_transaction("jumpi_at_the_end", &TESTS["jumpi_at_the_end"], true), true); }
#[test] fn jumpifInsidePushWithJumpDest() { assert_eq!(test_transaction("jumpifInsidePushWithJumpDest", &TESTS["jumpifInsidePushWithJumpDest"], true), true); }
#[test] fn jumpifInsidePushWithoutJumpDest() { assert_eq!(test_transaction("jumpifInsidePushWithoutJumpDest", &TESTS["jumpifInsidePushWithoutJumpDest"], true), true); }
#[test] fn kv1() { assert_eq!(test_transaction("kv1", &TESTS["kv1"], true), true); }
#[test] fn log1MemExp() { assert_eq!(test_transaction("log1MemExp", &TESTS["log1MemExp"], true), true); }
#[test] fn loop_stacklimit_1020() { assert_eq!(test_transaction("loop_stacklimit_1020", &TESTS["loop_stacklimit_1020"], true), true); }
#[test] fn loop_stacklimit_1021() { assert_eq!(test_transaction("loop_stacklimit_1021", &TESTS["loop_stacklimit_1021"], true), true); }
#[test] fn memory1() { assert_eq!(test_transaction("memory1", &TESTS["memory1"], true), true); }
#[test] fn mloadError0() { assert_eq!(test_transaction("mloadError0", &TESTS["mloadError0"], true), true); }
#[test] fn mloadError1() { assert_eq!(test_transaction("mloadError1", &TESTS["mloadError1"], true), true); }
#[test] fn mloadMemExp() { assert_eq!(test_transaction("mloadMemExp", &TESTS["mloadMemExp"], true), true); }
#[test] fn mloadOutOfGasError2() { assert_eq!(test_transaction("mloadOutOfGasError2", &TESTS["mloadOutOfGasError2"], true), true); }
#[test] fn msize0() { assert_eq!(test_transaction("msize0", &TESTS["msize0"], true), true); }
#[test] fn msize1() { assert_eq!(test_transaction("msize1", &TESTS["msize1"], true), true); }
#[test] fn msize2() { assert_eq!(test_transaction("msize2", &TESTS["msize2"], true), true); }
#[test] fn msize3() { assert_eq!(test_transaction("msize3", &TESTS["msize3"], true), true); }
#[test] fn mstore0() { assert_eq!(test_transaction("mstore0", &TESTS["mstore0"], true), true); }
#[test] fn mstore1() { assert_eq!(test_transaction("mstore1", &TESTS["mstore1"], true), true); }
#[test] fn mstore8MemExp() { assert_eq!(test_transaction("mstore8MemExp", &TESTS["mstore8MemExp"], true), true); }
#[test] fn mstore8WordToBigError() { assert_eq!(test_transaction("mstore8WordToBigError", &TESTS["mstore8WordToBigError"], true), true); }
#[test] fn mstore8_0() { assert_eq!(test_transaction("mstore8_0", &TESTS["mstore8_0"], true), true); }
#[test] fn mstore8_1() { assert_eq!(test_transaction("mstore8_1", &TESTS["mstore8_1"], true), true); }
#[test] fn mstoreMemExp() { assert_eq!(test_transaction("mstoreMemExp", &TESTS["mstoreMemExp"], true), true); }
#[test] fn mstoreWordToBigError() { assert_eq!(test_transaction("mstoreWordToBigError", &TESTS["mstoreWordToBigError"], true), true); }
#[test] fn mstore_mload0() { assert_eq!(test_transaction("mstore_mload0", &TESTS["mstore_mload0"], true), true); }
#[test] fn pc0() { assert_eq!(test_transaction("pc0", &TESTS["pc0"], true), true); }
#[test] fn pc1() { assert_eq!(test_transaction("pc1", &TESTS["pc1"], true), true); }
#[test] fn pop0() { assert_eq!(test_transaction("pop0", &TESTS["pop0"], true), true); }
#[test] fn pop1() { assert_eq!(test_transaction("pop1", &TESTS["pop1"], true), true); }
#[test] fn return1() { assert_eq!(test_transaction("return1", &TESTS["return1"], true), true); }
#[test] fn return2() { assert_eq!(test_transaction("return2", &TESTS["return2"], true), true); }
#[test] fn sha3MemExp() { assert_eq!(test_transaction("sha3MemExp", &TESTS["sha3MemExp"], true), true); }
#[test] fn sstore_load_0() { assert_eq!(test_transaction("sstore_load_0", &TESTS["sstore_load_0"], true), true); }
#[test] fn sstore_load_1() { assert_eq!(test_transaction("sstore_load_1", &TESTS["sstore_load_1"], true), true); }
#[test] fn sstore_load_2() { assert_eq!(test_transaction("sstore_load_2", &TESTS["sstore_load_2"], true), true); }
#[test] fn sstore_underflow() { assert_eq!(test_transaction("sstore_underflow", &TESTS["sstore_underflow"], true), true); }
#[test] fn stack_loop() { assert_eq!(test_transaction("stack_loop", &TESTS["stack_loop"], true), true); }
#[test] fn stackjump1() { assert_eq!(test_transaction("stackjump1", &TESTS["stackjump1"], true), true); }
#[test] fn swapAt52becameMstore() { assert_eq!(test_transaction("swapAt52becameMstore", &TESTS["swapAt52becameMstore"], true), true); }
#[test] fn when() { assert_eq!(test_transaction("when", &TESTS["when"], true), true); }