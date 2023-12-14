fn main() {
    let first_name: String = String::from("Nnaemeka");
    say_hello(&first_name); // we have passed a reference to first_name here withour transferring ownership
    println!(
        "First name is still {} in the main function where it was declared.",
        first_name
    ); // we can still safely use it here!
}

fn say_hello(s: &String) {
    println!("Hello there, {}", s);
}
