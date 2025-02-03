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
    println!("{}", get_input());
    println!("{}", add(x, y));
    let c = 10.0f32;//casting
    println!("Y:{}", c);
    let e = 19_000_i64;//casting
    println!("Y:{}", e);
}

fn get_input() -> String {
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("failed to read");
    return ret;
}

fn add(one: i32, two: i32) -> i32 {
    let result = one + two;
    return result
}
