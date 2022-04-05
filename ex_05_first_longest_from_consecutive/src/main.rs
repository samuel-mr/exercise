// Si se contatenan las palabras en grupos de N elementos, obtener el primer elemento más grande en tamaño
fn main() {
    let strarr = vec!["tre1", "tre2", "tre3", "bue", "ef", "yz"];
    for i in strarr.windows(2).map(|x| x.join("")).rev() {
        println!("{}", i);
    }

    // let result = longest_consec(strarr, 2);
    // println!("{result}");
}

#[test]
fn test() {
    assert_eq!(
        longest_consec(vec!["zone", "abigail", "theta", "form", "libe", "zas"], 2),
        "abigailtheta"
    );
    assert_eq!(
        longest_consec(
            vec![
                "ejjjjmmtthh",
                "zxxuueeg",
                "aanlljrrrxx",
                "dqqqaaabbb",
                "oocccffuucccjjjkkkjyyyeehh"
            ],
            1
        ),
        "oocccffuucccjjjkkkjyyyeehh"
    );
    assert_eq!(longest_consec(vec![], 3), "");
    assert_eq!(
        longest_consec(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 3),
        "ixoyx3452zzzzzzzzzzzz"
    );
    assert_eq!(
        longest_consec(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 15),
        ""
    );
    assert_eq!(
        longest_consec(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 0),
        ""
    );
}

#[derive(Debug)]
struct Palabras(String, usize);

fn longest_consec_version_a(strarr: Vec<&str>, k: usize) -> String {
    if strarr.len() == 0 {
        return String::new();
    }
    if k > strarr.len() {
        return String::new();
    }
    if k == 0 {
        return String::new();
    }

    let mut items: Vec<Palabras> = Vec::new();
    for i in 0..=strarr.len() - k {
        let mut owned_string = strarr[i].to_owned();
        let mut j = 1;
        while j < k {
            owned_string.push_str(strarr[i + j]);
            j = j + 1;
        }
        let size = owned_string.len();
        items.push(Palabras(owned_string, size));
    }
    let palabra_max = items.iter().max_by_key(|x| x.1).unwrap();
    let max = palabra_max.1;
    for item in items {
        // println!("{:?}", item);
        if item.1 == max {
            return String::from(item.0);
        }
    }
    return String::from("");
}

fn longest_consec_version_b(strarr: Vec<&str>, k: usize) -> String {
    if strarr.len() == 0 || k > strarr.len() || k == 0 {
        return String::new();
    }
    let mut result = String::new();
    for i in 0..=strarr.len() - k {
        let concatenated = strarr[i..i + k].join("");
        if concatenated.len() > result.len() {
            result = concatenated;
        }
    }
    return result;
}

// Pendiente: .rev parece que era para pasar la prueba porque me parece que 
// esta función no cubre el caso de 2 frases con igual tamaño pero diferentes 
// palabras donde el primero es el que debería retornarse
fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    if strarr.len() == 0 || k > strarr.len() || k == 0 {
        return String::new();
    }

    let result = strarr
        .windows(k)
        .map(|x| x.join(""))
        // .rev()
        .max_by_key(String::len)
        .unwrap_or(String::new());
    return result;
}
