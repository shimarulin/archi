mod setup;

fn main() {
    let config = setup::setup();

    println!("username is {}", config.answers.user.username);
    println!("password is {}", config.answers.user.password);
}
