use rand::Rng;

fn main() {

    let mut rng = rand::rng();

    let mut input = String::new();
    
    println!("Think deeply on your question and press Enter when ready...");
    
    std::io::stdin().read_line(&mut input).expect("whoops");

    let coin = rng.random_bool(0.5);
    if coin == true {
        println!("Heads! Go for it");
    } else {
        println!("Tails! Probably means 'no'.")
    }
}
