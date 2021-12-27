use std::io;

fn main() {
    println!("Welcome to Fibonnaci Generator\nEnter your nth number (integer): ");
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Error");

    let n: u32 = n.trim().parse().expect("Parse Error");

    println!("Answer: {}",fibo(n));
}

fn fibo(x: u32)->u32{
    let mut no1: u32 = 0;
    let mut no2: u32 = 1;
    let mut no3: u32 = 1;
    let mut count: u8 = x.try_into().unwrap();

    if x == 1{
        return 0;
    }
    else if x==2{
        return 1;
    }
    for i in (1..count-1){
        no1 = no2;
        no2 = no3;
        no3 = no1+no2;

        println!("Debug no1 {}", no1);
        println!("Debug no2 {}", no2);
        println!("Debug no3 {}", no3);
    }

    no3
}
