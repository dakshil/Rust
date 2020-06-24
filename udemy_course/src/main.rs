mod basic_data_types;
mod basic_operators;
mod basic_scoping;
mod use_of_constants;
mod stack_and_heap;
fn main() {
    println!("\nBasic Data Types:");
    basic_data_types::basic_data_types_fn();
    println!("\nBasic Operators:");
    basic_operators::operators_fn();
    println!("\nBasic Scoping:");
    basic_scoping::basic_scoping_fn();
    println!("\nUse of Constants");
    use_of_constants::constants_fn();
    println!("\nStack and Heap");
    stack_and_heap::stack_and_heap_fn();
}
