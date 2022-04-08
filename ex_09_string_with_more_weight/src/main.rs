// Se separa cada palabra y se calcula la suma de cada letra (el peso de cada letra aumenta en orden alfabético)
// finalmente se obtiene la palabra con mayor puntaje
fn main() {
    let result = high("man i need a taxi up to ubud");
    println!("{result}");
}

#[test]
fn main_test() {
    assert_eq!(high("man i need a taxi up to ubud"), "taxi");
    assert_eq!(high("what time are we climbing up the volcano"), "volcano");
    assert_eq!(high("take me to semynak"), "semynak");
    assert_eq!(high("massage yes massage yes massage"), "massage");
    assert_eq!(high("take two bintang and a dance please"), "bintang");
    assert_eq!(high("aa b"), "aa");
    assert_eq!(high("b aa"), "b");
    assert_eq!(high("bb d"), "bb");
    assert_eq!(high("d bb"), "d");
    assert_eq!(high("aaa b"), "aaa");
}

fn high_version1(input: &str) -> &str {
    //let frase = "hola como estas";
    let result = input
        .split(" ")
        .map(|x| {
            (
                x,
                x.as_bytes().iter().map(|&y| (y - 96) as u16).sum::<u16>(),
            )
        })
        .collect::<Vec<_>>();

    // let result2 = frase2.split(" ");

    let max = result.iter().max_by_key(|&x| x.1).unwrap();
    let mut grupo = result.iter().filter(|x| x.1 == max.1).take(1);
    // let max2 = result2.max().unwrap();
    // //let first = result.filter(|&x| x == max).next();
    return grupo.next().unwrap().0;
}

// Esta me funciona porq el 'split_ascii_whitespace' me retorna un iterator, si fuera 'split' no puedo hacer un rev()
fn high(input: &str) -> &str {
    let result = input
        .split_ascii_whitespace()
        .rev()
        .max_by_key(|&x| x.as_bytes().iter().map(|&y| y - 96).sum::<u8>());
    return result.unwrap();
}
