use rand::Rng;
use std::fmt;

struct Divination {
    cointoss: bool 
}

impl Divination {
    // the result is locked in on creation, much like your own fate in real life
    fn new() -> Self {
        let mut rng = rand::rng();

        let cointoss = rng.random_bool(0.5);
        
        Divination {cointoss} 
    }
}

impl fmt::Display for Divination {
    // emit text
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.cointoss == true {
            write!(f, "Heads! Go for it")?;
        } else {
            write!(f, "Tails! Probably means 'no'.")?;
        }

        Ok(())
    }
}

fn main() {
    let mut input = String::new();
    
    println!("Think deeply on your question and press Enter when ready...");
    
    std::io::stdin().read_line( &mut input).expect("whoops");

    let div = Divination::new();
    println!("{}",div);

}
