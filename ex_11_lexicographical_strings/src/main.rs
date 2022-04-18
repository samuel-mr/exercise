// Obtener una lista ordenada de palabras que existen dentro de otro grupo de palabras
// ejem: a = ["xyz", "live", "strong"]  b="lively" ... entonces 'live' será valido por existe en 'lively'
fn main() {
    let result = in_array(
        &["xyz", "live", "strong"],
        &["lively", "alive", "harp", "sharp", "armstrong"],
    );
    println!("{:?}", result);
}

fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let xx = arr_a
        .iter()
        .filter(|&a| arr_b.iter().any(|&b| b.contains(a)));

    println!("{:?}", xx);

    result.sort();
    result.dedup();
    return result;
}

fn in_array_version_b(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for b in arr_b {
        let mut res = arr_a
            .iter()
            .filter(|&a| b.contains(a))
            .map(|&m| m.to_string())
            .collect::<Vec<_>>();
        result.append(&mut res);
    }
    result.sort();
    result.dedup();
    return result;
}

fn in_array_version_c(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for b in arr_b {
        for a in arr_a {
            let contained = b.contains(a);
            if contained {
                let new_word = a.to_string();
                if result.contains(&new_word) == false {
                    result.push(new_word);
                }
            }
        }
    }
    result.sort();
    return result;
}

#[test]
fn examples() {
    assert_eq!(
        in_array(
            &["xyz", "live", "strong"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ),
        ["live", "strong"]
    );

    assert_eq!(
        in_array(
            &["live", "strong", "arp"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ),
        ["arp", "live", "strong"]
    );

    assert_eq!(
        in_array(
            &["tarp", "mice", "bull"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ),
        [] as [&str; 0]
    );

    assert_eq!(
        in_array(
            &["live", "strong", "arp", "arp"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ),
        ["arp", "live", "strong"]
    );
}
