//Declare const. Have to give the type
//This has no fixed address
const GLOBAL_VAR1:u8 = 42;

//global with address
static GLOBAL_VAR2:u8 = 40;

//unsafe global
static mut GLOBAL_VAR3:u8 = 38;

pub fn constants_fn()
{
    //no address since compiler will just replace the variable here with actual value
    //cant declare const as mut since it is replaced and is not an address. 
    println!("GLOBAL_VAR1 {}", GLOBAL_VAR1);
    //static can be declared as mut but that will not compile since its unsafe
    //can have multiple threads accessing and changing value
    //will need to use an unsafe block
    println!("GLOBAL_VAR2 {}", GLOBAL_VAR2);
    unsafe
    {
        GLOBAL_VAR3=12;
        println!("GLOBAL_VAR3 {}", GLOBAL_VAR3);
    }
}