use crate::vm::VM;

pub mod vm;
pub mod instruction;

fn main() {
    let mut test_vm = get_test_vm();
    test_vm.program = vec![2, 1, 0, 2];
    test_vm.run();
    println!("{}", test_vm.registers[0]);
}

fn get_test_vm() -> VM {
    let mut test_vm = VM::new();
    test_vm.registers[0] = 5;
    test_vm.registers[1] = 10;
    test_vm
}
