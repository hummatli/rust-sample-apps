use std::mem;

fn main() {
    //immutable variable
    let a:u8 = 123;
    println!("a {}", a);
    //a = 2; // Will give error cause it is immutable

    //mutable variable
    let mut b:i8 = 1;
    println!("b {}", b);
    
    b = 2;
    println!("b {}", b);


    //implicit type decleration
    let mut c = 112343454;
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    //char
    let d1:char = 'x';
    let d = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));
    
    //floating point numbers
    let e1:f64 = 2.3;
    let e = 2.3;
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    //boolean
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));

    let g1 = 1 < 9;
    println!("g1 = {}, size = {} bytes", g1, mem::size_of_val(&g1));

}