use inquire::ui::{Color, RenderConfig, StyleSheet, Styled};

pub fn get_render_config() -> RenderConfig {
    let mut render_config = RenderConfig::default();

    render_config.prompt = StyleSheet::new().with_fg(Color::DarkYellow);
    render_config.scroll_up_prefix = Styled::new("↑").with_fg(Color::LightCyan);
    render_config.scroll_down_prefix = Styled::new("↓").with_fg(Color::LightCyan);

    render_config
}
