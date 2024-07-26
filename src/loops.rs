
pub fn loops(){
    println!("Hello, world!");
    for number in 1..=10 {
        println!("Number: {}",number);
    }

    let mut num = 0;
    let arr = ["hello","hi"];

    loop {
        num = num+1;
        println!("{}",num);

        if num == 5{
            break;
        }
    }

    for element in arr {
        println!("{}",element)
    }

    //data types
    let i = 10;
    let f = 4.5;
    let c = 'n';
    let s = "nayan";
    let b = true;


    println!("{}, {}, {}, {}, {}",i,f,c,s,b);
}