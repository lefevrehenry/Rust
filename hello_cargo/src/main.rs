
const Y: u32 = 255;

fn main() {

    // Data Type
    // i/u (8,16,32,64,128,size)
    // f 32/64
    // bool
    // char

    // Literal
    // 34_234 / 1_000_000
    // 0b1111_0000
    // 0o77
    // 0xFF
    // b'F'

    // integer
    let x = 5;
    let y: i32 = 5;

    println!("Hello, world! {}, {}", x, y);
    
    // float
    let x = 5.0;
    let y: f64 = 5.45;

    println!("Hello, world! {}, {}", x, y);
    
    // bool
    let x = true;
    let y: bool = false;

    println!("Hello, world! {}, {}", x, y);
    
    // char
    let x = 'z';
    let y: char = 'W';
    let z = '😻';

    println!("Hello, world! {}, {}, {}", x, y, z);

    // tuple
    let x = (1,2,3);
    let y: (i32, char, bool) = (45, 'a', true);
    
    println!("Hello, world! {}, {}", x.1, y.2);

    let x = ();

    println!("Hello, world! {:?}", x);

    // array
    let x = [1,2,3,4,5];
    let y: [i32; 5] = [1,2,3,4,5];
    let z = [3; 5];

    println!("Hello, world! {:?}, {:?}, {:?}", x, y, z);

    // shadowing
    let x = 1;
    {
        let x = 2;
        // x equals 2
    }
    // x equels 1
    
    println!("Hello, world! {}", x);

    another_function();

    let x = add(7,3);

    println!("Hello, world! {}", x);

    // statement = do not return value
    let x = 42;

    // expression = something evaluate to a value
    // 6 is a expression
    // 3 + 4
    // calling a function
    // calling a macro
    // a new scope block
    let y = {
        let x = 3;
        x + 1           // <-- no semicolon here !!
    };
    
    println!("Hello, world! {}", y);

    // control workflow 

    let condition = true;
    if condition {
        println!("ok");
    } else {
        println!("!ok");
    }

    let x = if condition {4 } else { 5 };
    /*
    let x = loop {
        break 32;
    };

    while condition {
    }

    for element in array {
    }i
    */
    
    println!("Hello, world! {}", fibo(15));
    println!("Hello, world! {}", from_celsius(15.4));
    println!("Hello, world! {}", to_celsius(72.9));

    let mut x = 5;

    {
        let y = 6;

        x = y;
    }
    
    println!("Hello, world! {}", x);

    // String vs string literal
    let x = "hello";                    // string literal
    let s1 = String::from("hello");     // type String
    let s2 = s1.clone();                // deep copy

    println!("Hello, world! {}", x);
    println!("Hello, world! {}", s1);

    let mut s0 = String::from("Hello");     // String mutable
/*
    {
        let s1 = s0;                // move reference from s0
        let mut s2 = s1;            // move mutable reference from s1
    }
    {
        let s1 = &s0;               // borrow reference from s0
    }
    {
        let s1 = &mut s0;           // borrow mutable reference from s0 (s0 must be mutable)
        //let s2 = &mut s0;         // error (only one mutable reference at a time)
        change(s1);
    }
    {
        let s1 = &s0;               // 1st borrow reference
        let s2 = &s0;               // 2nd borrow reference
        //let s3 = &mut s0;         // error (cannot borrow mutable reference while we have an immutable one)
        
        println!(">>> {} {}", s1, s2);
    }
*/
    {
        let s1 = &s0;               // 1st borrow reference
        let s2 = &s0;               // 2nd borrow reference
        
        println!(">>> {} {}", s1, s2);
        
        let s3 = &mut s0;           // ok last used of s1, s2 happens before we borrow an immutable reference
        
        println!(">>> {}", s3);
    }

    println!("Hello, world! {}", s1);
}

fn change(s: &mut String) {
    s.push_str("_ok");
}

fn fibo(n: u32) -> u32
{
    if n <= 1 {
        return n;
    } else {
        return fibo(n-1) + fibo(n-2);
    }
}

fn from_celsius(x: f32) -> f32
{
    return 32. + (x * 1.8);
}

fn to_celsius(x: f32) -> f32
{
    return (x - 32.) / 1.8;
}

fn another_function()
{
    println!("Hello, world! {}, {}", 42, Y);
}

fn add(x: i32, y: i32) -> i32
{
    return x + y;
}
