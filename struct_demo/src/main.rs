
struct Rectangle{
    width: u32,
    height: u32,
}


impl Rectangle{
    fn area(&self) -> u32{
        self.width + self.height
    }

    // associated functions
    fn area_associated(rect: &Rectangle) -> u32 {
        rect.width + rect.height
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 20,
        height: 20,
    };
    println!("rectangle area: {}", rectangle.area());
    println!("rectangle area: {}", Rectangle::area_associated(&rectangle));
}
