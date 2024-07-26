pub fn ownership(){
    //heap memory
    let s1 = String::from("Hello");

    // let s2 = s1;
    // println!("{}",s1);
    // we cannot do this, Rust does not point to the same memory location  in heap
    // as it was the case in javascript.
    // value is moved to s2 , so s1 has nothing
    // THIS IS ALSO THE WAY TO UNDERSTAND OWNERSHIP AND BORROWING CONCEPT IN RUST

    // so to get clone we can use clone method
    let s2 = s1.clone();
    println!("{}",s1);


    let str1 = String::from("Namaste");
    let mut str2 = String::from("Pranam");
    let num = 60;

    let ref1 = &str2;
    let ref2 = &str2;

    let ref3 = &mut str2;
    // let ref4 = &mut str2;

    println!("{:?}",&str1[0..5]);

    println!("{}",num);
    println!("{}",&num);
}