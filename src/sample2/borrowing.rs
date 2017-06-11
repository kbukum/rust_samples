pub fn test() {
    println!("## Borrowing Tests ");
    // borrowing with references
    // borrowed references is immutable
    let v1 = vec![1,2,3];
    let v2 = vec![4,5];
    let v3 = borrowing_immutable_references(&v1, &v2);
    println!(" v1 + v2 = v3 -> {:?} + {:?} = {:?}", v1, v2, v3);

    // &mut borrowed references
    let mut x = 5;
    { //its importing to take back borrowed reference.
        println!("x = {:?}", x);
        let y = &mut x;
        *y += 1;
        println!("(x + 1)  = y = {:?}", y);
    }
    println!("x = {:?}", x);

    // &mut borrowed reference will got error !
  
    let mut x = 5;
    let y = &mut x;
    *y += 1;
   /*
    println!("{}", x);
   |                    ^ immutable borrow occurs here
    */ 

    let mut x = 6;
    println!("before borrowed x as muttable variable x = {:?}", x);
    borrowing_mutable_references(&mut x);
    println!("after back x got from borrowed place x = {:?}", x);
}

fn borrowing_immutable_references(v1: &Vec<i32>, v2: &Vec<i32>) -> Vec<i32> {
     let max_v = if v1.len() > v2.len() { v1 } else { v2 };
     let min_v = if v1.len() > v2.len() { v2 } else { v1 };
     
     let mut v3 = vec![];
     
     for i in 0..max_v.len() {
        v3.push(max_v[i]);
     }
     for i in 0..min_v.len() {
        v3[i] = v3[i] + min_v[i];
     }
     // v1.push(6) you cannot do that. Because borrowed references is immutable.
     v3
}

fn borrowing_mutable_references(x:&mut i32) {
    *x += 1;
}
