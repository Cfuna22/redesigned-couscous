fn main() {
    let age: u32 = 18;
    let temperature: f64 = 40.6;
    let num: i32 = -30;
    let grade: char = 'A';
    let is_active: bool = true;

    println!("age {}, temperature {}, number {}, grade {}, is_active {}", age, temperature, num, grade, is_active);

    // Compound type

    // tuple
    let person: (&str, u32) = ("maya", 19);
    println!("{} is {} years old", person.0, person.1);
    // array
    let score: [u32; 3] = [23, 40, 8];
    println!("the sore was {}", score[1]);
}