fn main() {
    let mut x:i32;
    let mut y:i32;
    let true_var: bool;
    let ages:Vec<i32> = vec![9,10,8,48,58,1,2,70];
    x=43;
    y=9;
    x=21-78;
    y=multi(x,y);
    println!("Hello, evgenii! : {}", sum(ages));
    fizz_buzz();
}
fn sum(items:Vec<i32>) -> i32 {
    let mut result = 0;
    for item in items {
        result=result + item;
        println!("{}", result);
    }
    result
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
fn f(i:i32) -> i32 {
    return match i {
        0 => 0,
        1 => 1,
        _ => f(i-1) + f(i-2)
    }
}

fn print_f(n:i32) {
    for i in 0..n {
        println!("{}", f(i));
    }
}
fn fizz_buzz(){
    for i in 1..201 {
        if is_divided_by_15(i) {
            println!("fizz_buzz");
        } else if is_divided_by_5(i) {
            println!("buzz")
        } else if is_divided_by_3(i) {
            println!("fizz")
        } else {
            println!("{}", i);
        }
    }
}

fn is_divided_by_3(i:i32) -> bool{
    i % 3 == 0
}

fn is_divided_by_5(i:i32) -> bool{
    i % 5 == 0
}

fn is_divided_by_15(i:i32) -> bool{
    i % 15 == 0
}