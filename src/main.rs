// fn main() {
//     greet("abel");
//     println!("{}",add(30, 40));
// }

// fn greet(name: &str) {
//     println!("greetings, {}", name);
// }

// fn add(a: i32, b: i32) -> i32{
//     a + b // this is an expression, it's value automatically becomes the return value
// }


fn main() {
    let number = 6;

    let label = if number % 2 == 0 {"even"} else {"odd"};

    println!("{}", label);

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 5 {
            break counter * 3;
        }
    };

    println!("Loop result: {}", result)
}
