use std::io;

fn main() {
    print!("Enter your weight (Kg): ");
    let mut input = String::new();

    // get input from user
    io::stdin().read_line(&mut input).unwrap();

    // convert it to float using the parse function and then unwrap (unsafely)
    let weight: f32 = input.trim().parse().unwrap();

    // pass the weight to the calc function we created
    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on mars is: {}", mars_weight);
    println!("Weight is {}", weight); // this will work beacause numbers are easily copyable. They implement the 'copy' trait.
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    return (weight / 9.81) * 3.711;
}
