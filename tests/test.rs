use markdownflow::{parse_heading, parse_bold, parse_strikethrough, parse_link};

// HEAD
// ============================================================================
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

// BOLD
// ============================================================================

#[test]
fn test_bold_basic() {
    let input = "**bold text**";
    let expected = "<strong>bold text</strong>";
    assert_eq!(parse_bold(input), expected);
}

#[test]
fn test_bold_single_word() {
    let input = "**important**";
    let expected = "<strong>important</strong>";
    assert_eq!(parse_bold(input), expected);
}

#[test]
fn test_bold_with_numbers() {
    let input = "**123 test**";
    let expected = "<strong>123 test</strong>";
    assert_eq!(parse_bold(input), expected);
}

#[test]
fn test_bold_with_ukrainian() {
    let input = "**жирний текст**";
    let expected = "<strong>жирний текст</strong>";
    assert_eq!(parse_bold(input), expected);
}

#[test]
fn test_bold_with_punctuation() {
    let input = "**Hello, World!**";
    let expected = "<strong>Hello, World!</strong>";
    assert_eq!(parse_bold(input), expected);
}

#[test]
fn test_bold_invalid_single_asterisk() {
    let input = "*not bold*";
    let expected = "*not bold*";
    assert_eq!(parse_bold(input), expected);
}

#[test]
fn test_bold_invalid_empty() {
    let input = "****";
    let expected = "****";
    assert_eq!(parse_bold(input), expected);
}

#[test]
fn test_bold_invalid_no_closing() {
    let input = "**no closing";
    let expected = "**no closing";
    assert_eq!(parse_bold(input), expected);
}

// STRIKETHROUGH
// ============================================================================

#[test]
fn test_strikethrough_basic() {
    let input = "~~deleted text~~";
    let expected = "<del>deleted text</del>";
    assert_eq!(parse_strikethrough(input), expected);
}

#[test]
fn test_strikethrough_single_word() {
    let input = "~~mistake~~";
    let expected = "<del>mistake</del>";
    assert_eq!(parse_strikethrough(input), expected);
}

#[test]
fn test_strikethrough_with_spaces() {
    let input = "~~this is wrong~~";
    let expected = "<del>this is wrong</del>";
    assert_eq!(parse_strikethrough(input), expected);
}

#[test]
fn test_strikethrough_with_ukrainian() {
    let input = "~~закреслений~~";
    let expected = "<del>закреслений</del>";
    assert_eq!(parse_strikethrough(input), expected);
}

#[test]
fn test_strikethrough_with_numbers() {
    let input = "~~version 1.0~~";
    let expected = "<del>version 1.0</del>";
    assert_eq!(parse_strikethrough(input), expected);
}

#[test]
fn test_strikethrough_invalid_single_tilde() {
    let input = "~not strikethrough~";
    let expected = "~not strikethrough~";
    assert_eq!(parse_strikethrough(input), expected);
}

#[test]
fn test_strikethrough_invalid_empty() {
    let input = "~~~~";
    let expected = "~~~~";
    assert_eq!(parse_strikethrough(input), expected);
}

#[test]
fn test_strikethrough_invalid_no_closing() {
    let input = "~~no closing";
    let expected = "~~no closing";
    assert_eq!(parse_strikethrough(input), expected);
}

// LINK
// ============================================================================

#[test]
fn test_link_basic() {
    let input = "[Google](https://google.com)";
    let expected = "<a href=\"https://google.com\">Google</a>";
    assert_eq!(parse_link(input), expected);
}

#[test]
fn test_link_github() {
    let input = "[GitHub Repository](https://github.com/majonory/markdownflow)";
    let expected = "<a href=\"https://github.com/majonory/markdownflow\">GitHub Repository</a>";
    assert_eq!(parse_link(input), expected);
}

#[test]
fn test_link_with_ukrainian() {
    let input = "[Посилання](https://7tv.app/emotes/01JMFCYYDKRH32NZNPYFV5SPGB)";
    let expected = "<a href=\"https://7tv.app/emotes/01JMFCYYDKRH32NZNPYFV5SPGB\">Посилання</a>";
    assert_eq!(parse_link(input), expected);
}

#[test]
fn test_link_with_path() {
    let input = "[Docs](/docs/readme)";
    let expected = "<a href=\"/docs/readme\">Docs</a>";
    assert_eq!(parse_link(input), expected);
}

#[test]
fn test_link_with_query_params() {
    let input = "[Search](https://example.com?q=test&lang=en)";
    let expected = "<a href=\"https://example.com?q=test&lang=en\">Search</a>";
    assert_eq!(parse_link(input), expected);
}

#[test]
fn test_link_with_anchor() {
    let input = "[Section](#section-id)";
    let expected = "<a href=\"#section-id\">Section</a>";
    assert_eq!(parse_link(input), expected);
}

#[test]
fn test_link_invalid_no_url() {
    let input = "[text without url]";
    let expected = "[text without url]";
    assert_eq!(parse_link(input), expected);
}

#[test]
fn test_link_invalid_no_text() {
    let input = "[](https://7tv.app/emotes/01JMFCYYDKRH32NZNPYFV5SPGB)";
    let expected = "[](https://7tv.app/emotes/01JMFCYYDKRH32NZNPYFV5SPGB)";
    assert_eq!(parse_link(input), expected);
}

#[test]
fn test_link_invalid_no_brackets() {
    let input = "(https://7tv.app/emotes/01JMFCYYDKRH32NZNPYFV5SPGB)";
    let expected = "(https://7tv.app/emotes/01JMFCYYDKRH32NZNPYFV5SPGB)";
    assert_eq!(parse_link(input), expected);
}

#[test]
fn test_edge_case_special_characters_link() {
    let link = parse_link(r"[Emoji 🚀](https://7tv.app/emotes/01JMFCYYDKRH32NZNPYFV5SPGB)");
    assert!(link.contains(r"Emoji 🚀"));
    assert_eq!(link, "<a href=\"https://7tv.app/emotes/01JMFCYYDKRH32NZNPYFV5SPGB\">Emoji 🚀</a>");
}
