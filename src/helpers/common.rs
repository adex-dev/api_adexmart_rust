use chrono::Local;
use regex::Regex;

//format : 2026-05-07 14:30:00

pub fn time_now_modif()->String{
    Local.now().format("%Y-%m-%d %H:%M:%S").to_string()
}

// trim + lower

pub fn convert_to_normalize_lower(value:&str)->String{
    value.trim().to_lowercase()
}

// check empty string

pub fn is_empty(value:&str)->bool{
    value.trim().is_empty()
}


/// Check not empty string
pub fn not_empty(value: &str) -> bool {
    !value.trim().is_empty()
}

/// Valid name:
/// a-z A-Z 0-9 . space
pub fn is_valid_name(name: &str) -> bool {
    let regex = Regex::new(r"^[a-zA-Z0-9.\s]+$").unwrap();
    regex.is_match(name)
}

/// Validate email
pub fn is_valid_email(email: &str) -> bool {
    let regex =
        Regex::new(r"^[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}$").unwrap();

    regex.is_match(email)
}

/// Validate price
/// only number + dot + space
pub fn is_valid_price(price: &str) -> bool {
    let regex = Regex::new(r"^[0-9.\s]+$").unwrap();
    regex.is_match(price)
}

/// Example:
/// 1 => 000001
pub fn sequence_number(n: i32) -> String {
    format!("{:06}", n)
}

/// Example:
/// INV-2605000001
pub fn prefix_module(code: &str, seq: i32) -> String {
    let date = Local::now().format("%y%m").to_string();

    format!("{}-{}{}", code, date, sequence_number(seq))
}