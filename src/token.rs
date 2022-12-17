/**
 * Rust Calculator
 * token.rs
 * (c) 2022 Gabuniku
 */
use std::num;
use std::{f32::NAN, num::ParseFloatError};
use std::{
    io::{BufRead, Write},
    num::ParseIntError,
};
#[derive(Debug)]
enum TOKEN_TYPE {
    NUMBER,              // 数字
    ADDITION,            // 足し算
    SUBTRACTION,         // 引き算
    MULTIPLICATION,      // 掛け算
    DIVISION,            // 割り算
    ROUND_BRACKET_OPEN,  //丸括弧
    ROUND_BRACKET_CLOSE, //丸括弧
    INVALID,             //無効
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
    let mut bracket_deep = 0;
    for (index, token) in tokens.iter().enumerate() {
        let mut p = match &token.token_type {
            TOKEN_TYPE::ADDITION => 1 + bracket_deep,
            TOKEN_TYPE::SUBTRACTION => 1 + bracket_deep,
            TOKEN_TYPE::MULTIPLICATION => 2 + bracket_deep,
            TOKEN_TYPE::DIVISION => 2 + bracket_deep,
            TOKEN_TYPE::ROUND_BRACKET_OPEN => {
                bracket_deep += 2;
                0
            }
            TOKEN_TYPE::ROUND_BRACKET_CLOSE => {
                bracket_deep -= 2;
                0
            }
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

fn remove_bracket(tokens: &mut Vec<Token>) {
    let mut bracket_open: usize = 0;
    let mut bracket_deep: usize = 0;
    let mut remove_index: Vec<usize> = Vec::new();
    // 要らない括弧を列挙
    for (index, token) in tokens.iter().enumerate() {
        if token.text == "(" {
            bracket_open = index;
            bracket_deep += 1;
        }
        if token.text == ")" {
            bracket_deep -= 1;
            // 中身がからか一個なら消す
            if bracket_open + 1 == index || bracket_open + 2 == index {
                remove_index.push(bracket_open);
                remove_index.push(index);
            }
        }
    }
    // 要らない括弧を削除
    remove_index.reverse(); // indexが狂うのを防ぐため後ろから消していく
    for index in remove_index.iter() {
        tokens.remove(*index);
    }
}

fn proc(mut tokens: Vec<Token>) -> Token {
    while tokens.len() != 1 {
        remove_bracket(&mut tokens);
        let prime = get_prime_index(&tokens);
        if prime >= tokens.len() - 1 || prime < 1 {
            break;
        }
        let r = tokens.remove(prime + 1);
        let l = tokens.remove(prime - 1);
        tokens[prime - 1].right = Some(Box::new(r));
        tokens[prime - 1].left = Some(Box::new(l));
        tokens[prime - 1].is_processed = true;
    }
    tokens.remove(0)
}

pub fn execute(text: String) -> Result<f64, ParseFloatError> {
    let mut is_num = false;
    let mut tokens: Vec<Token> = Vec::new();
    let mut num_minus = 1.0;
    let mut temp = String::new();
    for c in text.as_str().chars() {
        if c.is_digit(10) {
            is_num = true;
            temp.push(c);
        } else {
            if is_num {
                is_num = false;
                let num_v = num_minus * temp.parse::<f64>()?;
                tokens.push(Token::pre_init(TOKEN_TYPE::NUMBER, temp.clone(), num_v));
                num_minus = 1.0;
                temp.clear();
            }
            let token_type = match c {
                '+' => TOKEN_TYPE::ADDITION,
                '-' => {
                    if tokens.is_empty() {
                        num_minus *= -1.0;
                        continue;
                    } else {
                        if let Some(t) = tokens.last() {
                            match t.token_type {
                                TOKEN_TYPE::NUMBER => TOKEN_TYPE::SUBTRACTION,
                                TOKEN_TYPE::ROUND_BRACKET_CLOSE => TOKEN_TYPE::SUBTRACTION,
                                _ => {
                                    num_minus *= -1.0;
                                    continue;
                                }
                            }
                        } else {
                            TOKEN_TYPE::SUBTRACTION
                        }
                    }
                }
                '*' => TOKEN_TYPE::MULTIPLICATION,
                '/' => TOKEN_TYPE::DIVISION,
                '(' => TOKEN_TYPE::ROUND_BRACKET_OPEN,
                ')' => TOKEN_TYPE::ROUND_BRACKET_CLOSE,
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
