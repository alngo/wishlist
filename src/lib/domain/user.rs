mod email;
mod password;
mod repository;
mod service;

pub use email::UserEmail;
pub use password::UserPassword;
pub use repository::*;
pub use service::*;
use uuid::Uuid;

pub struct User {
    id: Uuid,
    anonymous: bool,
    email: UserEmail,
    password: UserPassword,
}

impl User {
    pub fn new(id: Uuid, email: UserEmail, password: UserPassword) -> Self {
        Self {
            id,
            anonymous: email.to_string().is_empty(),
            email,
            password,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn email(&self) -> &UserEmail {
        &self.email
    }

    pub fn password(&self) -> &UserPassword {
        &self.password
    }
}

#[cfg(test)]
mod user_tests {
    use uuid::Uuid;

    use super::{User, UserEmail, UserPassword};

    #[test]
    fn new_user() {
        let id = Uuid::now_v7();
        let user = User::new(id, UserEmail::from(""), UserPassword::from(""));
        assert_eq!(user.id, id);
        assert_eq!(user.anonymous, true);
        assert_eq!(user.email, UserEmail::from(""));
        assert_eq!(user.password, UserPassword::from(""));
    }
}
