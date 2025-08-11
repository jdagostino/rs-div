//use rand::Rng;
use std::fmt;
use std::fs;
use std::vec;


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

    // King Wen sequence number, look it up from the table
    king_wen_number: u8,

    // kingwen_number: u8,

    //upper_trigram: u8,
    //lower_trigram: u8,

    // how do we want to do name and other stuff? translations?
}

impl Hexagram {
    /// Generates a random hexagram, with changing and static lines. 
    /// Use to generate the present hexagram
    fn generate_present(mut rng: &mut impl rand::Rng) -> Self {
        let lines: [Line; 6] = (0..6).map(|_| {Line::generate(&mut rng)}).collect::<Vec<_>>().try_into().unwrap();
        let number = Hexagram::calculate_number(&lines);
        Hexagram{lines, king_wen_number: number}
    }

    /// Given the six lines, return the hexagram number (1 to 64) from the King Wen sequence.
    fn calculate_number(lines: &[Line; 6]) -> u8
    {
        // first calculate the binary number. This is basically the Fu Xi number except 0 indexed.
        let binary_num = lines.iter().enumerate().
            fold(0, |acc, (i, line)|{
                let bit = match line {
                    Line::StaticYin | Line::ChangingYin => 0,
                    Line::StaticYang | Line::ChangingYang => 1
                };
                acc | (bit << i)
        });

        // Look up the King Wen number from the table. 
        Hexagram::KING_WEN_SEQUENCE[binary_num]
    }

    fn calculate_trigram(lines: &[Line]) -> usize
    {
        
        let binary_num = lines.iter().enumerate().
            fold(0, |acc, (i, line)|{
                let bit = match line {
                    Line::StaticYin | Line::ChangingYin => 0,
                    Line::StaticYang | Line::ChangingYang => 1
                };
                acc | (bit << i)
        });
        binary_num + 1 // range 1-64
    }

    const TRIGRAMS: [&str;8] = [
        "Earth",      // 000 - kun, 坤
        "Thunder",    // 001 - zhen, 震
        "Water",      // 010 - kan, 坎
        "Lake",       // 011 - dui, 兌
        "Mountain",   // 100 - gen, 艮
        "Fire",       // 101 - li, 離
        "Wind",       // 110 - xun, 巽
        "Heaven"      // 111 - qian, 乾
    ];

    // Complete mapping from binary index (0-63) to King Wen hexagram numbers (1-64)
    // Binary index is calculated from bottom line = bit 0, top line = bit 5
    // where Yin = 0, Yang = 1
    const KING_WEN_SEQUENCE: [u8; 64] = [
        2,  // 000000 ☷☷ Kun (The Receptive)
        24, // 000001 ☷☳ Fu (Return)
        7,  // 000010 ☷☵ Shi (The Army) 
        19, // 000011 ☷☱ Lin (Approach)
        15, // 000100 ☷☶ Qian (Modesty)
        36, // 000101 ☷☲ Ming Yi (Darkening of the Light)
        46, // 000110 ☷☴ Sheng (Pushing Upward)
        11, // 000111 ☷☰ Tai (Peace)
        16, // 001000 ☳☷ Yu (Enthusiasm)
        51, // 001001 ☳☳ Zhen (The Arousing)
        40, // 001010 ☳☵ Jie (Deliverance)
        54, // 001011 ☳☱ Gui Mei (The Marrying Maiden)
        62, // 001100 ☳☶ Xiao Guo (Preponderance of the Small)
        55, // 001101 ☳☲ Feng (Abundance)
        32, // 001110 ☳☴ Heng (Duration)
        34, // 001111 ☳☰ Da Zhuang (The Power of the Great)
        8,  // 010000 ☵☷ Pi (Holding Together)
        3,  // 010001 ☵☳ Zhun (Difficulty at the Beginning)
        29, // 010010 ☵☵ Kan (The Abysmal)
        60, // 010011 ☵☱ Jie (Limitation)
        39, // 010100 ☵☶ Jian (Obstruction)
        63, // 010101 ☵☲ Ji Ji (After Completion)
        48, // 010110 ☵☴ Jing (The Well)
        5,  // 010111 ☵☰ Xu (Waiting)
        45, // 011000 ☱☷ Cui (Gathering Together)
        17, // 011001 ☱☳ Sui (Following)
        47, // 011010 ☱☵ Kun (Oppression)
        58, // 011011 ☱☱ Dui (The Joyous)
        31, // 011100 ☱☶ Xian (Influence)
        49, // 011101 ☱☲ Ge (Revolution)
        28, // 011110 ☱☴ Da Guo (Preponderance of the Great)
        43, // 011111 ☱☰ Guai (Breakthrough)
        23, // 100000 ☶☷ Po (Splitting Apart)
        27, // 100001 ☶☳ Yi (The Corners of the Mouth)
        4,  // 100010 ☶☵ Meng (Youthful Folly)
        41, // 100011 ☶☱ Sun (Decrease)
        52, // 100100 ☶☶ Gen (Keeping Still)
        22, // 100101 ☶☲ Pi (Grace)
        18, // 100110 ☶☴ Gu (Work on What Has Been Spoiled)
        26, // 100111 ☶☰ Da Xu (The Taming Power of the Great)
        35, // 101000 ☲☷ Jin (Progress)
        21, // 101001 ☲☳ Shi He (Biting Through)
        64, // 101010 ☲☵ Wei Ji (Before Completion)
        38, // 101011 ☲☱ Kui (Opposition)
        56, // 101100 ☲☶ Lu (The Wanderer)
        30, // 101101 ☲☲ Li (The Clinging)
        50, // 101110 ☲☴ Ding (The Cauldron)
        14, // 101111 ☲☰ Da You (Possession in Great Measure)
        20, // 110000 ☴☷ Guan (Contemplation)
        42, // 110001 ☴☳ Yi (Increase)
        59, // 110010 ☴☵ Huan (Dispersion)
        61, // 110011 ☴☱ Zhong Fu (Inner Truth)
        53, // 110100 ☴☶ Jian (Development)
        37, // 110101 ☴☲ Jia Ren (The Family)
        57, // 110110 ☴☴ Xun (The Gentle)
        9,  // 110111 ☴☰ Xiao Xu (The Taming Power of the Small)
        12, // 111000 ☰☷ Pi (Standstill)
        25, // 111001 ☰☳ Wu Wang (Innocence)
        6,  // 111010 ☰☵ Song (Conflict)
        10, // 111011 ☰☱ Lu (Treading)
        33, // 111100 ☰☶ Dun (Retreat)
        13, // 111101 ☰☲ Tong Ren (Fellowship with Men)
        44, // 111110 ☰☴ Gou (Coming to Meet)
        1   // 111111 ☰☰ Qian (The Creative)
    ];

