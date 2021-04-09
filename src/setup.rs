mod questions;

// struct Config {
//     user: UserConfig,
// }

pub fn setup() {
    let answers = questions::ask_questions();

    println!("!Hello {}!", answers.user.username);
    println!("!Password: {}", answers.user.password);
}