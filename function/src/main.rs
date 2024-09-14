fn main() {
    println!("Hello, world!");
    my_function(); // calling function
                   // my_function --> snake case
    another_function(5);
    print_labeled_measurement(5, 'h');
    // expression
    add(4, 5);
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn my_function() {
    println!("Hello form my function");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// add function
fn add(x: i32, y: i32) {
    let add: i32 = x + y;
    println!("Hello from add function: {add}")
}
