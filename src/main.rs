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
}

fn get_input() -> String {
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("failed to read");
    return ret;
}