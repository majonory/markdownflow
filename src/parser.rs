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

