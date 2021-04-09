mod questions;

// struct Config {
//     user: UserConfig,
// }

pub fn setup() {
    println!("Hello, setup!");
    let answers = questions::questions();

    println!("!Hello {}!", answers.user.username);
    println!("!Password: {}", answers.user.password);
}