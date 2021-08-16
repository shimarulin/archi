use console::Style;

pub fn format_message(text: &str) -> String {
    let gray = Style::new().black().bright();
    format!("{:<8} {}", text, gray.apply_to(":"))
}
