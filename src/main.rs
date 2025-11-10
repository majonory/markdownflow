use markdownflow::{parse_bold, parse_heading, parse_link, parse_strikethrough};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        show_menu();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "parse" | "1" => {
            if args.len() != 4 {
                println!("Використання: cargo run -- parse test.md output.html");
                return;
            }
            parse_file(&args[2], &args[3]);
        }
        "help" | "2" => show_help(),
        "credits" | "3" => show_credits(),
        _ => println!("Невідома команда. Використайте: parse, help, credits"),
    }
}

fn show_menu() {
    println!("====================================================");
    println!("            MarkdownFlow Parser v0.1.0");
    println!("      MarkdownFlow - Markdown → HTML Конвертер");
    println!("====================================================");
    println!("Оберіть команду:");
    println!();
    println!("  1. parse   - Парсинг файлу (Markdown → HTML)");
    println!("  2. help    - Довідка та інструкції");
    println!("  3. credits - Інформація про автора");
    println!();
    println!("Використання:");
    println!("  cargo run -- <команда> [аргументи]");
    println!();
    println!("Приклади:");
    println!("  cargo run -- parse test.md output.html");
    println!("                 або");
    println!("  cargo run -- 1 test.md output.html");
    println!("====================================================");
}

fn parse_file(input: &str, output: &str) {
    let content = fs::read_to_string(input).expect("Помилка читання файлу");
    let mut html =
        String::from("<!DOCTYPE html>\n<html>\n<head><meta charset=\"UTF-8\"></head>\n<body>\n\n");

    for line in content.lines() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        let mut processed = t.to_string();
        if t.starts_with('#') {
            let h = parse_heading(t);
            if h != t {
                html.push_str(&h);
                html.push('\n');
                continue;
            }
        }

        processed = parse_bold(&processed);
        processed = parse_strikethrough(&processed);
        processed = parse_link(&processed);

        html.push_str("<p>");
        html.push_str(&processed);
        html.push_str("</p>\n");
    }

    html.push_str("\n</body>\n</html>");
    fs::write(output, html).expect("Помилка запису файлу");
    println!("Файл збережено: {}", output);
}

fn show_help() {
    println!("====================================================");
    println!("                      HELP                          ");
    println!("====================================================");
    println!("      MarkdownFlow - Markdown → HTML Конвертер");
    println!("====================================================");
    println!("          ПІДТРИМУВАНІ ЕЛЕМЕНТИ MARKDOWN:");
    println!();
    println!("   • Заголовки:      # до ######");
    println!("   • Жирний текст:   **текст**");
    println!("   • Закреслений:    ~~текст~~");
    println!("   • Посилання:      [текст](url)");
    println!("====================================================");
    println!("               ГРАМАТИЧНІ ПРАВИЛА:");
    println!();
    println!("   Правило 1: HEADING := MARKER+ CONTENT");
    println!("   Правило 2: BOLD := '**' CONTENT '**'");
    println!("   Правило 3: STRIKETHROUGH := '~~' CONTENT '~~'");
    println!("   Правило 4: LINK := '[' TEXT ']' '(' URL ')'");
    println!("====================================================");
}

fn show_credits() {
    println!("====================================================");
    println!("                      CREDITS                       ");
    println!("====================================================");
    println!("      MarkdownFlow - Markdown → HTML Конвертер");
    println!("====================================================");
    println!("   Версія:     v0.1.0");
    println!("   Мова:       Rust ");
    println!();
    println!("====================================================");
    println!("   Автор:");
    println!("   Ім'я:       Тарасенко Тимофій");
    println!("   GitHub:     github.com/majonory");
    println!();
    println!("====================================================");
    println!("   Опис:");
    println!("   Повнофункціональний парсер Markdown → HTML");
    println!("   з підтримкою 4 граматичних правил та CLI");
    println!();
    println!("====================================================");
    println!("   Можливості:");
    println!("   * Парсинг заголовків (h1-h6)");
    println!("   * Форматування тексту (жирний, закреслений)");
    println!("   * Обробка посилань");
    println!("   * CLI інтерфейс");
    println!("====================================================");
}
