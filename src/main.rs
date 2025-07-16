//use rand::Rng;
use std::fmt;



// A broken line is "yin" and a solid line is "yang"; lines may be either static (young) or moving (old)
#[derive(Debug)]
enum Line {
    ChangingYin, //6
    StaticYang, //7
    StaticYin, //8
    ChangingYang, //9
}

#[allow(dead_code)]
#[derive(Debug)]
enum Aspect {
    Yin,
    Yang
}

impl Line {
    fn generate(rng: &mut impl rand::Rng) -> Self {
        //flip four coins. heads are worth 3 and tails 2. sum the numbers
        // there's some fancy syntax here that I should get used to

        /* 
        // first I did it the readable way, like this:

        let mut sum = 0;
        for _ in 0..3 {
            let cointoss = rng.random_bool(0.5);

            if cointoss == true {
                sum += 3;
            } else {
                sum += 2;
            }
        }
        */

        // fancy syntax version. I guess it's a one-liner?
        let sum = (0..3).map(|_| {if rng.random_bool(0.5) {3} else {2}}).sum();

        match sum {
            6 => Line::ChangingYin,
            7 => Line::StaticYang,
            8 => Line::StaticYin,
            9 => Line::ChangingYang,
            _ => unreachable!()
        }
    }


    #[allow(dead_code)]
    fn get_aspect(&self) -> Aspect {
        match self {
            Line::ChangingYang => Aspect::Yang,
            Line::StaticYang => Aspect::Yang,
            Line::ChangingYin => Aspect::Yin,
            Line::StaticYin => Aspect::Yin
        }
    }

}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Line::StaticYang   => "---------",
            Line::StaticYin    => "---   ---",
            Line::ChangingYang => "----o----",
            Line::ChangingYin  => "--- x ---"
        })
    }
}


#[derive(Debug)]
struct Divination {
    lines: [Line; 6]
}

impl Divination {
    // the result is locked in on creation, much like your own fate in real life
    fn new() -> Self {
        let mut rng = rand::rng();
        let lines: [Line; 6] = (0..6).map(|_| {Line::generate(&mut rng)}).collect::<Vec<_>>().try_into().unwrap();

        //let cointoss = rng.random_bool(0.5);        
        Divination {lines} 
    }
}

impl fmt::Display for Divination {
    // emit text for initial line draw
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i,line) in self.lines.iter().rev().enumerate() {
            let line_num = 6-i;
            write!(f, "{} {}\n", line_num, line)?;
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