    /*
    /// maps the Fu Xi sequence number to the King Wen sequence number because it's not very calculable
    const KING_WEN_SEQUENCE_MAP: [u8;64] = [
        0,  // invalid
        2,  // 000000 -> Hexagram 2 (The Receptive)
        24, // 000001 -> Hexagram 24 (Return)
        7,  // 000010 -> Hexagram 7 (The Army)
        // ... all 64 mappings
        1,  // 111111 -> Hexagram 1 (The Creative)

    ];
    */

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

        let future_number = Hexagram::calculate_number(&future_lines);
        Some(Hexagram{lines: future_lines, king_wen_number: future_number})
    }

}

impl fmt::Display for Hexagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HEXAGRAM {} : \n", self.king_wen_number)?;
        let lower_trigram = Hexagram::calculate_trigram(&self.lines[0..2]);

        let upper_trigram = Hexagram::calculate_trigram(&self.lines[3..5]);

        for (i,line) in self.lines.iter().rev().enumerate() {
            let line_num = 6-i;
            write!(f, "{} {}\n", line_num, line)?;
        }

        write!(f, "{} over {}\n", Hexagram::TRIGRAMS[upper_trigram], Hexagram::TRIGRAMS[lower_trigram])?;

        //TODO name, image, judgement, etc

        Ok(())
    }
}

/// Each hexagram has some text associated with it - we load different translations from different files
/// 
/* 
struct Translation {
    kingwen_number: u8,
    name: &String,
    image: &String,
    judgement: &str,
    changing_lines: [&str;6]
}


impl Translation {
    /// vow: result will have 65 elements and index 0 doesn't count
    fn init_translations(filename: &str) -> Result<Vec<Translation>, Box<dyn std::error::Error>>
    {
        let file_conts = fs::read_to_string(filename)?;
        let mut tr_array: Vec<Translation> = Vec::new();

        // TODO: this file format sucks, replace with yaml or something
        // CSV - King Wen number, name, image, judgement, line 1, line 2, line 3, line 4, line 5, line 6
        // First line in file contains that header, following lines contain each hexagram (in Fu Xi order)
        for (i, line) in file_conts.lines().enumerate() {
            let fields: Vec<&str> = line.split(',').collect();
            if (fields.len() != 10) {
            
            }

            tr_array.push(Translation{
                kingwen_number:fields[0].parse()?,
                name:fields[1],
                image: fields[2],
                judgement: fields[3],
                changing_lines: fields[4..10]});
        }

        Ok(tr_array)

    }
}
*/

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

        // look some stuff up now
        // we look up by the Fu Xi number because we're in a computer, 
        // but display the King Wen number because it's traditional

        for index in &self.present_hexagram.get_changing_lines() {
            write!(f, "line {} - {}\n", index, "")?;
        }

        // todo special case of hex 1 -> 64 and vice versa

        if let Some(future) = &self.future_hexagram {
            write!(f, "-- changing to -- \n{}", future)?;
        } else {
            write!(f, "-- unchanging --\n")?;
        }

        Ok(())
    }
}

fn main() {
    //let wade_giles_translation = Translation::init_translations("wade_giles.csv");

    let mut input = String::new();
    
    println!("Think deeply on your question and press Enter when ready...");
    
    std::io::stdin().read_line( &mut input).expect("something went wrong");

    let div = Divination::new();
    println!("{}",div);

}
