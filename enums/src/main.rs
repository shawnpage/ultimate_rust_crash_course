use std::fs::File;


fn main() {
    #[derive(Debug)]
    enum Color {
        Red,
        Green,
        Blue,
    }
    let red_color = Color::Red;
    let green_color = Color::Green;
    let blue_color = Color::Blue;
    println!("color is {:?}", blue_color);
    println!("color is {:?}", green_color);
    println!("color is {:?}", red_color);


    // let res = File::open("hello.txt");
    let res = File::open("src/main.rs");
    if res.is_ok() {
        let f = res.unwrap();
        println!("file is {:?}", f);
    } else {
        println!("file is not ok");
    }

}
