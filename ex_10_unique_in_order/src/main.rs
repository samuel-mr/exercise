fn main() {
    let result = unique_in_order([1, 2, 2, 2, 2, 3, 3]);
    println!("{:?}", result);
}

fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut result: Vec<T::Item> = vec![];
    let r = sequence.into_iter().take(1);
    for item in sequence {
        if result.is_empty() {
            result.push(item);
        } else {
            let last = &result[result.len() - 1];
            if item != *last {
                result.push(item);
            }
        }
    }
    return result;
}

fn unique_in_order_version2<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
     let mut result : Vec<_> = sequence.into_iter().collect();
     result.dedup();
     result
}
