use std::fs;

use colored::Colorize;
use rand::prelude::SliceRandom;
use rand;
use libm;
#[allow(dead_code, unused_imports)]

const DIGITS: [&'static str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
const LOWERCASE: [&'static str; 26] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
const UPPERCASE: [&'static str; 26] = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
const SPECIAL: [&'static str; 9] = ["!", "$", "%", "(", ")", "/", "#", "+", "?"];

pub fn generate_pass<'a>(config: &'a Config) -> String{
    
    /*let len = rand::thread_rng().gen_range(0..len)
        .try_into()
        .unwrap_or_else(|_| {5});*/
    //Choose multiple chars from the provided array in the config using choose_multiple from SliceRandom
    let out: Vec<&'static str>  = config.chars.choose_multiple(
        &mut rand::thread_rng(),
        config.len.try_into().unwrap_or_else(|_| {
            // Handle Error case if no valid USIZE is provided for password length
            eprintln!("No valid length provided. Expected usize but got {}. Defaulting to 16", config.len);
            16
        }))
        .map(|x| {x.to_owned()})
        .collect();
    out.join("")
}

pub fn format_pass(pass: String, entropy: f64) -> String{
    //Format the output using the "format" file
    /*
        <pasword> is replaced by the actual password
    */

    let format = fs::read_to_string("format").unwrap_or_else(|_| {"
    =====
    <password>

    Entropy: <entropy>
    =====
    ".to_string()});
    let format = format.replace("<password>", {
        if entropy < 32_f64{pass.as_str()}
        else{pass.as_str()}
    }).replace("<entropy>", entropy.to_string().as_str());
    format
}

pub fn calc_entropy<'a>(len: i32, pool_size: i32) -> f64{
    //Calculate the passwords entropy (log(pow(poolSize, length), 2))
    libm::log2(libm::pow(pool_size.into(), len.into()).into()).round()
}

pub fn generate_final<>() -> String{
    let config: Config = Config::new(5, true, true, true, false);
    let entropy = calc_entropy(config.len, config.chars.len() as i32);
    format_pass(generate_pass(&config), entropy)
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