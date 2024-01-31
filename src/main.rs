use clap::Parser;
use fake::{Fake, Faker, faker::name::raw::Name, locales::EN};

fn lexer(format: &str) -> Vec<String> {
    let mut tokens = vec![];
    let mut current_token = String::new();
    let mut in_token = false;

    for c in format.chars() {
        if c == '%' {
            if in_token {
                tokens.push(current_token.clone());
                current_token.clear();
            }
            in_token = !in_token;
        } else if in_token {
            current_token.push(c);
        } else {
            tokens.push(c.to_string());
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

fn parse(format: &str) -> String {
    let tokens = lexer(format);
    let mut result = String::new();

    for token in tokens {
        match token.as_str() {
            "%name" => result.push_str(&Name(EN).fake::<String>()),
            "%u32" => result.push_str(&format!("{}", Faker.fake::<u32>())),
            _ => result.push_str(&token),
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
