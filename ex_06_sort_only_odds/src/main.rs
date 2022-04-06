// Ordena solo los números impares
fn main() {
    let result = sort_array(&[5, 8, 6, 3, 4]);
    println!("{:?}", result);
}

#[test]
fn main_test() {
    let result = sort_array(&[5, 8, 6, 3, 4]);
    let expected = vec![3, 8, 6, 5, 4];
    assert_eq!(expected, result);
}
#[test]
fn main_test_2() {
    let result = sort_array(&[1, 5, 2, 8, 3, 4]);
    let expected = vec![1, 3, 2, 8, 5, 4];
    assert_eq!(expected, result);
}

fn sort_array_version_a(arr: &[i32]) -> Vec<i32> {
    let mut result = arr.to_vec();
    for i in 0..result.len() {
        if result[i] % 2 == 0 {
            continue;
        };
        println!("{}", result[i]);
        for j in (i + 1)..result.len() {
            if result[j] % 2 == 0 || i == j {
                continue;
            };
            if result[j] < result[i] {
                let temp = result[i];
                result[i] = result[j];
                result[j] = temp;
            }
        }
    }
    return result;
}

fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut impares_collection: Vec<&i32> = arr.iter().filter(|&x| x % 2 == 1).collect();
    impares_collection.sort();

    // si fuera .iter() cuando lo consumo tendría que hacer 2 asteriscos: **impares.next()
    let mut impares = impares_collection.into_iter();

    arr.iter()
        .map(|&x| {
            if x % 2 == 0 {
                x
            } else {
                *impares.next().unwrap()
            }
        })
        .collect::<Vec<_>>()
}