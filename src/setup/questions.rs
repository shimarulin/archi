mod user;

pub struct Answers {
    pub user: user::User,
}

pub fn questions() -> Answers {
    Answers {
        user: user::setup_user()
    }
}