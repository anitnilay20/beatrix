pub(crate) fn format_name(name: &str) -> String {
    let mut new_name: String = "".into();

    for (index, char) in name.chars().enumerate() {
        if char.is_uppercase() {
            if index == 0 {
                new_name.push(char.to_ascii_lowercase());
            } else {
                new_name.push_str(&format!("_{}", char.to_ascii_lowercase()));
            }
        } else {
            new_name.push(char);
        }
    }

    new_name
}
