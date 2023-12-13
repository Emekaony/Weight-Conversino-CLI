fn calc_weight_on_mars(weight: &f32) -> f32 {
    let weight_on_mars: f32 = (weight / 9.81) * 3.711;
    return weight_on_mars;
}

fn main() {
    let weight_on_earth: f32 = 187.6;
    let weight_on_mars: f32 = calc_weight_on_mars(&weight_on_earth);
    // used cargo expand to see macros in action.
    println!(
        "My weight on Earch is {}lbs.\nAnd my weight on Mars is {}kg",
        weight_on_earth, weight_on_mars
    )
}
