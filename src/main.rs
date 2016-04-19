extern crate regex;

use regex::Regex;

fn main() {
}

struct Token {
    name: &'static str,
    value: String,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut chars = input.chars().collect::<Vec<char>>();
    let mut tokens = Vec::new();
    let mut index = 0;

    let whitespace = Regex::new(r"\s").unwrap();
    let numeral = Regex::new(r"[0-9]").unwrap();
    let letters = Regex::new(r"[a-zA-Z]").unwrap();

    while chars.len() > index {
        let mut current = chars[index];

        if current == '(' {
            tokens.push(Token { name: "paren", value: String::from("(") });
            index += 1;
            continue;
        }

        if current == ')' {
            tokens.push(Token { name: "paren", value: String::from(")") });
            index += 1;
            continue;
        }

        if whitespace.is_match(&current.to_string()) {
            index += 1;
            continue;
        }

        if numeral.is_match(&current.to_string()) {
            let mut values = String::from("");

            while numeral.is_match(&current.to_string()) {
                values.push(current);
                index += 1;
                current = chars[index];
            }

            tokens.push(Token { name: "number", value: values});
            continue;
        }

        if letters.is_match(&current.to_string()) {
            let mut values = String::from("");

            while letters.is_match(&current.to_string()) {
                values.push(current);
                index += 1;
                current = chars[index];
            }

            tokens.push(Token {name: "name", value: values});
            continue;
        }

    }

    return tokens;
}

#[test]
fn it_tokenizes() {
    let stdin = "()".to_string();
    let tokens = tokenize(&stdin);
    assert_eq!(tokens.len(), 2);

    let stdin = "( )".to_string();
    let tokens = tokenize(&stdin);
    assert_eq!(tokens.len(), 2);

    let stdin = "(123)".to_string();
    let tokens = tokenize(&stdin);
    assert_eq!(tokens.len(), 3);

    let stdin = "(abc 123)".to_string();
    let tokens = tokenize(&stdin);
    assert_eq!(tokens.len(), 4);
}
