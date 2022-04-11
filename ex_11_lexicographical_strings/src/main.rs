fn main() {
    let result = in_array(
        &["xyz", "live", "strong"],
        &["lively", "alive", "harp", "sharp", "armstrong"],
    );
    println!("{:?}", result);
}

fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for b in arr_b {
        for a in arr_a {
            let contained = b.contains(a);
            if contained {
                let ss = a.to_string();
                if result.contains(&ss) == false {
                    result.push(a.to_string());
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
