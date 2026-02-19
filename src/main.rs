// main.rs
use std::error::Error;
use std::fmt;

// Define a custom error type
#[derive(Debug)]
enum MarkdownError {
    InvalidHeader,
    InvalidBold,
    InvalidItalic,
    InvalidStrikethrough,
}

impl Error for MarkdownError {}

impl fmt::Display for MarkdownError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MarkdownError::InvalidHeader => write!(f, "Invalid header"),
            MarkdownError::InvalidBold => write!(f, "Invalid bold"),
            MarkdownError::InvalidItalic => write!(f, "Invalid italic"),
            MarkdownError::InvalidStrikethrough => write!(f, "Invalid strikethrough"),
        }
    }
}

// Define a struct to represent markdown elements
#[derive(Debug)]
enum MarkdownElement {
    Header(String, u32),
    Bold(String),
    Italic(String),
    Strikethrough(String),
    Text(String),
}

// Define a parser
struct MarkdownParser;

impl MarkdownParser {
    // Parse a markdown string into a vector of markdown elements
    fn parse(markdown: &str) -> Result<Vec<MarkdownElement>, MarkdownError> {
        let mut elements = Vec::new();
        let mut lines = markdown.lines();

        while let Some(line) = lines.next() {
            // Check if the line is a header
            if line.starts_with('#') {
                let mut header_level = 0;
                let mut header_text = String::new();

                for c in line.chars() {
                    if c == '#' {
                        header_level += 1;
                    } else {
                        header_text.push(c);
                    }
                }

                // Check if the header level is valid
                if header_level > 6 {
                    return Err(MarkdownError::InvalidHeader);
                }

                elements.push(MarkdownElement::Header(header_text.trim().to_string(), header_level));
            } else {
                // Split the line into words
                let words: Vec<&str> = line.split_whitespace().collect();

                for word in words {
                    // Check if the word is bold
                    if word.starts_with("**") && word.ends_with("**") {
                        elements.push(MarkdownElement::Bold(word.trim_matches('*').to_string()));
                    } else if word.starts_with('*') && word.ends_with('*') {
                        // Check if the word is italic
                        elements.push(MarkdownElement::Italic(word.trim_matches('*').to_string()));
                    } else if word.starts_with('~') && word.ends_with('~') {
                        // Check if the word is strikethrough
                        elements.push(MarkdownElement::Strikethrough(word.trim_matches('~').to_string()));
                    } else {
                        // If the word is not bold, italic, or strikethrough, it's plain text
                        elements.push(MarkdownElement::Text(word.to_string()));
                    }
                }
            }
        }

        Ok(elements)
    }
}

fn main() {
    let markdown = "# Hello World\n**This is bold**\n*This is italic*\n~This is strikethrough~";
    let elements = MarkdownParser::parse(markdown);

    match elements {
        Ok(elements) => {
            for element in elements {
                match element {
                    MarkdownElement::Header(text, level) => println!("Header level {}: {}", level, text),
                    MarkdownElement::Bold(text) => println!("Bold: {}", text),
                    MarkdownElement::Italic(text) => println!("Italic: {}", text),
                    MarkdownElement::Strikethrough(text) => println!("Strikethrough: {}", text),
                    MarkdownElement::Text(text) => println!("Text: {}", text),
                }
            }
        },
        Err(err) => eprintln!("Error: {}", err),
    }
}