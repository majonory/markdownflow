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

pub fn parse_markdown(input: &str) -> String {
    let result = input.to_string();
    result
}
