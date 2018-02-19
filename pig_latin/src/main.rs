fn pig_split(word: &str) -> (&str, &str) {
    let mut chars = word.chars();
    let first = chars.next();
    if first == Some('a') || first == Some('e') || first == Some('i') || first == Some('o')
        || first == Some('u')
    {
        return ("h", word);
    } else {
        let second = chars.next();
        if second == Some('l') || second == Some('h') || second == Some('b') || second == Some('p')
        {
            return word.split_at(2);
        }
    }
    return word.split_at(1);
}
fn pig_latin(word: &str) -> String {
    let (first, second) = pig_split(word);
    let r = format!("{}-{}ay", second, first);
    r
}
fn main() {
    let v = ["hello", "world", "split", "return", "aline", "ink"];
    for i in v.into_iter() {
        println!("{} : {}", i, pig_latin(i));
    }
}
