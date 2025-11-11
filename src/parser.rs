pub fn parse_heading(input: &str) -> String {
    let trimmed = input.trim();
    let mut level = 0;
    let mut chars = trimmed.chars();

    while let Some(ch) = chars.next() {
        if ch == '#' {
            level += 1;
        } else {
            break;
        }
    }
    if level == 0 || level > 6 {
        return input.to_string();
    }
    let content_start = level;
    if trimmed.len() <= content_start {
        return input.to_string();
    }

    let content = &trimmed[content_start..];
    let content = content.trim_start();
    if content.is_empty() {
        return input.to_string();
    }
    format!("<h{}>{}</h{}>", level, content, level)
}

pub fn parse_bold(input: &str) -> String {
    let mut result = input.to_string();

    while let Some(start) = result.find("**") {
        if let Some(end) = result[start + 2..].find("**") {
            let end_pos = start + 2 + end;
            let content = &result[start + 2..end_pos];

            if content.is_empty() {
                break;
            }

            let replacement = format!("<strong>{}</strong>", content);
            result.replace_range(start..end_pos + 2, &replacement);
        } else {
            break;
        }
    }

    result
}

pub fn parse_strikethrough(input: &str) -> String {
    let mut result = input.to_string();

    while let Some(start) = result.find("~~") {
        if let Some(end) = result[start + 2..].find("~~") {
            let end_pos = start + 2 + end;
            let content = &result[start + 2..end_pos];

            if content.is_empty() {
                break;
            }

            let replacement = format!("<del>{}</del>", content);
            result.replace_range(start..end_pos + 2, &replacement);
        } else {
            break;
        }
    }
    result
}
pub fn parse_link(input: &str) -> String {
    let mut result = input.to_string();

    while let Some(start) = result.find('[') {
        if let Some(close_bracket) = result[start..].find(']') {
            let close_bracket_pos = start + close_bracket;
            let link_text = &result[start + 1..close_bracket_pos];

            if close_bracket_pos + 1 < result.len()
                && &result[close_bracket_pos + 1..close_bracket_pos + 2] == "("
            {
                if let Some(close_paren) = result[close_bracket_pos + 2..].find(')') {
                    let close_paren_pos = close_bracket_pos + 2 + close_paren;
                    let url = &result[close_bracket_pos + 2..close_paren_pos];

                    if !link_text.is_empty() && !url.is_empty() {
                        let replacement = format!("<a href=\"{}\">{}</a>", url, link_text);
                        result.replace_range(start..close_paren_pos + 1, &replacement);
                        continue;
                    }
                }
            }
        }
        break;
    }
    result
}


pub fn parse_image(input: &str) -> String {
    let mut result = input.to_string();

    while let Some(start) = result.find("![") {
        if let Some(close_bracket) = result[start..].find(']') {
            let close_bracket_pos = start + close_bracket;
            let alt_text = &result[start + 2..close_bracket_pos];

            if close_bracket_pos + 1 < result.len()
                && &result[close_bracket_pos + 1..close_bracket_pos + 2] == "("
            {
                if let Some(close_paren) = result[close_bracket_pos + 2..].find(')') {
                    let close_paren_pos = close_bracket_pos + 2 + close_paren;
                    let url = &result[close_bracket_pos + 2..close_paren_pos];

                    if !url.is_empty() {
                        let replacement = format!("<img src=\"{}\" alt=\"{}\">", url, alt_text);
                        result.replace_range(start..close_paren_pos + 1, &replacement);
                        continue;
                    }
                }
            }
        }
        break;
    }
    result
}

pub fn parse_italic(input: &str) -> String {
    let mut result = input.to_string();

    while let Some(start) = result.find('_') {
        if let Some(end) = result[start + 1..].find('_') {
            let end_pos = start + 1 + end;
            let content = &result[start + 1..end_pos];

            if content.is_empty() {
                break;
            }

            let replacement = format!("<em>{}</em>", content);
            result.replace_range(start..end_pos + 1, &replacement);
        } else {
            break;
        }
    }
    result
}

pub fn parse_blockquote(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut result = String::new();
    let mut in_quote = false;
    let mut quote_content = String::new();

    for line in lines {
        let trimmed = line.trim_start();

        if trimmed.starts_with('>') {
            let content = trimmed[1..].trim_start();

            if !in_quote {
                in_quote = true;
                quote_content.clear();
            }

            if !quote_content.is_empty() {
                quote_content.push('\n');
            }
            quote_content.push_str(content);
        } else {
            if in_quote {
                result.push_str(&format!("<blockquote>{}</blockquote>\n", quote_content));
                in_quote = false;
                quote_content.clear();
            }
            result.push_str(line);
            result.push('\n');
        }
    }
    if in_quote {
        result.push_str(&format!("<blockquote>{}</blockquote>\n", quote_content));
    }

    result.trim_end().to_string()
}

pub fn parse_markdown(input: &str) -> String {
    let result = input.to_string();
    result
}
