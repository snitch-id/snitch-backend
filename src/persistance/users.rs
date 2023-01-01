use crate::User;

use crate::model::user::UserID;
use crate::service::authentication::{hash_password, valid_hash};
use argonautica::Hasher;
use std::collections::HashMap;
use std::fmt::Error;

pub struct Users {
    users: HashMap<UserID, User>,
}

impl Users {
    pub fn add_user(&mut self, user: User) -> Result<User, Box<dyn std::error::Error>> {
        let id = self.users.len() as UserID;
        self.users.insert(id, user.clone());
        Ok(user)
    }

    pub fn delete_user(&mut self, user_id: UserID) -> Result<User, Box<dyn std::error::Error>> {
        let user = self.users.remove(&user_id).expect("failed deleting user");
        Ok(user)
    }

    pub fn get_users(&self) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let users = self.users.values().cloned().collect();
        Ok(users)
    }

    pub fn get_user_by_id(&self, user_id: UserID) -> Result<User, Box<dyn std::error::Error>> {
        let user = self.users.get(&user_id);
        let user = match user {
            Some(user) => user,
            None => return Err(Box::try_from(Error).unwrap()),
        };
        Ok(user.clone())
    }

    pub fn get_user_by_name(&self, username: &str) -> Option<&User> {
        let users = self
            .users
            .values()
            .filter(|user| user.username == username)
            .collect::<Vec<&User>>();
        if users.len() != 1 {
            None
        } else {
            Some(users[0])
        }
    }

    pub fn example() -> Self {
        let test_user = User {
            username: "testuser".to_string(),
            password_hash: hash_password("grr"),
        };
        let mut users = Users {
            users: Default::default(),
        };
        users
            .add_user(test_user)
            .expect("Failed setting up example");
        users
    }

    pub fn valid_password(&self, username: &str, password: &str) -> bool {
        let user = self.get_user_by_name(username).unwrap();
        valid_hash(&user.password_hash, password)
    }
}

#[test]
fn test_valid_password() {
    let users = Users::example();
    assert_eq!(users.valid_password("non-existent", "password"), false);
    assert_eq!(users.valid_password("testuser", "password"), false);
    assert_eq!(users.valid_password("testuser", "grr"), true);
}
