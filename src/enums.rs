use crate::enums::UsState::Alaska;

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

#[derive(Debug)]
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
    Quarter(UsState),
}

pub fn enums(){
    let localhost = IpAddrType::v4(127,0,0,1);

    let x = Some(5);
    let y = Some("Hello");
    let z: Option<i32> = None;

    let num1 = 5;
    let num2 = Some(10);
    let num3 = None;

    //let result = num1 + num2;   //Cannot add `Option<i32>` to `i32`
    let result1 = num1 + num2.unwrap_or(0);
    let result2 = num1 + num3.unwrap_or(0);
    println!("{}",result1);
    println!("{}",result2);

    let amount = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}",amount);

    let a:Option<i32>  = Some(5);
    let b:Option<i32> = None;

    let res1 = add_one(a);
    println!("{:?}",res1);
    let res2 = add_one(b);
    println!("{:?}",res2);
}

fn value_in_cents(coin: Coin) -> u8{
    match coin {
        Coin::Penny => {
            println!("Penny");
            1
        },
        Coin::Nickel => {
            println!("Nickel");
            5
        },
        Coin::Dime => {
            println!("Dime");
            10
        },
        Coin::Quarter(state) => {
            println!("Quarter: {:?}",state);
            25
        }
    }
}

fn add_one(g: Option<i32>) -> Option<i32>{
    match g {
        // None => None,
        Some(i) => Some(i+1),
        _ => None,
    }
}