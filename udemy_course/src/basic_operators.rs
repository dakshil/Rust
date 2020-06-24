pub fn operators_fn()
{
    let mut a = 2+3*4;//+,-* follows mult before addition prec
    println!("a={}",a);
    a+=1;//cannot use ++, --
    println!("a={}",a);
    println!("Remainder of {} / {} = {}",a,3,(a%3));

    let a_cubed = i32::pow(a,3);//pow is part of i32
    println!("a_cubed={}",a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b,3);//i stands for integral power
    let b_to_pi = f64::powf(b,std::f64::consts::PI);//floating power. Here PI is got from the std consts
    println!("b_cubed={}, b_to_pi={}",b_cubed, b_to_pi);

    //bitwise only for int
    let c = 1 | 2; //| OR, & AND, ^ XOR, ! NOR
    println!("c={}",c);
    
    let two_pow_ten = 1 << 10; //shift op
    println!("2 pow 10 = {}",two_pow_ten);
    println!("two_pow_ten={}",two_pow_ten);

    //logical
    let pi_less_4 = std::f64::consts::PI < 4.0; //true
    println!("pi < 4 = {}",pi_less_4);

    // > <= >= ==
    let x=5;
    let x_is_5 = x==5;
    println!("x={}, x==5 = {}",x,x_is_5);

}