mod basic_data_types;
mod basic_operators;
mod basic_scoping;
fn main() {
    println!("\nBasic Data Types:");
    basic_data_types::basic_data_types_fn();
    println!("\nBasic Operators:");
    basic_operators::operators_fn();
    println!("\nBasic Scoping:");
    basic_scoping::basic_scoping_fn()
}
