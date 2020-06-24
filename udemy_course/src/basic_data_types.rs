use std::mem; //import mem to get size. mem is for ops on memory
pub fn basic_data_types_fn() {
    //bind 123 to a variable 'a' of type u8 ie unsigned 8 bit integer
    //i8 will be signed 8 bits. msb is sign, remaining 7 for 1-127 
    //variable bindings are immutable. Cant change value later
    let a:u8 = 123;
    println!("a={}",a);

    //mutable value
    let mut b:i8 = 0;
    println!("b={}",b);
    b=42;
    println!("b={}",b);

    let mut c =123456789; //32 bit signed ie i32. This is letting rust choose the type
    println!("c={}, size={} bytes",c,mem::size_of_val(&c));
    c = -1;
    println!("c={} after modification to show it is signed int, size={} bytes",c,mem::size_of_val(&c));
 
    //i8 u8 i16 u16 i32 u32 i64 u64

    let z:isize=123; //isize, usize are integral data types ie size of architecture
    let size_of_z = mem::size_of_val(&z);
    println!("z = {} takes up {} bytes on {}-bit os",z, size_of_z, size_of_z*8);

    //char. size of char is 4 bytes
    let d:char = 'x';//even let d = 'x' will work
    println!("d={}, size={} bytes",d,mem::size_of_val(&d));

    //double-precision value, 8 bytes or 64 bits. data-type is f64. for 32bit use f32
    let e=2.5;
    println!("e={}, size={} bytes",e,mem::size_of_val(&e));

    //bool is single byte
    let g=false;
    println!("g={}, size={} bytes",g,mem::size_of_val(&g));
    let f = 4>0;//true
    println!("f={}, size={} bytes",f,mem::size_of_val(&f));
}
