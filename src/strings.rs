pub fn run() {
    let mut cohort = String::from("WBA");
    println!("{}", cohort);

    let duration = String::from("Eight Weeks");
    
    //push as string
    cohort.push_str(". Come buidl with us");

    println!("Length of word: {}", cohort.len());
    println!("{}", cohort);

    //is_empty
    println!("Is Empty: {}", cohort.is_empty());

    //contains
    println!("Contains 'buidl' :{}", cohort.contains("buidl"));

    //replace 
    println!("Replace : {}", cohort.replace("buidl","build"));

    //loop through string by
    for word in cohort.split_whitespace() {
        println!("{}", word);
    }

    //create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);
    
    //assertions
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

}
