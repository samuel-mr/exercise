fn main() {
    let result = descending_order(521);
    println!("{result}");
}
#[test]
fn returns_expected() {
    assert_eq!(descending_order(0), 0);
    assert_eq!(descending_order(1), 1);
    assert_eq!(descending_order(15), 51);
    assert_eq!(descending_order(1021), 2110);
    assert_eq!(descending_order(123456789), 987654321);
    assert_eq!(descending_order(145263), 654321);
    assert_eq!(descending_order(1254859723), 9875543221);
}

fn descending_order(x: u64) -> u64 {
    const RADIX: u32 = 10;
    let text = x.to_string();
    let nums = text.chars().map(|x| x.to_digit(RADIX).unwrap());
    let mut result: Vec<u32> = Vec::new();
    for n in nums {
        result.push(n);
    }
    result.sort_unstable();
    result.reverse();
    println!("{:?}", result);
    // let  a = unsafe{text.as_bytes_mut()} ;
    // a.sort();
    // a.reverse();
    // for i in a{
    //     let conv = u32::from(*i);
    //     conv
    //     let r = char::from_u32(conv).unwrap();
    //     let ee = u8::from(r);
    //     println!("{i} - {r}");
    // }
    let texto = result.iter().map(|x| x.to_string()).collect::<String>();
    texto.parse::<u64>().unwrap()
}
