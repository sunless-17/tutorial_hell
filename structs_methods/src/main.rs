#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[derive(Debug)]
struct Square {
    width: u32,
}

impl Square {
    fn square(self) {
        println!("{}", self.width * 2);
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let sqwea = Square { width: 13 };

    println!(
        "The area of the rectangle is {:?} square pixels",
        rect1.area(),
    );
    sqwea.square();
}
