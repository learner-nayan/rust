use std::sync::mpsc::SyncSender;

#[derive(Debug)]
struct User{
    username:String,
    email:String,
    active:bool,
    sign_in_count:u64,
}

pub fn structure(){
    let mut user1 = User{
        username: String::from("nayan"),
        email: String::from("nayan@gmail.com"),
        active: true,
        sign_in_count: 1,
    };

    user1.username = String::from("ramesh");

    println!("{}",user1.username);

    let user2 = User{
        username: String::from("khanal"),
        email: String::from("khanal@gmail.com"),
        active: false,
        sign_in_count: 2,
    };

    let user3 = User{
        username: String::from("rajesh"),
        email: String::from("rajesh@gmail.com"),
        ..user2
    };

    let user4 = create_user(String::from("rakesh"),String::from("rajesh@gmail.com"),true,5);

    println!("{:#?}", user4)
}

fn create_user(username:String, email:String, active:bool, sign_in_count:u64) ->User{
    User{
        username,
        email,
        active,
        sign_in_count
    }
}