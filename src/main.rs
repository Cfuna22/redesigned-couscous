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


// fn main() {
//     let number = 6;

//     let label = if number % 2 == 0 {"even"} else {"odd"};

//     println!("{}", label);

//     let mut counter = 0;
//     let result = loop {
//         counter += 1;

//         if counter == 5 {
//             break counter * 2;
//         }
//     };

//     println!("Loop result: {}", result);

//     let mut n = 3;
//     while n!= 0 {
//         println!("{}", n);
//         n-=1;
//     }

//     for i in 1..4 {
//         println!("{}", i);
//     }

//     let scores = [59, 89, 9, 90, 04, 04, 85, 36, 94, 36, 06, 934, 984, 84];
//     for score in scores {
//         println!("score is: {}", score);
//     }
// }

fn main() {
    let s1 = String::from("Hello"); // create a heap allocated growablestring different from the &str
    let s2 = s1.clone();
    println!("{}, {}", s2, s1);

    let ss = String::from("Hello");
    let len = calculate_length(&ss);
    println!("the length of {} is {}", ss, len)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
