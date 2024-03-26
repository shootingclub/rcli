fn main() {
    println!("Hello, world!");
    let a = 1;
    let b = 2;
    let rslt: Option<i32> = add(a, b);
    println!("{},{},{:?}", a, b, rslt.unwrap());
}

fn add(x: i32, y: i32) -> Option<i32> {
    Some(x + y)
}
