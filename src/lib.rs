pub mod parser;

pub use parser::{
    parse_heading,
    parse_bold,
    parse_strikethrough,
    parse_link,
    parse_markdown,
};