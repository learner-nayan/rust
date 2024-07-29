struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn holds_on(&self, rectangle:&Rectangle) -> bool{
        self.width > rectangle.width && self.height > rectangle.height
    }
}


impl Rectangle{
    fn square(size:u32) -> Rectangle{
        Rectangle{
            width: size,
            height: size
        }
    }
}

pub fn structure_methods(){
    let rect = Rectangle{
        width: 5,
        height: 10
    };

    let rect2 = Rectangle{
        width: 7,
        height: 13,
    };

    let rect3 = Rectangle::square(5);

    println!("{}",rect.area());

    println!("{}",rect2.holds_on(&rect));

}
