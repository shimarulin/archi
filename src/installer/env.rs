use crate::utils::file;

fn set_default_editor() {
    let content = "#!/bin/sh

export EDITOR=/usr/bin/nvim
export VISUAL=/usr/bin/nvim
";

    file::create("/mnt/etc/profile.d/env_editor.sh", &*content);
}

pub fn setup() {
    set_default_editor();
}
