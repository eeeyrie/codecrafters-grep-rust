use std::env;
use std::io;
use std::process;

#[derive(Debug, PartialEq, Clone)]
enum CharacterClass {
    AnyDigit,
    AnyAlphanumeric,
    StartOfStringAnchor,
    EndOfStringAnchor,
    Wildcard,
    LiteralCharacter(char),
    OneOrMoreCharacters(char),
    ZeroOrOneCharacters(char),
    PosCharacter(String),
    NegCharacter(String),
}

fn parse_pattern<'a>(pattern: &'a str) -> Vec<CharacterClass> {
    let mut pattern_as_enums: Vec<CharacterClass> = Vec::new();
    let mut pattern_iterator = pattern.chars().peekable();
    
    if pattern.chars().next() == Some('^') {
        pattern_as_enums.push(CharacterClass::StartOfStringAnchor);
        pattern_iterator.next();
    }

    while let Some(current_char) = pattern_iterator.next() {
        //dbg!(current_char);
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
            },
            '.' => CharacterClass::Wildcard,
            _ => {
                match pattern_iterator.peek() {
                    Some('+') => {pattern_iterator.next(); CharacterClass::OneOrMoreCharacters(current_char)},
                    Some('?') => {pattern_iterator.next(); CharacterClass::ZeroOrOneCharacters(current_char)},
                    _ => CharacterClass::LiteralCharacter(current_char)
                }
            }
        })
    }

    if pattern_as_enums[pattern_as_enums.len() - 1] == CharacterClass::LiteralCharacter('$') {
        pattern_as_enums.pop();
        pattern_as_enums.push(CharacterClass::EndOfStringAnchor)
    }
    
    return pattern_as_enums
}

fn match_pattern(input_line: &str, pattern: &str, match_from_start: bool) -> bool {
    if pattern.starts_with('(') && pattern.ends_with(')') {
        let trimmed_pattern = pattern.trim_matches('(').trim_end_matches(')');
        for subpattern in trimmed_pattern.split('|') {
            if match_pattern(input_line, subpattern, false) {
                return true
            } else {
                continue
            }
        }
        
        return false
    }

    let parsed_pattern: Vec<CharacterClass> = parse_pattern(pattern);
    let mut pattern_iterator = parsed_pattern.iter().peekable();
    let mut input_iterator = input_line.chars().peekable();
    //let original_iterator_length = pattern_iterator.len();

    if let Some(CharacterClass::StartOfStringAnchor) = pattern_iterator.peek() {
        return match_pattern(input_line, &pattern[1..], true);
    }

    while let Some(current_class) = pattern_iterator.next() {
        dbg!(current_class);
        dbg!(pattern_iterator.len());

        match current_class {
            CharacterClass::EndOfStringAnchor => {
                let (lower_bound, upper_bound) = input_iterator.size_hint();
                dbg!(lower_bound, upper_bound);
                return lower_bound == 0
            },
            CharacterClass::ZeroOrOneCharacters(character) => {
                if input_iterator.peek() == Some(character) {
                    input_iterator.next();
                }
                continue
            },
            _ => {}
        }

        if let Some(current_char) = input_iterator.next() {
            dbg!(current_char);
            let does_character_match: bool = match current_class {
                CharacterClass::AnyDigit => current_char.is_numeric(),
                CharacterClass::AnyAlphanumeric => current_char.is_alphanumeric(),
                CharacterClass::LiteralCharacter(character) => current_char == *character,
                CharacterClass::PosCharacter(characters) => characters.contains(current_char),
                CharacterClass::NegCharacter(characters) => !characters.contains(current_char),
                CharacterClass::StartOfStringAnchor => current_char == '^', // this should not happen
                CharacterClass::Wildcard => true,
                CharacterClass::EndOfStringAnchor => panic!("end of string anchor should have been handled earlier"),
                CharacterClass::OneOrMoreCharacters(character) => {
                    if current_char == *character {
                        while let Some(next_char) = input_iterator.peek() {
                            if next_char == character {input_iterator.next();} else {break}
                        }
                        true
                    } else {false}
                },
                CharacterClass::ZeroOrOneCharacters(_) => panic!("zero or one quantifier should have been handled earlier"),
                _ => panic!("Unhandled character class: {:?}", current_class)
            };
            if !does_character_match && !match_from_start {
                pattern_iterator = parsed_pattern.iter().peekable();
            } else if !does_character_match && match_from_start {
                return false
            }
        } else {
            return false
        }
    }

    match pattern_iterator.next() {
        None => true,
        Some(current_class) => {dbg!(current_class); false}
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
    if match_pattern(&input_line, &pattern, false) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
