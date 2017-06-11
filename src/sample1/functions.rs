
pub fn test() {
    println!("## Functions Tests ");
    empty();
    print_one_number_parameter(5);
    print_two_params(7, 8);
    let mut x: i32 = 5;
    println!("before x = {}", x);
    x = increment(x);
    println!("after increment x = {}", x);
    println!("after early return x = {}", early_return(x));
    
    println!("before fn_pointers x = {}", x);
    x = fn_pointers(x);
    println!("after fn_pointers x = {}", x);
}

fn empty() {
    println!("this is empty fn");
}

fn print_one_number_parameter(x: i32) {
    println!("Given parameter to print_one_number_parameter is {}", x);
}

fn print_two_params(x: i32, y: i32) {
    println!("Given parameters to print_two_params is {} and {}", x, y);
}

fn increment(x: i32) -> i32 {
    x + 1
}

fn early_return(x: i32) -> i32 {
    return x;
    increment(x)
}


fn fn_pointers(x: i32) -> i32 {
    fn plus_one(i: i32) -> i32 {
        i + 1
    }
    let f: fn(i32) -> i32 = plus_one;
    f(x)
}