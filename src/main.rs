fn main() {
    let mut x:i32;
    let mut y:i32;
    x=43;
    y=9;
    x=21-78;
    y=multi(x,y);
    println!("Hello, evgenii! : {} {}", add(x,y), multi(10,10));
}
fn add(a:i32,b:i32) -> i32 {
    a+b
}
fn multi(a:i32,b:i32) ->i32 {
    a*b
}