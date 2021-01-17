fn main() {
    test_tuple();
    test_args(3, 5);
    test_return();
    test_branching(3, 4);
    test_branching(5, 1);
    test_branching(6, 6);
    loop_ten();
    loop_break_with_return();
    iterate_elements_in_array();
    generate_string_from_literals();
}

fn test_tuple() {
    let tup: (i32, &str, i32) = (25, "ffs, fml", 32);
    let (a, b, c) = tup;
    
    println!("{} + 7 = {}", a, c);
    println!("{}", b);
}

fn test_args(x: i32, y: i32) {
    let a = x + y;
    println!("{} + {} = {}", x, y, a);
}

fn test_return() {
    let a = return_two() + return_one();
    println!("2 + 1 = {}", a);
}

fn return_two() -> i32 {
    2
}

fn return_one() -> i32 {
    return 1;
}

fn test_branching(x: i32, y: i32) {
    if x < y {
        println!("{} < {}", x, y);
    } else if x > y {
        println!("{} > {}", x, y);
    } else {
        println!("{} = {}", x, y);
    }
}

fn loop_ten() -> i32 {
    let mut a = 0;
    loop {
        a += 1;
        println!("Loop - {}", a);
        if a == 10 {
            println!("Break");
            break;
        }
    }
    return a;
}

fn loop_break_with_return() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result = {}", result);
}

fn iterate_elements_in_array() {
    let a = [1, 2, 3, 4, 5];
    
    for e in a.iter() {
        println!("element value: {}", e);
    }
}

fn generate_string_from_literals() {
    let s1 = String::from("Rust is awesome!");
    let s2 = String::from("I love Rust");
    let s3 = String::from("I think I am a Rustacean!");
    
    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
    
    println!("I do not need to free the assigned memories, since Rust will automatically free the assigned memories!!")
}
