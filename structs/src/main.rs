struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
   let mut user1 = User {
       email: String::from("a@xyz.com"),
       username: String::from("alfon"),
       active: true,
       sign_in_count: 1,
   };
   user1.email = String::from("alfon@xyz.com");
   
   let mut user2 = User {
       email: String::from("0x@xyz.com"),
       ..user1
   };
   user2.username = String::from("alfon2");
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
