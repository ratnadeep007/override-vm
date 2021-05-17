use crate::vm::VM;

pub mod vm;
pub mod instruction;
pub mod repl;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}

// fn get_test_vm() -> VM {
//     let mut test_vm = VM::new();
//     test_vm.registers[0] = 5;
//     test_vm.registers[1] = 10;
//     test_vm
// }
