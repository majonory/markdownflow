use markdownflow::parse_heading;

fn main() {
    println!("MarkdownFlow Parser - Демонстрація");
    println!();
    
    let md1 = "# Головний заголовок";
    let html1 = parse_heading(md1);
    println!("Markdown: {}", md1);
    println!("HTML:     {}", html1);
    println!();
    
    let md2 = "## Розділ документації";
    let html2 = parse_heading(md2);
    println!("Markdown: {}", md2);
    println!("HTML:     {}", html2);
    println!();
    
    let md3 = "### Підрозділ";
    let html3 = parse_heading(md3);
    println!("Markdown: {}", md3);
    println!("HTML:     {}", html3);
    println!();
    
    let md4 = "#### Деталі";
    let html4 = parse_heading(md4);
    println!("Markdown: {}", md4);
    println!("HTML:     {}", html4);
    println!();
    
    let md5 = "##### Примітка";
    let html5 = parse_heading(md5);
    println!("Markdown: {}", md5);
    println!("HTML:     {}", html5);
    println!();
    
    let md6 = "###### Найменший заголовок";
    let html6 = parse_heading(md6);
    println!("Markdown: {}", md6);
    println!("HTML:     {}", html6);
}