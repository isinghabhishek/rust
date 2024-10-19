// NOTE: Variables and mutability

// fn main() {
//     // println!("Hello, world!");

//     let mut age: i32 = 24; // NOTE: by placing mut it will be mutable
//     println!("value of age is {age}");

//     age = 25; // INFO: at the time of variable declaration need to put mut, otherwise it is not mutable
//     println!("value of age is {age}");

//     // Constant are Immutable
//     const pi: f64 = 3.14;
//     println!("PI value is {pi}");

//     Const THREE_HOURS_IN_SECONDS: u32 = 60 * 68 * 3;
//     println!("value of THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");
// }

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
