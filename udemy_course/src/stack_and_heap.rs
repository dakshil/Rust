use std::mem;
struct Point
{
    x: f64,
    y: f64
}
fn origin() -> Point
{
    //returns Point with val 0.0,0.0
    Point{x: 0.0, y:0.0}
}
pub fn stack_and_heap_fn()
{
    //allocates in function stack
    let p1 = origin();
    //allocats in heap. returns pointer to where the point is stored
    let p2 = Box::new(origin());
    //16 bytes since 2 variables
    println!("p1 takes up {} bytes on stack", mem::size_of_val(&p1));
    //8 bytes since only pointer size is returned
    println!("p2 takes up {} bytes on stack", mem::size_of_val(&p2));

    //assign value at p2 to p3
    //ie unbox the value back onto the stack
    let p3 = *p2;
    println!("p3 takes up {} bytes on stack", mem::size_of_val(&p3));

}