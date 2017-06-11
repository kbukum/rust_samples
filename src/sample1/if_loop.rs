

pub fn test(){
    println!("## Loops Tests ");
  // ## if and loop statements
  // ## if statement
  equalsFiveOrNot(5);
  equalsFiveOrNot(6);
  equalsFiveOrNot(7);
  
  let x = 5;
  let result = if x == 5 {
      "x equals 5"
  } else {
      "x is not equals 5"
  };
  println!("check x result = {:?}", result);

  let mut x = 0;
  println!("Loop will exist if x equals 5");
  /*
  #### loop
  */
  loop {
      if x == 5 {
          break;
      }
      x = x + 1;
      println!("Loop counter = {:?}", x);
  }

    /*
    #### while
    */
    println!("While will exist if x equals 0");
    while x > 0 {
        println!("While counter = {:?}", x);
        x = x - 1;
    }

    /*
    #### for
    */

    for i in 0..10 {
        println!("for counter i {:?}", i); // x: i32
    }

    for (index, value) in (5..10).enumerate() {
        println!("index = {} and value = {}", index, value);
    }

    let lines = "hello\nworld".lines();

    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }


    /* 
    #### labels
    */
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; } // Continues the loop over `x`.
            if y % 2 == 0 { continue 'inner; } // Continues the loop over `y`.
            println!("x: {}, y: {}", x, y);
        }
    }
    
}

fn equalsFiveOrNot(x: i32){
    if x == 5 {
      println!("{:?} is equals 5", x);
    } else if x == 6 {
        println!("{:?} is equals 6", x);
    } else {
        println!("{:?} is not equals 5 or 6", x);
    }
}