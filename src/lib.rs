use rand::prelude::SliceRandom;
use rand;
#[allow(dead_code, unused_imports)]

const DIGITS: [&'static str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
const LOWERCASE: [&'static str; 26] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
const UPPERCASE: [&'static str; 26] = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
const SPECIAL: [&'static str; 9] = ["!", "$", "%", "(", ")", "/", "#", "+", "?"];

pub fn generate_pass<'a>(len: i32, config: &'a Config) -> String{
    
    /*let len = rand::thread_rng().gen_range(0..len)
        .try_into()
        .unwrap_or_else(|_| {5});*/
    let out: Vec<&'static str>  = config.chars.choose_multiple(
        &mut rand::thread_rng(),
        len.try_into().unwrap_or_else(|_| {
            // Handle Error case if no valid USIZE is provided for password length
            eprintln!("No valid length provided. Expected usize but got {}. Defaulting to 16", len);
            16
        }))
        .map(|x| {x.to_owned()})
        .collect();
    out.join("")
}


#[derive(Clone)]
pub struct Config{
    pub chars: Vec<&'static str>,
    pub len: i32,
}

impl Config{
    pub fn new(len: i32, uppercase: bool, lowercase: bool, digits: bool, special: bool) -> Config{
        let mut final_chars: Vec<&'static str> = Vec::new();
        if uppercase{
            final_chars.append(Vec::from(UPPERCASE).as_mut())
        }
        if lowercase{
            final_chars.append(Vec::from(LOWERCASE).as_mut())
        }
        if digits{
            final_chars.append(Vec::from(DIGITS).as_mut())
        }
        if special{
            final_chars.append(Vec::from(SPECIAL).as_mut())
        }

        Config { chars: Vec::from(final_chars), len: len }
    }
}