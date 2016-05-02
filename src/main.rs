extern crate regex;

use regex::Regex;

fn main() {
}

struct Token {
    name: &'static str,
    value: String,
}

fn tokenize(input: &str) -> Vec<Token> {
    let chars = input.chars().collect::<Vec<char>>();
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

                if index >= chars.len() {
                    break;
                }
                
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

struct ASTRoot {
    name: &'static str,
    body: Vec<ASTNode>
}

struct ASTNode {
    name: &'static str,
    value: String,
    params: Vec<ASTNode>
}

fn parse(tokens: &Vec<Token>) -> ASTRoot {
    let mut ast = ASTRoot {
        name: "Program",
        body: Vec::new()
    };

    let mut index = 0;

    fn walk(index: &mut usize, tokens: &Vec<Token>) -> ASTNode {
        let token = &tokens[*index];

        if token.name == "number" {
            *index += 1;

            return ASTNode {
                name: "NumberLiteral",
                value: token.value.clone(),
                params: Vec::new()
            }
        }

        if token.name == "paren" && token.value == "(" {

            *index += 1;
            let token = &tokens[*index];

            let mut node = ASTNode {
                name: "CallExpression",
                value: token.value.clone(),
                params: Vec::new()
            };

            *index += 1;
            let mut token = &tokens[*index];

            while !(token.name == "paren" && token.value == ")") {
                node.params.push(walk(index, tokens));
                token = &tokens[*index];
            }

            *index += 1;

            return node;
        }

        panic!("Invalid AST");
    }

    while index < tokens.len() {
        ast.body.push(walk(&mut index, tokens));
    }

    return ast
}

#[test]
fn it_parses() {
    let stdin = "(a 1 234)".to_string();
    let tokens = tokenize(&stdin);
    let ast = parse(&tokens);
    assert_eq!(ast.name, "Program");
    assert_eq!(ast.body[0].name, "CallExpression");
    assert_eq!(ast.body[0].params[0].name, "NumberLiteral");
    assert_eq!(ast.body[0].params[1].name, "NumberLiteral");
}

fn traverse(node: &ASTNode) -> i32 {
    if node.name == "NumberLiteral" {
        return node.value.parse::<i32>().unwrap();
    }

    if node.name == "CallExpression" {

        if node.value == "add" {
            let result = traverse(&node.params[0]) + traverse(&node.params[1]);
            return result;
        }

        if node.value == "subtract" {
            let result = traverse(&node.params[0]) - traverse(&node.params[1]);
            return result;
        }

        if node.value == "multiply" {
            let result = traverse(&node.params[0]) * traverse(&node.params[1]);
            return result;
        }

        if node.value == "divide" {
            let result = traverse(&node.params[0]) / traverse(&node.params[1]);
            return result;
        }
    }

    panic!("Invalid AST!");
}

fn evaluate(ast: ASTRoot) -> i32 {
    let mut result = 0;

    for node in ast.body {
        result += traverse(&node);
    }

    return result;
}

#[test]
fn it_evaluates() {
    let stdin = "(multiply 2 (add 4 6))".to_string();
    let tokens = tokenize(&stdin);
    let ast = parse(&tokens);
    let result = evaluate(ast);

    assert_eq!(result, 20);
}
