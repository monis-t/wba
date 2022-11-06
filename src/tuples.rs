/*
Tuples are used for grouping values of different types.
A tuple can accomodate 12 elements.
*/


pub fn run() {

    let person: (&str, &str, i8) = ("Monis","India",42);
    println!("{} is from {}. He is {}.",person.0,person.1,person.2);
}