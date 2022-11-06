/** 
 * Vectors are resizeable arrays. 
*/

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    println!("Numbers in array are {:?}", numbers);

    //reassigning value
    numbers[2] = 20;
    println!("Numbers in array are {:?}", numbers);

    //printing single value
    println!("2nd element of array is {}", numbers[2]);

    //add onto vector
    numbers.push(56);
    println!("Numbers in array are {:?}", numbers);

    numbers.pop();
    println!("Numbers in array are {:?}", numbers);

    //the operations that can be performed on a fixed array can also be perfomed here.

    //loop through vector values
    for x in numbers.iter() {
        println!("number: {}", x);
    }

    //mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);

}