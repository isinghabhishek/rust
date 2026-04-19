// system programming language like rust

fn main() {
    println!("Hello, world of ownership!");

    {
        let s = "hello";
        // do stuff with s
    } // this out of scope, and s is no longer valid

    str_type();
    memory_allocation();
    sone_eq_stwo();
    variable_data_clone();
    stack_only_data();
    ownership_function();
    values_scope();
}

fn str_type() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");
}

fn memory_allocation() {
    {
        let s = String::from("hello");
        // do stuff with s
    } // scope over, s no longer valid
}

fn sone_eq_stwo() {
    let s1 = String::from("hello");
    let s2 = s1;
}

fn variable_data_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");
}

fn stack_only_data() {
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");
}

fn ownership_function() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s);            // s's value moves into the function

    let x = 5;                     // x comes into scope

    makes_copy(x);                 // i32 is Copy, so x is still usable after
} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // some_string goes out of scope, drop is called, memory freed.

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
} // some_integer goes out of scope. Nothing special happens.

fn values_scope() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
} // s3 dropped, s2 was moved, s1 dropped.

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn returning_parameter() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
