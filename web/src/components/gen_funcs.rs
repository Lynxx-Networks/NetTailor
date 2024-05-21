
use web_sys::Url;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};


pub fn format_date(date_str: &str) -> String {
    let date = chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S")
        .unwrap_or_else(|_| chrono::DateTime::<chrono::Utc>::from_timestamp(0, 0).unwrap().naive_utc()); // Fallback for parsing error
    date.format("%m-%d-%Y").to_string()
}



pub fn encode_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(password_hash)
}


pub fn validate_user_input(username: &str, password: &str, email: &str) -> Result<(), String> {
    if username.len() < 4 {
        return Err("Username must be at least 4 characters long".to_string());
    }

    if password.len() < 6 {
        return Err("Password must be at least 6 characters long".to_string());
    }

    let email_regex = regex::Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$").unwrap();
    if !email_regex.is_match(email) {
        return Err("Email is not in a valid format".to_string());
    }

    Ok(())
}

pub fn get_base_url() -> Result<String, &'static str> {
    let window = web_sys::window().ok_or("No global `window` exists")?;
    let href = window.location().href().map_err(|_| "Failed to retrieve the href")?;

    // Create a new URL object from the href
    let url = Url::new(&href).map_err(|_| "Failed to construct URL object")?;

    // Construct the base URL using the protocol and host
    let base_url = format!("{}//{}", url.protocol(), url.host());
    Ok(base_url)
}