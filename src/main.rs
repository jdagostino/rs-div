//use rand::Rng;
use std::fmt;



// A broken line is "yin" and a solid line is "yang"; lines may be either static (young) or moving (old)
#[derive(Debug, Clone, Copy)]
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
        //flip three coins. heads are worth 3 and tails 2. sum the numbers

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

    fn is_changing(&self) -> bool {
        matches!(self, Line::ChangingYang | Line::ChangingYin)
    }

    fn change_line(&self) -> Line {
        match self {
            Line::ChangingYang => Line::StaticYin,
            Line::ChangingYin  => Line::StaticYang,
            other => *other
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
struct Hexagram
{
    /// Lines are stored in the order generated, 
    /// i.e. lines[0] is the bottom line of the hexagram. 
    lines: [Line; 6],
    number: u8
    //name, image, judgement, etc through lookup by number?
}

impl Hexagram {
    /// Generates a random hexagram, with changing and static lines. 
    /// Use to generate the present hexagram
    fn generate_present(mut rng: &mut impl rand::Rng) -> Self {
        let lines: [Line; 6] = (0..6).map(|_| {Line::generate(&mut rng)}).collect::<Vec<_>>().try_into().unwrap();
        //TODO lookup number
        Hexagram{lines, number: 0}
    }

    /// Returns a vector of the changing lines. 
    /// Indexes are in the traditional I Ching order, i.e. 
    /// the first line generated is the bottom line is 1, 
    /// the top line is 6 which is the last line that was generated.
    fn get_changing_lines(&self) -> Vec<u8> {
        self.lines.iter().enumerate()
            .filter_map(|(i, line)| {
                if line.is_changing() { Some((i+1) as u8)} else {None}
            }).collect()
    }

    /// Given a "present" hexagram, return the future hexagram
    /// (with all changing lines changed)
    fn change(&self) -> Option<Hexagram> {
        if self.get_changing_lines().is_empty() {
            return None;
        }

        let future_lines: [Line; 6] = self.lines.iter()
            .map(|line| line.change_line())
            .collect::<Vec<_>>().try_into().unwrap();

        // TODO lookup number
        Some(Hexagram{lines: future_lines, number:0})
    }

}

impl fmt::Display for Hexagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i,line) in self.lines.iter().rev().enumerate() {
            let line_num = 6-i;
            write!(f, "{} {}\n", line_num, line)?;
        }

        //TODO name, image, judgement, etc

        Ok(())
    }
}


#[derive(Debug)]
struct Divination {
    present_hexagram: Hexagram,
    future_hexagram: Option<Hexagram>
}

impl Divination {
    // the result is locked in on creation, much like your own fate in real life
    fn new() -> Self {
        let mut rng = rand::rng();
        let present_hexagram = Hexagram::generate_present(&mut rng);
        let future_hexagram = present_hexagram.change();

        Divination {present_hexagram, future_hexagram} 
    }
}

impl fmt::Display for Divination {
    // show present, show changes, show future
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", &self.present_hexagram)?;

        if let Some(future) = &self.future_hexagram {
            write!(f, "-- changing to -- \n{}", future)?;
        } else {
            write!(f, "-- unchanging --\n")?;
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
