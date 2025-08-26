use rand::RngCore;
use std::fmt;
use std::fs;
use std::vec;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Error;


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
        binary_num // range 0-7 for direct array indexing
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
        let lower_trigram = Hexagram::calculate_trigram(&self.lines[0..3]);

        let upper_trigram = Hexagram::calculate_trigram(&self.lines[3..6]);

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


#[derive(Debug, Deserialize, Clone)]
struct HexagramName {
    english: String,
    chinese: Option<String>,
    pinyin: Option<String>,
    wade_giles: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct Trigram {
    name: String,
    element: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct Trigrams {
    upper: Trigram,
    lower: Trigram,
}

#[derive(Debug, Deserialize, Clone)]
struct Commentary {
    image: Option<String>,
    judgement: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct HexagramLine {
    position: u8,
    #[serde(rename = "type")]
    line_type: String, // "yin" or "yang" 
    text: String,
    commentary: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct HexagramData {
    king_wen_number: u8,
    fu_xi_number: Option<u8>,
    name: HexagramName,
    trigrams: Trigrams,
    image: String,
    judgement: String,
    commentary: Option<Commentary>,
    lines: Vec<HexagramLine>,
    keywords: Option<Vec<String>>,
    sequence_notes: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    translation_source: Option<String>,
    version: Option<String>,
    notes: Option<String>,
}

#[derive(Debug, Deserialize)]
struct IChingData {
    hexagrams: Vec<HexagramData>,
    metadata: Option<Metadata>,
}

// Error handling for JSON loading
#[derive(Debug)]
enum IChingError {
    FileError(std::io::Error),
    JsonError(serde_json::Error),
    DataError(String),
}

impl fmt::Display for IChingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IChingError::FileError(e) => write!(f, "File error: {}", e),
            IChingError::JsonError(e) => write!(f, "JSON parsing error: {}", e),
            IChingError::DataError(msg) => write!(f, "Data validation error: {}", msg),
        }
    }
}
impl std::error::Error for IChingError {}

impl From<std::io::Error> for IChingError {
    fn from(error: std::io::Error) -> Self {
        IChingError::FileError(error)
    }
}

impl From<serde_json::Error> for IChingError {
    fn from(error: serde_json::Error) -> Self {
        IChingError::JsonError(error)
    }
}

type IChingTranslation = HashMap<u8, HexagramData>;

#[derive(Debug)]
struct Divination<'tr> {
    present_hexagram: Hexagram,
    future_hexagram: Option<Hexagram>,
    translation: &'tr IChingTranslation,
}

impl<'tr> Divination<'tr> {
    // the result is locked in on creation, much like your own fate in real life
    fn new(translation: &'tr IChingTranslation) -> Self {
        let mut rng = rand::rng();
        let present_hexagram = Hexagram::generate_present(&mut rng);
        let future_hexagram = present_hexagram.change();

        Divination {present_hexagram, future_hexagram, translation} 
    }
}

impl<'tr> fmt::Display for Divination<'tr> {
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

fn load_hexagram_data(filename: &str) -> Result<HashMap<u8, HexagramData>, IChingError> {
    let iching_data: IChingData = serde_json::from_str(&fs::read_to_string(filename)?)?;

    // Validate we have exactly 64 hexagrams
    if iching_data.hexagrams.len() != 64 {
        return Err(IChingError::DataError(
            format!("Expected 64 hexagrams, found {}", iching_data.hexagrams.len())
        ));
    }

    let mut hexagram_map = HashMap::new();
    for hexagram in iching_data.hexagrams {
        // Validate hexagram has 6 lines
        if hexagram.lines.len() != 6 {
            return Err(IChingError::DataError(
                format!("Hexagram {} has {} lines, expected 6", 
                       hexagram.king_wen_number, hexagram.lines.len())
            ));
        }
        
        // Validate King Wen number range
        if hexagram.king_wen_number < 1 || hexagram.king_wen_number > 64 {
            return Err(IChingError::DataError(
                format!("Invalid King Wen number: {}", hexagram.king_wen_number)
            ));
        }
        
        hexagram_map.insert(hexagram.king_wen_number, hexagram);
    }

    Ok(hexagram_map)

}

fn main() {
    //todo handle errors better
    let wilhelm_baynes_translation = load_hexagram_data("data/wilhelm_baynes.json").expect("failed to read input file data/wilhelm_baynes.json");

    let mut input = String::new();
    
    println!("Think deeply on your question and press Enter when ready...");
    
    std::io::stdin().read_line( &mut input).expect("something went wrong");

    let div = Divination::new(&wilhelm_baynes_translation);
    println!("{}",div);

}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn test_line_generation_known_values() {
        // Create a custom RNG that returns specific boolean values
        struct MockRng {
            values: Vec<bool>,
            index: usize,
        }
        
        impl rand::RngCore for MockRng {
            fn next_u32(&mut self) -> u32 {
                if self.index < self.values.len() {
                    // Return max value for true, 0 for false to make random_bool work correctly
                    let val = if self.values[self.index] { u32::MAX } else { 0 };
                    self.index += 1;
                    val
                } else {
                    0
                }
            }
            
            fn next_u64(&mut self) -> u64 {
                self.next_u32() as u64
            }
            
            fn fill_bytes(&mut self, dest: &mut [u8]) {
                for byte in dest.iter_mut() {
                    *byte = self.next_u32() as u8;
                }
            }
            
        }
        
        
        // Test 3 tails (false) = sum 6 = ChangingYin
        let mut rng = MockRng { values: vec![false, false, false], index: 0 };
        let line = Line::generate(&mut rng);
        assert!(matches!(line, Line::ChangingYin)); // 3 tails = 6
        
        // Test 3 heads (true) = sum 9 = ChangingYang
        let mut rng = MockRng { values: vec![true, true, true], index: 0 };
        let line = Line::generate(&mut rng);
        assert!(matches!(line, Line::ChangingYang)); // 3 heads = 9
    }

