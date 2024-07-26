use std::result;

pub fn basics(){
    //--------------------------CONSTANTS--------------------------
    const NAME:&str = "nayan";
    const NUMBER:u32 = 5;
    const PI:f32 = 3.14;

    //--------------------------VARIABLES--------------------------
    let mut  val = 100;
    let num = 7;
    println!("num: {}",num);
    println!("val: {}",val);

    val = 105;
    let num = 9;   //way to change value of immutable variable , here 'num'.
    println!("num: {}",num);
    println!("val: {}",val);

    //--------------------------DATA TYPES--------------------------
    //scalar and compound data types

    //scalar data types
    let a = 56_444;
    let b = 0xff; //hex
    let c = 0o77; //octal
    let d = 0b1111_0000;
    let e = b'A';
    let f = 5.6;
    let g = false;
    let h = 'a';

    //compound data types
    let tup = ("Nayan", 100);
    let (name, percentage) = tup;
    let user = tup.1;

    //--------------------------ARRAY--------------------------
    let error_codes = [200, 404, 500];
    println!("{}", error_codes[0]);

    let byte = [0; 8];
    println!("{:?}", byte);

    let result = add(3,5);
    println!("{}", result);

    for err in error_codes.iter() {
        println!("{}", err);
    }

    for a in 1..4 {
        println!("{}", a);
    }
}

//--------------------------FUNCTION--------------------------
fn add(a:i32, b:i32) ->i32{
    a+b   // same as doing return a+b;
}