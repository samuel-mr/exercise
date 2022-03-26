// retornar si el final del primero parámetro es igual al segundo parámetro
// ejem: "abc" y "bc" => true
fn main() {
    println!("{}", ex2("example", "ple"));
    println!("{}", ex2_version_b("example", "ple"));
}
fn ex2(word: &str, ending: &str) -> bool {
    word.ends_with(ending)
}
#[test]
fn test_ex2() {
    assert_eq!(ex2("abc", "c"), true);
    assert_eq!(ex2("strawberry", "banana"), false);
}


fn ex2_version_b(word: &str, ending: &str) -> bool {
    if word.len() < ending.len() {
        return false;
    }
    let word_end: &str = &word[(word.len() - ending.len()..)];
    word_end == ending
}

#[test]
fn test_ex2_version_b() {
    assert_eq!(ex2_version_b("abc", "c"), true);
    assert_eq!(ex2_version_b("strawberry", "banana"), false);
}
