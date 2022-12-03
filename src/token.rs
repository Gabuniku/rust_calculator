use std::{f32::NAN, num::ParseFloatError};

#[derive(Debug)]
enum TOKEN_TYPE {
    NUMBER,         // 数字
    ADDITION,       // 足し算
    SUBTRACTION,    // 引き算
    MULTIPLICATION, // 掛け算
    DIVISION,       // 割り算
    PARENTHESES,    //丸括弧
    INVALID,        //無効
}
#[derive(Debug)]
pub struct Token {
    token_type: TOKEN_TYPE,
    text: String,
    left: Option<Box<Token>>,
    right: Option<Box<Token>>,
    value: f64,
    is_processed: bool,
}

impl Token {
    fn new() -> Token {
        Token {
            token_type: TOKEN_TYPE::INVALID,
            text: String::new(),
            left: None,
            right: None,
            value: NAN as f64,
            is_processed: false,
        }
    }

    fn pre_init(type_: TOKEN_TYPE, text: String, value: f64) -> Token {
        Token {
            token_type: type_,
            text: text,
            left: None,
            right: None,
            value: value,
            is_processed: false,
        }
    }

    fn get_value(&self) -> Result<f64, &str> {
        let mut l_v = 0.0;
        let mut r_v = 0.0;
        if let Some(v) = &self.left {
            l_v = v.get_value()?;
        }
        if let Some(v) = &self.right {
            r_v = v.get_value()?;
        }
        match &self.token_type {
            TOKEN_TYPE::NUMBER => Ok(self.value),
            TOKEN_TYPE::ADDITION => Ok(l_v + r_v),
            TOKEN_TYPE::SUBTRACTION => Ok(l_v - r_v),
            TOKEN_TYPE::DIVISION => {
                if r_v == 0.0 {
                    return Err("division by zero");
                }
                Ok(l_v / r_v)
            }
            TOKEN_TYPE::MULTIPLICATION => Ok(l_v * r_v),
            _ => panic!(),
        }
    }
}

fn get_prime_index(tokens: &Vec<Token>) -> usize {
    let mut prime = 0;
    let mut prime_index: usize = 0;
    for (index, token) in tokens.iter().enumerate() {
        let mut p = match &token.token_type {
            TOKEN_TYPE::ADDITION => 1,
            TOKEN_TYPE::SUBTRACTION => 1,
            TOKEN_TYPE::MULTIPLICATION => 2,
            TOKEN_TYPE::DIVISION => 2,
            _ => 0,
        };
        if token.is_processed {
            p = 0;
        }
        if p > prime {
            prime = p;
            prime_index = index;
        }
    }
    prime_index
}

fn proc(mut tokens: Vec<Token>) -> Token {
    while tokens.len() != 1 {
        let prime = get_prime_index(&tokens);
        if prime >= tokens.len()-1 || prime < 1 {
            break;
        }
        let r = tokens.swap_remove(prime + 1);
        let l = tokens.swap_remove(prime - 1);
        tokens[prime - 1].right = Some(Box::new(r));
        tokens[prime - 1].left = Some(Box::new(l));
    }
    tokens.swap_remove(0)
}

pub fn parse(text: String) -> Result<f64, ParseFloatError> {
    let mut is_num = false;
    let mut tokens: Vec<Token> = Vec::new();
    let mut temp = String::new();
    for c in text.as_str().chars() {
        if c.is_digit(10) {
            is_num = true;
            temp.push(c);
        } else {
            if is_num {
                is_num = false;
                let num_v = temp.parse::<f64>()?;
                tokens.push(Token::pre_init(TOKEN_TYPE::NUMBER, temp.clone(), num_v));
                temp.clear();
            }
            let token_type = match c {
                '+' => TOKEN_TYPE::ADDITION,
                '-' => TOKEN_TYPE::SUBTRACTION,
                '*' => TOKEN_TYPE::MULTIPLICATION,
                '/' => TOKEN_TYPE::DIVISION,
                ' ' => continue,
                _ => panic!(),
            };
            tokens.push(Token::pre_init(token_type, String::from(c), 0.0));
        }
    }
    if is_num {
        let num_v = temp.parse::<f64>()?;
        tokens.push(Token::pre_init(TOKEN_TYPE::NUMBER, temp.clone(), num_v));
    }
    let result = proc(tokens).get_value().expect("Error");
    Ok(result)
}
