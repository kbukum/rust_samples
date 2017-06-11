
pub fn test5(){
    /*
    ## non primitive types
    */
    let v = vec![1, 2, 3, 4, 5]; // v: Vec<i32>
    println!("Vector v = {:?}", v);

    let v = vec![0; 10]; // A vector of ten zeroes.
    println!("Vector v = {:?}", v);

    let v = vec![1, 2, 3, 4, 5];
    println!("Vector v = {:?}", v);
    println!("The third element of v is {}", v[2]);
    let i: usize = 4;
    println!("The fifth element of v is {}", v[i]);

    let v = vec![1, 2, 3];
    println!("Vector v = {:?}", v);
    println!("Try to get eigth element of v !");
    match v.get(7) {
        Some(x) => println!("Item 7 is {}", x),
        None => println!("Sorry, this vector is too short.")
    }

    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("A reference to {}", i);
    }

    for i in &mut v {
        println!("A mutable reference to {}", i);
    }
    for i in &v {
        println!("Call again A reference to {}", i);
    }

    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }
}