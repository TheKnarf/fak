use clap::Parser;
use fake::{Fake, Faker, faker::name::raw::Name, locales::EN};


fn lexer(format: &str) -> Vec<String> {
    let mut tokens = vec![];
    let mut current_token = String::new();
    let mut in_token = false;

    for c in format.chars() {
        if c == '%' {
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
                current_token.clear();
            }
            in_token = true;
            current_token.push(c);
        } else if in_token {
            if c.is_whitespace() || c.is_ascii_punctuation() {
                tokens.push(current_token.clone());
                current_token.clear();
                in_token = false;
            }
            current_token.push(c);
        } else {
            current_token.push(c);
        }
    }

    if in_token || !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

fn parse(format: &str) -> String {
    let tokens = lexer(format);
    let mut result = String::new();

    for token in tokens {
        if token.starts_with("%") {
            let token_name = token.trim_start_matches('%').trim_end_matches(|c: char| c.is_ascii_punctuation());
            match token_name {
                "name" => result.push_str(&Name(EN).fake::<String>()),
                "u32" => result.push_str(&format!("{}", Faker.fake::<u32>())),
                _ => result.push_str(&token),
            }
            let punctuation = token.trim_start_matches(|c: char| !c.is_ascii_punctuation()).trim_start_matches('%').trim_start_matches(token_name);
            result.push_str(punctuation);
        } else {
            result.push_str(&token);
        }
    }

    result
}

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// String format for what to gennerate
    format: String,
}

fn main() {
  let args = Cli::parse();
  println!("{}", parse(&args.format));
}
