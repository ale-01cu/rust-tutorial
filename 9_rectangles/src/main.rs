#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}


fn main() {
    println!("Hello, world!");

    let rect = Rectangle{
        width: 30,
        height: 50
    };

    let rect_area = area(&rect);

    println!("The area of the rectangle {:?} is {} square pixels.", rect, rect_area);

}


fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}


