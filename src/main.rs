fn main() {
    let first_name: String = String::from("Nnaemeka");
    say_hello(&first_name); // we have passed a reference to first_name here withour transferring ownership
    println!(
        "First name is still {} in the main function where it was declared.",
        first_name
    ); // we can still safely use it here!
    let mut hello_world: String = String::from("Hello");
    append_world(&mut hello_world);
    println!("hello world variable is now {}", hello_world);
}

// immutable references
fn say_hello(s: &String) {
    println!("Hello there, {}", s);
}

// we also have mutable references
fn append_world(first_name: &mut String) {
    first_name.push_str(", world");
}
