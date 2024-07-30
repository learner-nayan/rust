enum IpAddrType{
    v4(u8,u8,u8,u8),
    v6(String)
}

enum Message{
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

struct QuitMessage; //unit struct
struct MoveMessage{
    x:i32,
    y:i32,
}

// optional values . enums
// enum Option<T>{
//     Some(T),
//     None,
// }

enum UsState{
    Albana,
    Alaska,
    Arizona,
    Arkansas,
    California,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub fn enums(){
    let localhost = IpAddrType::v4(127,0,0,1);

    let x = Some(5);
    let y = Some("Hello");
    let z: Option<u32> = None;

    let num1 = 5;
    let num2 = Some(10);
    let num3 = None;

    //let result = num1 + num2;   //Cannot add `Option<i32>` to `i32`
    let result1 = num1 + num2.unwrap_or(0);
    let result2 = num1 + num3.unwrap_or(0);
    println!("{}",result1);
    println!("{}",result2);
}