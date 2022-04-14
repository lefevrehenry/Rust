
fn main() {
    // let mut i: i8 = 0;
    // i = 2;

    let my: i8 = 50;

    match my
    {
        x @ 0..100 => {
            println!("ok")
        }
        _ => {
            println!("unknown")
        }
    }

    for i in 10..20 {
        println!("Hello, world! {}", i);
    }

}
