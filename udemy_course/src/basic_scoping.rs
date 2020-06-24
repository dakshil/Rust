fn scope_and_shadowing()
{
    let a = 123;
    println!("a={}",a);
    //this redeclares a in scope of this function
    let a = 1234;
    {
        //b only exists in this scope {}
        let b = 456;
        println!("Inside, b has value {}",b);
        //a also exists here
        println!("Inside, a has value {}",a);
        let a = 234;
        //the print now refers to this inner a
        println!("Inside, shadow a has value {}",a);
    }
    //b doesn't exist here
    //a refers to the function scope a
    println!("Outside, a has value {}",a);
}
pub fn basic_scoping_fn()
{
    scope_and_shadowing();

}