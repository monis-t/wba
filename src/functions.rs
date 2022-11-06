pub fn run() {
    greeting("hello", " jane");

    //bind function values to variables
    let get_sum = add(3,4);
    println!("Sum: {}", get_sum);

    // closures
    let c: i32 = 20;
    let add_nums = |a:i32, b:i32| a + b + c;
    println!("C sum: {}", add_nums(3,2));
}

fn greeting(greet: &str, name : &str) {
    println!("{}{}, nice to meet you", greet, name);

}

fn add(a: i32, b:i32) -> i32 {
    a + b 
}