    #[test]
    fn test_line_change() {
        assert_eq!(matches!(Line::ChangingYin.change_line(), Line::StaticYang), true);
        assert_eq!(matches!(Line::ChangingYang.change_line(), Line::StaticYin), true);
        assert_eq!(matches!(Line::StaticYin.change_line(), Line::StaticYin), true);
        assert_eq!(matches!(Line::StaticYang.change_line(), Line::StaticYang), true);
    }

    #[test]
    fn test_line_get_aspect() {
        assert!(matches!(Line::ChangingYang.get_aspect(), Aspect::Yang));
        assert!(matches!(Line::StaticYang.get_aspect(), Aspect::Yang));
        assert!(matches!(Line::ChangingYin.get_aspect(), Aspect::Yin));
        assert!(matches!(Line::StaticYin.get_aspect(), Aspect::Yin));
    }

    #[test]
    fn test_line_display() {
        assert_eq!(format!("{}", Line::StaticYang), "---------");
        assert_eq!(format!("{}", Line::StaticYin), "---   ---");
        assert_eq!(format!("{}", Line::ChangingYang), "----o----");
        assert_eq!(format!("{}", Line::ChangingYin), "--- x ---");
    }

    #[test]
    fn test_hexagram_calculate_number() {
        // Test Hexagram 1 (Qian - all yang lines)
        let all_yang = [Line::StaticYang; 6];
        assert_eq!(Hexagram::calculate_number(&all_yang), 1);
        // Test Hexagram 2 (Kun - all yin lines)
        let all_yin = [Line::StaticYin; 6];
        assert_eq!(Hexagram::calculate_number(&all_yin), 2);
        // that's probably good enough lmao
    }

    #[test]
    fn test_hexagram_change_with_changing_lines() {
        let lines = [
            Line::StaticYin,      // stays yin
            Line::ChangingYang,   // becomes yin
            Line::StaticYang,     // stays yang
            Line::ChangingYin,    // becomes yang
            Line::StaticYin,      // stays yin
            Line::StaticYang,     // stays yang
        ];
        let hexagram = Hexagram {
            lines,
            king_wen_number: 1,
        };
        
        let future = hexagram.change().unwrap();
        assert_eq!(matches!(future.lines[1], Line::StaticYin), true);
        assert_eq!(matches!(future.lines[3], Line::StaticYang), true);
    }

    #[test]
    fn test_hexagram_change_no_changing_lines() {
        let lines = [Line::StaticYin, Line::StaticYang, Line::StaticYin, 
                     Line::StaticYang, Line::StaticYin, Line::StaticYang];
        let hexagram = Hexagram {
            lines,
            king_wen_number: 1,
        };
        
        assert!(hexagram.change().is_none());
    }

    #[test]
    fn test_calculate_trigram() {
        // Test Earth trigram (000)
        let earth_lines = [Line::StaticYin, Line::StaticYin, Line::StaticYin];
        assert_eq!(Hexagram::calculate_trigram(&earth_lines), 0);
        
        // Test Heaven trigram (111) 
        let heaven_lines = [Line::StaticYang, Line::StaticYang, Line::StaticYang];
        assert_eq!(Hexagram::calculate_trigram(&heaven_lines), 7);
    }

    #[test]
    fn test_divination_display() {
        // Create a test hexagram with known lines
        let lines = [
            Line::StaticYin,      // bottom line (1)
            Line::ChangingYang,   // line 2
            Line::StaticYang,     // line 3
            Line::ChangingYin,    // line 4
            Line::StaticYin,      // line 5
            Line::StaticYang,     // top line (6)
        ];
        let present_hexagram = Hexagram {
            lines,
            king_wen_number: 42,
        };
        
        // Create future hexagram by changing the lines
        let future_hexagram = present_hexagram.change();
        
        let divination = Divination {
            present_hexagram,
            future_hexagram,
        };
        
        let display_output = format!("{}", divination);
        
        // Check that the output contains expected elements
        assert!(display_output.contains("HEXAGRAM 42"));
        assert!(display_output.contains("line 2 -")); // changing line
        assert!(display_output.contains("line 4 -")); // changing line
        assert!(display_output.contains("-- changing to --"));
        
        // Check that trigram names are displayed correctly
        // Lower trigram: lines 0-2 = Yin,Yang,Yang = 011 binary = 3 = "Lake"
        // Upper trigram: lines 3-5 = Yin,Yin,Yang = 100 binary = 1 = "Thunder"
        assert!(display_output.contains("Thunder over Lake"));
    }


}