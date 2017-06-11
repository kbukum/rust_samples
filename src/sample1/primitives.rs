

pub fn test3(){
    /*
    ## boolean types
    */
    let x = true;
    let y: bool = false;
    println!("bool x = {}, bool y = {}", x, y);

    /* 
    ## char type
    */
    let x = 'x';
    let symbol = '💕';
    println!("char x = {}", x);
    println!("char symbol = {}", symbol);

    /* 
    ## numeric types
    #### all numeric types
    - i8
    - i16
    - i32
    - i64
    - u8
    - u16
    - u32
    - u64
    - isize
    - usize
    - f32
    - f64
    */
    let x = 42; // `x` has type `i32`.
    println!("i32 x = {}", x);
    let y = 1.0; // `y` has type `f64`.
    println!("f64 y = {}", y);

    /* 
    ## Array Type
    */
    let a = [1, 2, 3]; // a: [i32; 3]
    println!("array a = {:?}", a);
    let mut m = [1, 2, 3]; // m: [i32; 3]
    println!("array m = {:?}", m);
    // set default value and create
    let a = [0; 20]; // a: [i32; 20]
    println!("array a = {:?}", a);
    // get len of array
    println!("array a.length = {:?}", a.len());
    // get particular element of array
    let names = ["Graydon", "Brian", "Niko"]; // names: [&str; 3]
    println!("The second name is: {}", names[1]);
    // Slices
    let a = [0, 1, 2, 3, 4];
    println!("array a = {:?}", a);
    let new_a = &a[..]; // A slice containing all of the elements in `a`.
    println!("array new_a = {:?}", new_a);
    let part_of_a = &a[1..4]; // A slice of `a`: only the elements `1`, `2`, and `3`.
    println!("part_of_a = {:?}", part_of_a);

    /*
    ## Tupples
    */
    let x = (1, "hello");
    println!("tupple x:(i32, &str) = {:?}", x);
    // type annotated tupple
    let x: (i32, &str) = (1, "hello");
    println!("tupple type annotated x:(i32, &str) = {:?}", x);
    // assign any tupple to another tupple
    let mut x = (1, 2); // x: (i32, i32)
    println!("tupple x: (i32, i32) = {:?}", x);
    let y = (2, 3); // y: (i32, i32)
    println!("tupple y: (i32, i32) = {:?}", y);
    x = y;
    println!("tupple after assign y to x : (i32, i32) = {:?}", x);
    // destructuring
    let a = (1, 2, 3);
    println!("tupple a = {:?}", a);
    let (x, y, z) = a;
    println!("tupple elements a.(x, y, z) = a.({:?}, {:?}, {:?})", x, y, z);
    // tupple indexing
    let a = (1, 2, 3);
    println!("tupple a = {:?} ", a);
    println!("tupple a first element a.0 = {:?}", a.0);
    println!("tupple a second element a.1 = {:?}", a.1);
    println!("tupple a thirth element a.2 = {:?}", a.2);
}