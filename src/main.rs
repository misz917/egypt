use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut chars: HashMap<char, char> = HashMap::new();

    for line in handle.lines() {
        match line {
            Ok(line) => {
                let mut coded_line = String::new();

                for c in line.chars() {
                    let output_char: char;
                    if c.is_whitespace() {
                        output_char = c;
                    } else if let Some(char_found) = chars.get(&c) {
                        output_char = *char_found;
                    } else {
                        let random_hieroglyph_code = rand::random_range(0x13000..=0x1342F);
                        let random_hieroglyph = char::from_u32(random_hieroglyph_code).unwrap();
                        chars.insert(c, random_hieroglyph);
                        output_char = random_hieroglyph;
                    }
                    coded_line.push(output_char);
                    coded_line.push(' ');
                }

                println!("{}", coded_line);
            }
            Err(e) => panic!("{}", e),
        }
    }
}
