use markdownflow::parse_heading;

#[test]
fn test_heading_level_1() {
    let input = "# Main Title";
    let expected = "<h1>Main Title</h1>";
    assert_eq!(parse_heading(input), expected);
}

#[test]
fn test_heading_level_2() {
    let input = "## Section Header";
    let expected = "<h2>Section Header</h2>";
    assert_eq!(parse_heading(input), expected);
}

#[test]
fn test_heading_level_3() {
    let input = "### Subsection";
    let expected = "<h3>Subsection</h3>";
    assert_eq!(parse_heading(input), expected);
}

#[test]
fn test_heading_level_4() {
    let input = "#### Minor Heading";
    let expected = "<h4>Minor Heading</h4>";
    assert_eq!(parse_heading(input), expected);
}

#[test]
fn test_heading_level_5() {
    let input = "##### Small Heading";
    let expected = "<h5>Small Heading</h5>";
    assert_eq!(parse_heading(input), expected);
}

#[test]
fn test_heading_level_6() {
    let input = "###### Smallest Heading";
    let expected = "<h6>Smallest Heading</h6>";
    assert_eq!(parse_heading(input), expected);
}

#[test]
fn test_heading_with_special_characters() {
    let input = "# Hello, World! 🌍";
    let expected = "<h1>Hello, World! 🌍</h1>";
    assert_eq!(parse_heading(input), expected);
}

#[test]
fn test_heading_with_numbers() {
    let input = "## Chapter 42: The Answer";
    let expected = "<h2>Chapter 42: The Answer</h2>";
    assert_eq!(parse_heading(input), expected);
}

#[test]
fn test_heading_with_multiple_spaces() {
    let input = "#     Extra    Spaces   Inside";
    let expected = "<h1>Extra    Spaces   Inside</h1>";
    assert_eq!(parse_heading(input), expected);
}

#[test]
fn test_heading_with_trailing_whitespace() {
    let input = "# Title With Spaces   ";
    let expected = "<h1>Title With Spaces</h1>";
    assert_eq!(parse_heading(input), expected);
}

#[test]
fn test_heading_with_leading_whitespace() {
    let input = "   ## Indented Heading";
    let expected = "<h2>Indented Heading</h2>";
    assert_eq!(parse_heading(input), expected);
}

#[test]
fn test_invalid_too_many_hashes() {
    let input = "####### Invalid Level";
    let expected = "####### Invalid Level";
    assert_eq!(parse_heading(input), expected);
}

#[test]
fn test_invalid_no_space_after_hash() {
    let input = "#NoSpace";
    let expected = "<h1>NoSpace</h1>";
    assert_eq!(parse_heading(input), expected);
}

#[test]
fn test_invalid_only_hashes() {
    let input = "###";
    let expected = "###";
    assert_eq!(parse_heading(input), expected);
}

#[test]
fn test_invalid_no_hash() {
    let input = "Just plain text";
    let expected = "Just plain text";
    assert_eq!(parse_heading(input), expected);
}

#[test]
fn test_heading_with_punctuation() {
    let input = "# What is Rust?";
    let expected = "<h1>What is Rust?</h1>";
    assert_eq!(parse_heading(input), expected);
}

#[test]
fn test_heading_with_code_like_text() {
    let input = "## fn main() {}";
    let expected = "<h2>fn main() {}</h2>";
    assert_eq!(parse_heading(input), expected);
}

#[test]
fn test_heading_with_dashes() {
    let input = "### This-Is-A-Heading";
    let expected = "<h3>This-Is-A-Heading</h3>";
    assert_eq!(parse_heading(input), expected);
}

#[test]
fn test_heading_with_underscores() {
    let input = "#### snake_case_heading";
    let expected = "<h4>snake_case_heading</h4>";
    assert_eq!(parse_heading(input), expected);
}

#[test]
fn test_empty_string() {
    let input = "";
    let expected = "";
    assert_eq!(parse_heading(input), expected);
}

#[test]
fn test_only_whitespace() {
    let input = "    ";
    let expected = "    ";
    assert_eq!(parse_heading(input), expected);
}