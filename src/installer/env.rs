use crate::setup::questions::editor::get_editor_path;
use crate::utils::file;

fn set_default_editor(editor: &str) {
    let editor_path = get_editor_path(&editor);
    let content = format!(
        "#!/bin/sh

export EDITOR={0}
export VISUAL={0}
",
        editor_path
    );

    file::create("/mnt/etc/profile.d/env_editor.sh", &*content);
}

pub fn setup(editor: &str) {
    set_default_editor(&editor);
}
