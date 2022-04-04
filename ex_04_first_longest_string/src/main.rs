use std::str::FromStr;

// Retorna la palabra con mayor cantidad de caracteres 'k'
fn main() {
    let vec = vec!["tree", "foling", "trashy", "blue", "abcdef8", "uvwxyz"];

    let result = longest_consec(vec, 7);
    println!("{result}");
}

fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    let mut mayor: &str = "";
    for i in strarr {
        if i.len()>= k && i.len() > mayor.len() {
            mayor = i;
        }
    }
    let n = String::from_str(mayor).expect("Debe ser string");
    return n;
}
