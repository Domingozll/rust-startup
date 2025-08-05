use AUTH_SERVICE::Credentials;

fn main() {
    println!("runing RUST-LIB/AUTH_SERVICE...");
    let creds = Credentials {
        username: "letsgetresty".to_owned(),
        password: "password123".to_owned(),
    };
    println!("runing RUST-LIB/AUTH_SERVICE end...");

}