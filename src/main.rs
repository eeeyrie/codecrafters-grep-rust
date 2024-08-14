use std::env;
use std::io;
use std::process;

enum CharacterClass {
    AnyDigit,
    AnyAlphanumeric,
    LiteralCharacter(character: char),
    PosCharacter(characters: &str),
    NegCharacter(characters: &str),
}

fn parse_pattern(pattern: &str) => Vec<CharacterClass> {
    let pattern_as_enums: Vec<CharacterClass> = Vec::new();
    while let Some(current_char) = pattern.iter().next() {
        pattern_as_enums.push(match current_char {
            '\\' => match pattern.iter().next() {
                Some('d') => CharacterClass::AnyDigit,
                Some('w') => CharacterClass::AnyAlphanumeric,
                _ => continue
            },
            '[' => {
                let mut characters: String = String::new();
                let mut is_positive_class: bool = true;

                match pattern.iter().next() {
                    Some('^') => is_positive_class = false,
                    Some(chara) => characters.push(chara),
                    None => break
                }

                while let Some(current_class_char) = pattern.iter().next() {
                    match current_class_char {
                        ']' => break,
                        _ => characters.push(chara)
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
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    dbg!(parse_pattern(pattern));
    match pattern {
        "\\d" => return input_line.contains(|character: char| character.is_numeric()),
        "\\w" => return input_line.contains(|character: char| character.is_alphanumeric()),
        p if p.chars().count() == 1 => return input_line.contains(pattern),
        p if p.starts_with("[^") && p.ends_with(']') => {
            let trimmed_pattern: &str = &pattern.trim_start_matches('[').trim_end_matches(']')[..];
            return input_line.contains(|character: char| !trimmed_pattern.contains(character))
        },
        p if p.starts_with('[') && p.ends_with(']') => {
            let trimmed_pattern: &str = &pattern.trim_start_matches('[').trim_end_matches(']')[..];
            return input_line.contains(|character: char| trimmed_pattern.contains(character))
        },
        _ => panic!("Unhandled pattern: {}", pattern)
    }
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
