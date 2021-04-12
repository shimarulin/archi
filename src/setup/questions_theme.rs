use dialoguer::console::style;
use dialoguer::theme::ColorfulTheme;

pub fn get_questions_theme() -> ColorfulTheme {
    ColorfulTheme {
        active_item_prefix: style(">".to_string()).for_stderr().green(),
        prompt_suffix: style(":".to_string()).for_stderr().black().bright(),
        success_prefix: style("»".to_string()).for_stderr().green(),
        success_suffix: style(":".to_string()).for_stderr().black().bright(),
        ..ColorfulTheme::default()
    }
}
