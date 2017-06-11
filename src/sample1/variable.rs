pub fn test1(){
    // type inference
    // let x: i32; you need to initialize value
   let x = 5;
   println!("x = {:?}", x);

   // type inference
   let (y, z) = (5, 6);

   println!(" y = {}, z = {}", y, z);

   // static type
   let a: u32 = 10;
   println!("a = {:?}", a);

  // mutability  
   let mut b = 5;
   println!("b = {:?}", b);
   b = 8;
   println!("b = {:?}", b);

   // Scope and shadowing
   let c = 5;
   {
       let d = 8;
       println!("c = {}, d = {}", c, d);
   }
   // println!("c = {}, d = {}", c, d); throw error because of scope of d which is alive ended. 

   let e = 6;
   {
       println!("e = {}", e);
       let e = 10;
       println!("e = {}", e);
   }

   println!("e = {}", e);
   let e = 42;
   println!("e = {}", e);
   // change mutable to immutable
   let mut f: i32 = 1;
    f = 7;
    let f = f; // `f` is now immutable and is bound to `7`.
    println!("f = {}", f);
    // f = 5 throw error . 'f' is immutable
}