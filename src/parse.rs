use fake::{faker::name::raw::Name, locales::EN, Fake, Faker};

/// Splits a given format string into tokens.
///
/// This function identifies and separates special tokens (indicated by '%')
/// from regular text. It handles cases where tokens are followed by punctuation
/// and/or spaces. Each token or text segment is returned as an element in a vector.
///
/// # Arguments
///
/// * `format` - A string slice that holds the format string to be tokenized.
///
/// # Examples
///
/// ```
/// let tokens = lexer("Hi %name!");
/// assert_eq!(tokens, vec!["Hi ", "%name", "!"]);
/// ```
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

/// Parses a format string and replaces special tokens with fake data.
///
/// This function processes a format string, replacing tokens like '%name' and '%u32'
/// with corresponding fake data generated using the `fake` library. Regular text
/// and punctuation are preserved as is. The function supports handling spaces and
/// punctuation following tokens.
///
/// # Arguments
///
/// * `format` - A string slice that holds the format string to be parsed.
///
/// # Returns
///
/// Returns a `String` that is the parsed and processed version of the input format string.
///
/// # Examples
///
/// ```
/// let greeting = parse("Hi %name!");
/// // The output will be "Hi " followed by a random name and an exclamation mark.
/// ```
pub fn parse(format: &str) -> String {
    let tokens = lexer(format);
    let mut result = String::new();

    for token in tokens {
        if token.starts_with("%") {
            let token_name = token
                .trim_start_matches('%')
                .trim_end_matches(|c: char| c.is_ascii_punctuation());
            match token_name {
                "name" => result.push_str(&Name(EN).fake::<String>()),
                "u32" => result.push_str(&format!("{}", Faker.fake::<u32>())),
                _ => result.push_str(&token),
            }
            let punctuation = token
                .trim_start_matches(|c: char| !c.is_ascii_punctuation())
                .trim_start_matches('%')
                .trim_start_matches(token_name);
            result.push_str(punctuation);
        } else {
            result.push_str(&token);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_simple_text() {
        assert_eq!(lexer("Hello"), vec!["Hello"]);
    }

    #[test]
    fn test_lexer_single_token() {
        assert_eq!(lexer("%name"), vec!["%name"]);
    }

    #[test]
    fn test_lexer_text_with_token() {
        assert_eq!(lexer("Hi %name!"), vec!["Hi ", "%name", "!"]);
    }

    #[test]
    fn test_lexer_multiple_tokens() {
        assert_eq!(lexer("%name%u32"), vec!["%name", "%u32"]);
    }

    #[test]
    fn test_lexer_token_with_punctuation() {
        assert_eq!(lexer("Hello, %name."), vec!["Hello, ", "%name", "."]);
    }

    #[test]
    fn test_parse_simple_text() {
        assert_eq!(parse("Hello"), "Hello");
    }

    #[test]
    fn test_parse_text_with_token() {
        let result = parse("Hi %name!");
        assert!(result.starts_with("Hi "));
        assert!(result.ends_with("!"));
    }

    #[test]
    fn test_parse_multiple_tokens() {
        let result = parse("%name%u32");
        assert!(result.contains(char::is_numeric));
    }

    #[test]
    fn test_parse_token_with_punctuation() {
        let result = parse("Hello, %name.");
        assert!(result.starts_with("Hello, "));
        assert!(result.ends_with("."));
    }

    #[test]
    fn test_parse_no_tokens() {
        assert_eq!(parse("This is a test."), "This is a test.");
    }

    #[test]
    fn test_parse_unknown_token() {
        assert_eq!(
            parse("This is a %unknown token."),
            "This is a %unknown token."
        );
    }
}
