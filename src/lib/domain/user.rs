mod email;
mod password;

pub use email::UserEmail;
pub use password::UserPassword;
use uuid::Uuid;

pub struct User {
    id: Uuid,
    anonymous: bool,
    email: UserEmail,
    password: UserPassword,
}

impl User {
    pub fn register(id: Uuid, _email: UserEmail, _password: UserPassword) -> Self {
        Self {
            id,
            anonymous: true,
            email: UserEmail::from(""),
            password: UserPassword::from(""),
        }
    }
}

#[cfg(test)]
mod user_tests {
    use uuid::Uuid;

    use super::{User, UserEmail, UserPassword};

    #[test]
    fn register_anonymous_user() {
        let id = Uuid::now_v7();
        let user = User::register(id, UserEmail::from(""), UserPassword::from(""));
        assert_eq!(user.id, id);
        assert_eq!(user.anonymous, true);
        assert_eq!(user.email, UserEmail::from(""));
        assert_eq!(user.password, UserPassword::from(""));
    }
}
