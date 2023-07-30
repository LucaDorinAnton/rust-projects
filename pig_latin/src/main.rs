
fn to_pig_latin(s: &String) -> String {
    let vowels = String::from("aeiouAEIOU");

    let mut pig_latin_words: Vec<String> = Vec::new();


    for word in s.split_whitespace() {

        let mut first_cons: Option<char> = None;
        let mut cur_word = String::new();

        for (i, c) in word.chars().enumerate() {
            if i == 0 && !vowels.contains(c) {
                first_cons = Some(c)
            } else {
                cur_word.push(c);
            }
        }

        match first_cons {
            Some(c) => cur_word.push_str(&format!("-{c}ay")),
            None => cur_word.push_str("-hay"),
        }

        pig_latin_words.push(cur_word);
    }


    pig_latin_words.join(" ")
}



fn main() {
    let normal_text = String::from("Hello this is some text");
    let pig_latin_text = to_pig_latin(&normal_text);

    println!("Normal text: {normal_text}");
    println!("In pig Latin: {pig_latin_text}");

}
