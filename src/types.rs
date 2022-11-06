pub fn run() {
    //by default i32.
    let x = 1;
    //by default f64.
    let y = 1.1;

    //Add explicit type
    let z: i64 = 239802;

    //find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean
    let is_active = true;

    //Get boolean from an expression.
    let is_greater = 10 > 2;

     //char 
     let a1 = 'a';

    println!("{:?}", (x,y,z, is_active, is_greater,a1));


   
}