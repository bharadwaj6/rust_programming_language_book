fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..]
}

fn main() {
    let astring = String::from("something else");
    let first_word = first_word(&astring);
    println!("{}", first_word);
    // astring.clear();
}