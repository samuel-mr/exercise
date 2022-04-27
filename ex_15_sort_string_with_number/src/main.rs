use std::str;
// Retorna la frase pero con cada palabra ordenada segun el número que se encuentra dentro de cada palabra
// ejem: "is2 Thi1s T4est 3a"  -->  "Thi1s is2 3a T4est"

// pediente mejorar: https://www.codewars.com/kata/55c45be3b2079eccff00010f/solutions/rust
fn main() {
    let result = order("is2 Thi1s T4est 3a");
    println!("{result}");
}

fn order_no_terminado(sentence: &str) -> String {
    let array = sentence.split(" ").collect::<Vec<_>>();
    let mut array_position: Vec<usize> = Vec::new();
    for i in array {
        for (pos, character) in i.as_bytes().iter().enumerate() {
            if character.is_ascii_digit() {
                array_position.push(pos);
                //println!("{pos}");
            }

            // println!("{current} {}",);
        }
        println!("{i}");
    }
    // println!("{:?}",array);
    return String::from("");
}

fn order(sentence: &str) -> String {
    if sentence.is_empty(){
        return String::from("");
    }
    let mut array = sentence
        .split(" ")
        .map(|f| {
            (
                f.as_bytes()
                    .iter()
                    .enumerate()
                    .find(|&m| m.1.is_ascii_digit())
                    .unwrap()
                    .1, //aqui le digo que quiero el numero el si, no la posición
                f,
            )
        })
        .collect::<Vec<_>>();

    array.sort_by_key(|x| x.0);
    println!("{:?}", array);
    let result = array.iter().map(|m| m.1).collect::<Vec<_>>().join(" ");
     result
}
