pub fn run() {
    //variable declaration, immutable by default. Use "mut" for immutability.
        let name = "Monis";
        let age = 47;
        println!("My name is {}, I am {} years odl.", name, age);
    
    //Define constant
    const ID: i32 = 007;
    println!("ID: {}", ID);
    
    
    //assing multiple variables
    let (my_age, my_name) = ("Monis", 47);
    println!("age: {}, name: {}", my_age, my_name);
    
    }
    