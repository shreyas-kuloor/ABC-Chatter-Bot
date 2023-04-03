const ONES: [&str; 10] = [
    "0️⃣",
    "1️⃣",
    "2️⃣",
    "3️⃣",
    "4️⃣",
    "5️⃣",
    "6️⃣",
    "7️⃣",
    "8️⃣",
    "9️⃣",
];

pub fn get_unicode_from_number(num: usize) -> Option<String> {
    match num {
        0..=9 => Some(ONES[num].to_string()),
        _ => None
    }
}