use crate::utils::file;

fn set_default_editor() {
    let content = "
EDITOR=/usr/bin/nvim
VISUAL=/usr/bin/nvim
";

    file::append("/mnt/etc/environment", &*content);
}

fn set_path() {
    let content = "
# If user ID is greater than or equal to 1000 &
# if ~/.local/bin exists and is a directory &
# if ~/.local/bin is not already in your $PATH
# then export ~/.local/bin to your $PATH.
if [[ $UID -ge 1000 && -d $HOME/.local/bin && -z $(echo $PATH | grep -o $HOME/.local/bin) ]]
then
        export PATH=$HOME/.local/bin:${PATH}
fi
";

    file::append("/mnt/etc/profile", &*content);
}

pub fn setup() {
    set_default_editor();
    set_path();
}
