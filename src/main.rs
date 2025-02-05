use std::io;

fn main() {
    const VALUE_CONST :u32 = 66;
    println!("Hello, world!");
    let mut x = 5;
    println!("X:{:?}", x);
    x = 3;
    println!("X:{}", x);
    let y = 3;
    println!("Y:{}", y);
    let y = 4;
    println!("Y:{}", y);
    println!("CONST:{}", VALUE_CONST);
    let stri = get_input();
    println!("{}", stri.trim());

    let inp_input: i64 = stri.trim().parse().unwrap();
    println!("Y:{}", inp_input);


    println!("{}", add(x, y));
    let c = 10.0f32;//casting
    println!("Y:{}", c);
    let e = 19_000_i64;//casting
    println!("Y:{}", e);
    let mut a = 19 as i64;//casting
    println!("Y:{}", a);
    a = c as i64; //explicit type convertion
    println!("Y:{}", a);


    let u = (i32::MAX as i64) + 1;
    println!("Y:{}", (u as i32));


    
}

fn get_input() -> String {
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("failed to read");
    return ret;
}

fn add(one: i32, two: i32) -> i32 {
    return one + two;

}
fn sub(one: i32, two: i32) -> i32 {
    one - two

}