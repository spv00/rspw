#![allow(dead_code, unused_imports, unreachable_code, unused_variables, unused_mut)]

use std::fmt::Display;
use std::fs;
use rand::prelude::SliceRandom;
use rand;
use libm;

const DIGITS: [&'static str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
const LOWERCASE: [&'static str; 26] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
const UPPERCASE: [&'static str; 26] = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
const SPECIAL: [&'static str; 9] = ["!", "$", "%", "(", ")", "/", "#", "+", "?"];

pub fn generate_pass<'a>(config: &'a Config) -> String{
    //Generate the random string

    let mut out: String = String::new();
    for i in 0..config.len.clone(){
        out.push(
            config.chars.choose(&mut rand::thread_rng()).unwrap().to_owned().to_owned().chars().next().expect("Something went wrong")
        );
    }

    out
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

pub fn calc_entropy(len: i32, pool_size: i32) -> f64{
    //Calculate the passwords entropy (log(pow(poolSize, length), 2))
    libm::log2(libm::pow(pool_size.into(), len.into()).into()).round()
}

pub fn generate_final<>(config: Config) -> String{
    //let config: Config = Config::new(5, true, true, true, false);
    let entropy = calc_entropy(config.len, config.chars.len() as i32);
    format_pass(generate_pass(&config), entropy)
}

#[derive(Clone, Debug)]
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

    pub fn parse(args: Vec<String>) -> Result<Config, &'static str>{
        //Parse an array of string primitives to a config. For example -l 16 -p uld wil result in a config of length 16 and with an uppercase, lowercase and digit pool
        //returns either the parsed Config or the default config
        let (mut len, mut uppercase, mut lowercase, mut digits, mut special) = (12, true, true, true, false);
        if args.len() <= 1{
            return Ok(Config::default());
        }
        len = args[2].parse::<i32>().unwrap_or(12);
        if args.contains(&"-p".to_string()){
            (uppercase, lowercase, digits, special) = (false, false, false, false);
            let pool_i = args.iter().position(|x| {x == "-p"}).unwrap().to_owned();
            let pool = &args[pool_i + 1];
            if pool.contains(&"u".to_string()){uppercase = true}
            if pool.contains(&"l".to_string()){lowercase = true}
            if pool.contains(&"d".to_string()){digits = true}
            if pool.contains(&"s".to_string()){special = true}
        }
        

        return Ok(Config::new(len, uppercase, lowercase, digits, special));
    }
}

impl Display for Config{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "length: {}, character pool: [{}]", self.len, self.chars.join(""))
    }
}

impl Default for Config{
    fn default() -> Self {
        Config::new(12, true, true, true, false)
    }
}