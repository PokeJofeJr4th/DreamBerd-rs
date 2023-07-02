use std::iter::Peekable;

use crate::types::prelude::*;

pub fn tokenize(source: &str) -> SResult<Vec<Token>> {
    let mut chars = source.chars().peekable();
    let mut token_stream = Vec::new();
    while chars.peek().is_some() {
        if let Some(tok) = inner_tokenize(&mut chars)? {
            token_stream.push(tok);
        }
    }
    Ok(token_stream)
}

macro_rules! multi_character_pattern {
    ($chars:ident $just:expr; {$($char:expr => $eq:expr),*}) => {
        match $chars.peek() {
            $(Some($char) => {
                $chars.next();
                $eq
            })*
            _ => $just,
        }
    };
}

fn lex_string<T: Iterator<Item = char>>(chars: &mut Peekable<T>, end: char) -> SResult<Token> {
    let mut string_buf = String::new();
    while let Some(next) = chars.next() {
        if next == end {
            break;
        }
        string_buf.push(next);
        if next == '\\' {
            string_buf.push(
                chars
                    .next()
                    .ok_or_else(|| String::from("Unexpected end of file"))?,
            );
        }
    }
    Ok(Token::String(string_buf))
}

fn count_char<T: Iterator<Item = char>, F: Fn(u8) -> Token>(
    chars: &mut Peekable<T>,
    tok: char,
    typ: F,
) -> Token {
    let mut count = 1;
    while chars.peek() == Some(&tok) {
        chars.next();
        count += 1;
    }
    typ(count)
}

fn inner_tokenize<T: Iterator<Item = char>>(chars: &mut Peekable<T>) -> SResult<Option<Token>> {
    let Some(char) = chars.next() else {
        return Err(String::from("Unexpected end of file"))
    };
    Ok(Some(match char {
        '{' => Token::LSquirrely,
        '}' => Token::RSquirrely,
        '(' => Token::LParen,
        ')' => Token::RParen,
        '[' => Token::LSquare,
        ']' => Token::RSquare,
        ';' => Token::Semicolon,
        ':' => Token::Colon,
        '.' => Token::Dot,
        ',' => Token::Comma,
        '&' => Token::And,
        '|' => Token::Or,
        '+' => multi_character_pattern!(chars Token::Plus; {'=' => Token::PlusEq}),
        '-' => {
            multi_character_pattern!(chars Token::Tack; {'=' => Token::TackEq, '>' => Token::Arrow})
        }
        '*' => multi_character_pattern!(chars Token::Star; {'=' => Token::StarEq}),
        '/' => multi_character_pattern!(chars Token::Slash; {'=' => Token::SlashEq}),
        '%' => multi_character_pattern!(chars Token::Percent; {'=' => Token::PercentEq}),
        '<' => multi_character_pattern!(chars Token::LCaret; {'=' => Token::LCaretEq}),
        '>' => multi_character_pattern!(chars Token::RCaret; {'=' => Token::RCaretEq}),
        '"' => lex_string(chars, '"')?,
        '\'' => lex_string(chars, '\'')?,
        '`' => lex_string(chars, '`')?,
        '«' => lex_string(chars, '»')?,
        '=' => count_char(chars, '=', Token::Equal),
        '!' => count_char(chars, '!', Token::Bang),
        '?' => count_char(chars, '?', Token::Question),
        _ => {
            if char.is_whitespace() {
                let mut whitespace_count = 1;
                while let Some(tok) = chars.peek() {
                    if tok.is_whitespace() {
                        // `'\n'` counts as multiple whitespaces
                        whitespace_count += match tok {
                            '\n' => 3,
                            _ => 1,
                        };
                        chars.next();
                    } else {
                        break;
                    }
                }
                Token::Space(whitespace_count)
            } else {
                let mut ident_buf = String::from(char);
                while let Some(next) = chars.peek() {
                    match inner_tokenize(&mut std::iter::once(*next).peekable()) {
                        Ok(Some(Token::Ident(id))) => {
                            ident_buf.push_str(&id);
                            chars.next();
                        }
                        _ => break,
                    }
                }
                Token::Ident(ident_buf)
            }
        }
    }))
}
