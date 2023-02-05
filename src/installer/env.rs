use crate::setup::questions::editor::get_editor_options;
use crate::utils::file;

fn set_default_editor(editor: &str) {
    let editor_options = get_editor_options();
    let editor_path: &&str = editor_options
        .iter()
        .find(|opt| opt.first().unwrap() == &editor)
        .unwrap()
        .get(1)
        .unwrap();
    let content = format!(
        "#!/bin/sh

export EDITOR={}
",
        editor_path
    );

    file::create("/mnt/etc/profile.d/env_editor.sh", &*content);
}

pub fn setup(editor: &str) {
    set_default_editor(&editor);
}
