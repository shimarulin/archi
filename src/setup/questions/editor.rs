use crate::utils::input::answer_option_handler;
use crate::utils::message::format_message;
use crate::utils::render_config::get_render_config;
use inquire::Select;

pub fn get_editor_options() -> Vec<Vec<&'static str>> {
    vec![
        vec!["neovim", "/usr/bin/nvim"],
        vec!["micro", "/usr/bin/micro"],
        vec!["helix", "/usr/bin/helix"],
        vec!["nano", "/usr/bin/nano"],
    ]
}

pub fn get_editor_path(editor: &str) -> &str {
    let editor_options = get_editor_options();
    editor_options
        .iter()
        .find(|opt| opt.first().unwrap() == &editor)
        .unwrap()
        .get(1)
        .unwrap_or(&"/usr/bin/nvim")
}

pub fn select_editor() -> String {
    let select_editor_items = get_editor_options()
        .clone()
        .iter()
        .map(|opt| -> String { opt.first().unwrap().to_string() })
        .collect::<Vec<_>>();

    let render_config = get_render_config();
    answer_option_handler(
        Select::new(&*format_message("Editor"), select_editor_items)
            .with_render_config(render_config)
            .raw_prompt(),
    )
}
