fn main() {
    let username = get_username(2);
    // match username {
    //     Some(name) => println!("{name}"),
    //     None => {}
    // }

    if let Some(name) = username {
        println!("{name}");
    }
}

fn get_username(user_id: u32) -> Option<String> {
    // get username from databases
    let query = format!("Get username from users WHERE id = {user_id}");

    let db_result = query_db(query);
    db_result.ok()
}

fn query_db(query: String) -> Result<String, String> {
    if query.is_empty() {
        Err(String::from("query string is empty"))
    } else {
        Ok(String::from("Leo"))
    }
}