
fn main() {
    greet("abel");
    println!("{}",add(30, 40));
}

fn greet(name: &str) {
    println!("greetings, {}", name);
}

fn add(a: i32, b: i32) -> i32{
    a + b // this is an expression, it's value automatically becomes the return value
}
