use crate::utils::input::answer_option_handler;
use crate::utils::message::format_message;
use crate::utils::render_config::get_render_config;
use inquire::Select;

pub fn select_kernel() -> String {
    let select_kernel_items = vec!["linux", "linux-lts"]
        .iter()
        .map(|s| -> String { s.to_string() })
        .collect::<Vec<_>>();

    let render_config = get_render_config();
    answer_option_handler(
        Select::new(&*format_message("Kernel"), select_kernel_items)
            .with_render_config(render_config)
            .raw_prompt(),
    )
}
