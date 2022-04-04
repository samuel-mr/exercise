// Recibe una lista de números e identifica cuáles son los 2 mayores, retorna en orden ascendente
fn main() {
    let array = [1, 5, 87, 45, 8, 8];
    let result = two_oldest_ages_version_b(&array);
    for r in result{
        println!("{r}");
    }
}

fn two_oldest_ages(ages: &[u8]) -> [u8; 2] {
    let mut great_a = 0;
    let mut great_b = 0;
    for age in ages {
        if *age > great_b {
            great_b = *age;
        }
        if great_b > great_a {
            let temp = great_a;
            great_a = great_b;
            great_b = temp;
        }
    }
    return [great_b,great_a];
}

fn two_oldest_ages_version_b(ages: &[u8]) -> [u8; 2] {
    let mut v = ages.to_vec();
    v.sort();
    return [v[v.len()-2], v[v.len()-1] ];
}