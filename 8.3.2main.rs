fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut result = String::new();

    for word in text.split_whitespace() {
        let mut chars: Vec<char> = word.chars().collect();
        if vowels.contains(&chars[0]) {
            result.push_str(&format!("{}-hay ", word));
        } else {
            let first_consonant = chars.remove(0);
            result.push_str(&format!("{}-{}ay ", &chars.iter().collect::<String>(), first_consonant));
        }
    }
    result.trim_end().to_string()
}

fn main() {
    let text = "first apple happy";
    let pig_latin_text = pig_latin(text);
    println!("{}", pig_latin_text);
}
