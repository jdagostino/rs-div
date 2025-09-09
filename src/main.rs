use rand::RngCore;
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

impl Line {
    fn to_string(&self) -> String {
        match self {
            Line::StaticYang   => "---------".to_string(),
            Line::StaticYin    => "---   ---".to_string(),
            Line::ChangingYang => "----o----".to_string(),
            Line::ChangingYin  => "--- x ---".to_string()
        }
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

impl Hexagram {
    fn to_string(&self) -> String {
        let mut result = format!("HEXAGRAM {} : \n", self.king_wen_number);
        let lower_trigram = Hexagram::calculate_trigram(&self.lines[0..3]);
        let upper_trigram = Hexagram::calculate_trigram(&self.lines[3..6]);

        for (i, line) in self.lines.iter().rev().enumerate() {
            let line_num = 6-i;
            result.push_str(&format!("{} {}\n", line_num, line.to_string()));
        }

        result.push_str(&format!("{} over {}\n", Hexagram::TRIGRAMS[upper_trigram], Hexagram::TRIGRAMS[lower_trigram]));

        //TODO name, image, judgement, etc

        result
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

impl IChingError {
    fn to_string(&self) -> String {
        match self {
            IChingError::FileError(e) => format!("File error: {}", e),
            IChingError::JsonError(e) => format!("JSON parsing error: {}", e),
            IChingError::DataError(msg) => format!("Data validation error: {}", msg),
        }
    }
}

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

impl<'tr> Divination<'tr> {
    // show present, show changes, show future
    fn to_string(&self) -> String {
        let mut result = format!("{}\n", self.present_hexagram.to_string());

        // look some stuff up now
        // we look up by the Fu Xi number because we're in a computer, 
        // but display the King Wen number because it's traditional

        for index in &self.present_hexagram.get_changing_lines() {
            result.push_str(&format!("line {} - {}\n", index, ""));
        }

        // todo special case of hex 1 -> 64 and vice versa

        if let Some(future) = &self.future_hexagram {
            result.push_str(&format!("-- changing to -- \n{}", future.to_string()));
        } else {
            result.push_str("-- unchanging --\n");
        }

        result
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
    println!("{}", div.to_string());

}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn test_line_generation_known_values() {
        use rand::SeedableRng;
        use rand::rngs::StdRng;
        
        let mut rng = StdRng::seed_from_u64(42);
        
        // Test with known seed - these are the actual results from seed 42
        let line1 = Line::generate(&mut rng);
        let line2 = Line::generate(&mut rng);
        let line3 = Line::generate(&mut rng);
        
        assert!(matches!(line1, Line::ChangingYin));
        assert!(matches!(line2, Line::ChangingYang));
        assert!(matches!(line3, Line::StaticYang));
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
        assert_eq!(Line::StaticYang.to_string(), "---------");
        assert_eq!(Line::StaticYin.to_string(), "---   ---");
        assert_eq!(Line::ChangingYang.to_string(), "----o----");
        assert_eq!(Line::ChangingYin.to_string(), "--- x ---");
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
            king_wen_number: Hexagram::calculate_number(&lines),
        };
        
        
        // Create future hexagram by changing the lines
        let future_hexagram = present_hexagram.change();
        
        let translation = HashMap::new();
        let divination = Divination {
            present_hexagram,
            future_hexagram,
            translation: &translation,
        };
        
        let display_output = divination.to_string();
        println!("{}", display_output);
        
        // Check that the output contains expected elements
        assert!(display_output.contains("HEXAGRAM 18"));
        assert!(display_output.contains("line 2 -")); // changing line
        assert!(display_output.contains("line 4 -")); // changing line
        assert!(display_output.contains("-- changing to --"));
        
        // Check that trigram names are displayed correctly
        //assert!(display_output.contains("Mountain over Wind"));
    }


}