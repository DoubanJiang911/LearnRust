fn main() {
    let test_cases = vec![
        "first",
        "apple",
        "hello",
        "egg",
        "string",
        "école",    // 法语单词
        "ñandú",    // 西班牙语单词
        "🍎apple",  // 包含表情符号
        "中国",      // 中文字符
    ];
    for &word in test_cases.iter() {
        println!("'{}' -> '{}'", word, to_pig_latin(word));
    }

    fn to_pig_latin(word: &str) -> String {
        if word.is_empty() {
            return String::new();
        }
        let first_char = match word.chars().next() {
            Some(c) => c,
            None => return String::new(),
        };
        if is_vowel(first_char) {
            format!("{}-hay", word)
        } else {
            let mut chars = word.chars();
            let first_char = chars.next().unwrap();
            let rest_of_word: String = chars.collect();
            format!("{}-{}ay", rest_of_word, first_char)
        }
    }
    fn is_vowel(c: char) -> bool {
        matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
    }
    fn sentence_to_pig_latin(sentence: &str) -> String {
        sentence
            .split_whitespace()
            .map(to_pig_latin)
            .collect::<Vec<String>>()
            .join(" ")
    }
}
