use std::collections::HashMap;

#[derive(Debug, Default)]
struct PasswordManager {
    pub is_locked: bool,
    pub master_password: String,
    pub passwords: HashMap<String, String>,
}

impl PasswordManager {
    fn new(master_pass: String) -> Self {
        Self {
            is_locked: true,
            master_password: master_pass,
            passwords: Default::default(),
        }
    }

    fn lock(&mut self) {
        self.is_locked = true
    }

    fn unlock(&mut self, master_pass: String) {
        if self.master_password == master_pass {
            self.is_locked = false;
        }
    }

    fn list_password(&self) -> Option<HashMap<String, String>> {
        if (!self.is_locked) {
            Some(self.passwords.clone())
        } else {
            None
        }
    }

    fn add_password(&mut self, password: String, username: String) {
        if (!self.is_locked) {
            self.passwords.insert(username, password);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
