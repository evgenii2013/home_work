fn main() {
    let mut x:i32;
    let mut y:i32;
    let true_var: bool;/* Type */;
    x=43;
    y=9;
    x=21-78;
    y=multi(x,y);
    println!("Hello, evgenii! : {} {}", bigger_or_equal(5,5), less(3,4));
}
fn add(a:i32,b:i32) -> i32 {
    a+b
}
fn multi(a:i32,b:i32) -> i32 {
    a*b
}
fn less(a:i32,b:i32) -> bool {
    if a < b { true } else { false }
}
fn bigger_or_equal(a:i32, b:i32) -> bool {
    if a >= b { true } else { false }
}