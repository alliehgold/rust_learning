use rand::Rng;
fn main() {
    let fortunes = ["You’re crushing it!", "Compiling = winning.", "Take a break—you earned it."];
    let random_index = rand::rng().random_range(0..fortunes.len());
    println!("{}", fortunes[random_index]);
}
