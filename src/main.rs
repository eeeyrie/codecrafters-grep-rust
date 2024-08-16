use std::env;
use std::io;
use std::process;
use std::collections::VecDeque;

enum CharacterClass {
    AnyDigit,
    AnyAlphanumeric,
    LiteralCharacter(char),
    PosCharacter(String),
    NegCharacter(String),
}

fn parse_pattern<'a>(pattern: &'a str) -> Vec<CharacterClass> {
    let mut pattern_as_enums: Vec<CharacterClass> = Vec::new();
    let mut pattern_iterator = pattern.chars();
    while let Some(current_char) = pattern_iterator.next() {
        dbg!(current_char);
        pattern_as_enums.push(match current_char {
            '\\' => match pattern_iterator.next() {
                Some('d') => CharacterClass::AnyDigit,
                Some('w') => CharacterClass::AnyAlphanumeric,
                Some('\\') => CharacterClass::LiteralCharacter('\\'),
                _ => continue
            },
            '[' => {
                let mut characters: String = String::new();
                let mut is_positive_class: bool = true;

                match pattern_iterator.next() {
                    Some('^') => is_positive_class = false,
                    Some(chara) => characters.push(chara),
                    None => break
                }

                while let Some(current_class_char) = pattern_iterator.next() {
                    match current_class_char {
                        ']' => break,
                        _ => characters.push(current_class_char)
                    }
                }
                
                if is_positive_class {
                    CharacterClass::PosCharacter(characters)
                } else {
                    CharacterClass::NegCharacter(characters)
                }
            }
            _ => CharacterClass::LiteralCharacter(current_char)
        })
    }
    
    return pattern_as_enums
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let mut parsed_pattern: VecDeque<CharacterClass> = VecDeque::from(parse_pattern(pattern));
    let mut input_iterator = input_line.chars();
    while let Some(chara) = input_iterator.next() {
        if let Some(current_class) = parsed_pattern.pop_front() {
            match current_class {
                CharacterClass::AnyDigit => if !chara.is_numeric() {return false},
                CharacterClass::AnyAlphanumeric => if !chara.is_alphanumeric() {return false},
                CharacterClass::LiteralCharacter(character) => if chara != character {return false},
                CharacterClass::NegCharacter(characters) => if characters.contains(chara) {return false},
                CharacterClass::PosCharacter(characters) => if !characters.contains(chara) {return false},
                _ => panic!("Unhandled pattern: {}", pattern)
            }
        }
    }
    return true
    
    //match pattern {
    //    "\\d" => return input_line.contains(|character: char| character.is_numeric()),
    //    "\\w" => return input_line.contains(|character: char| character.is_alphanumeric()),
    //    p if p.chars().count() == 1 => return input_line.contains(pattern),
    //    p if p.starts_with("[^") && p.ends_with(']') => {
    //        let trimmed_pattern: &str = &pattern.trim_start_matches('[').trim_end_matches(']')[..];
    //        return input_line.contains(|character: char| !trimmed_pattern.contains(character))
    //    },
    //    p if p.starts_with('[') && p.ends_with(']') => {
    //        let trimmed_pattern: &str = &pattern.trim_start_matches('[').trim_end_matches(']')[..];
    //        return input_line.contains(|character: char| trimmed_pattern.contains(character))
    //    },
    //    _ => panic!("Unhandled pattern: {}", pattern)
    //}
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Uncomment this block to pass the first stage
    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
