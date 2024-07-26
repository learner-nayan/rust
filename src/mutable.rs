pub fn mutable(){
    let str = String::from("hello");
    let len = calculate_length(&str);
    println!("{}, {}",str,len);
}

fn calculate_length(s:&String) -> usize{
    s.len()
}