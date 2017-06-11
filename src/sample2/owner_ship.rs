pub fn test1() {
    move_when_assign_non_primitive_variables();
    copy_when_assign_primitive_variables();
}

/*
    error: use of moved value: `a`
    println!("{}", a);
*/
fn move_when_assign_non_primitive_variables(){
    let v = vec![1, 2, 3];
     // v comes into scope 
    // a new vector is created on the stack
    // it allocates space on the heap for its elements

    println!("Defined variable comes into scope v = {:?} ", v);
    // v moved to the v2
    let v2 = v;
    // println!("Defined variable comes into scope v = {:?} ", v); // get error ! 
    println!("Defined v variable to move another pointer v2 = {:?} ", v2);
    // v goes out of scope at the end
}

fn copy_when_assign_primitive_variables(){
    let v = 3;
     // v comes into scope 
    // a new vector is created on the stack
    // it allocates space on the heap for its elements
    println!("Defined variable comes into scope v = {:?} ", v);
    // v copied to the v2
    let v2 = v;
    println!("Defined variable comes into scope v = {:?} ", v);
    println!("Defined v variable to copied another variable v2 = {:?} ", v2);
    // v goes out of scope at the end
}
