use std::fs::create_dir_all;

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
    let items:Vec<i32> = vec![9,10,8,48,58,1,2,70];
    println!("sum, evgenii! : {}", sum_of_3(1,2,2));
    print_power_of_5 (10);
    println!("are_friends : {}", are_friends(4,2));
    println!("magic_number : {}", magic_number(magic_number(magic_number(3))));
    println!("bar : {}", bar(5));
    println!("hello_developer : {}", )

    let r1=Rectangle{
        h:21,
        w:62
    };

    let r2=Rectangle{
        h:2100,
        w:620
    };

    let family : Vec<Person> = vec![
        Person{
            name: "Papa".to_string(),
            age: 56,
            money:1000,
            height:182
        },
        Person {
            name: "eugenii".to_string(),
            age:10,
            money:1000,
            height:158
        },
        Person{
            name: "mama".to_string(),
            age: 46,
            money:1000,
            height:162
        },
        Person{
            name: "max".to_string(),
            age: 8,
            money:1000,
            height:144
        },
    ];

    println!("Rectangle area : {} {}", r1.area(),r2.area());
    println!("family height :{}",  total_height(family));
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

fn pari_dispari(){
    for i in 1..201 {
        if is_divided_by_2(i) {
            println!("pari");
        } else {
            println!("dispari")
        }
    }
}

fn is_divided_by_2(i:i32) -> bool{
    i % 2 == 0
}

fn is_divided_by_10(i:i32) -> bool{
    i % 10 == 0
}

fn all_5_friends(){
    for i in 1..11 {
        if is_divided_by_10(i) {
            println!("{}:I am your best friend",i);
        }else if is_divided_by_5(i) {
            println!("{}:I am your friend", i)
        }else  {
            println!("{}:I am not your friend", i)
        }
    }
}

fn all_6_friends(){
    for i in 1..101 {
        if is_divided_by_36(i) {
            println!("{}:I am your best friend",i);
        }else if is_divided_by_6(i) {
            println!("{}:I am your friend", i)
        }else  {
            println!("{}:I am not your friend", i)
        }
    }
}

fn is_divided_by_6(i:i32) -> bool{
    i % 6 == 0
}


fn is_divided_by_36(i:i32) -> bool{
    i % 36 == 0
}

fn max_from_vec(ar:Vec<i32>) -> i32 {
    let mut max: i32 = ar[0];
    for i in ar {
        if max <= i {
            max = i
        }
    }
    return max
}

fn power_of_2(i:i32) -> i32 {
    return match i {
        0 => 1,
        1 => 2,
        _ => 2 *  power_of_2(i-1)
    }
}

fn print_power_of_2(n:i32) {
    for i in 0..n {
        println!("{}",  power_of_2(i));
    }
}

fn min_from_vec(ar:Vec<i32>) -> i32 {
    let mut min: i32 = ar[0];
    for i in ar {
        if min >= i {
            min = i
        }
    }
    return min
}

fn power_of_3(i:i32) -> i32 {
    return match i {
        0 => 1,
        1 => 3,
        _ => 3 *  crate::power_of_3(i-1)
    }
}

fn print_power_of_3(n:i32) {
    for i in 0..n {
        println!("{}",  power_of_3(i));
    }
}

fn sum_of_3 (x:i32,y:i32,z:i32) -> i32 {
    return x + y + z
}

fn power_of_5(i:i32) -> i32 {
    return match i {
        0 => 1,
        1 => 5,
        _ => 5 *  power_of_5(i-1)
    }
}

fn print_power_of_5(n:i32) {
    for i in 0..n {
        println!("{}",  power_of_5(i));
    }
}

fn are_friends (a:i32,b:i32) -> bool {
    a % b == 0
}

fn magic_number(n:i32) -> i32 {
    if n % 2 == 0  { n * 2 } else { n + 1 }
}

struct Rectangle {
    h:i32,
    w:i32
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.w * self.h
    }
}
struct Person {
    name: String,
    age: i32,
    money:i32,
    height:i32,
}

impl Person {
    fn hello(&self){
        println!("Hello {}", self.name)
    }
}

fn total_age(family:Vec<Person>) -> i32 {
    let mut result:i32 = 0;
    for person in family{
        result=result + person.age;
    }
    result
}

fn total_money(family:Vec<Person>) -> i32 {
    let mut result:i32 = 0;
    for person in family{
        result=result + person.money;
    }
    result
}

fn total_height(family:Vec<Person>) -> i32 {
    let mut result:i32 = 0;
    for person in family{
        result=result + person.height;
    }
    result
}

fn foo () -> i32 {
    10
}

fn bar (x:i32) -> i32 {
    x * foo()
}

fn print_hello () {
    println!("{hello}", )
}

fn print_goodbye () {
    println!("{goodbye}",)
}

fn hello_developer () {
    print_hello; println! ("{eugenii}",) ; {
        print_goodbye()
    }
}











































