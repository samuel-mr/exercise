fn main() {
    let result = perimeter(5);
    println!("{result}");
}
#[test]
fn test_fibonacci_sum() {
    assert_eq!(fibonacci_sum(1), 1);
    assert_eq!(fibonacci_sum(2), 2);
    assert_eq!(fibonacci_sum(3), 4);
    assert_eq!(fibonacci_sum(4), 7);
    assert_eq!(fibonacci_sum(5), 12);
    assert_eq!(fibonacci_sum(6), 20);
}

#[test]
fn test_perimeter(){
    assert_eq!(perimeter(5), 80);
    assert_eq!(perimeter(7), 216);
}

fn perimeter(n: u64) -> u64 {
    4 * fibonacci_sum(n+1)
}
fn fibonacci_sum(n: u64) -> u64 {
    let mut i = 1;
    let mut first: u64 = 0;
    let mut last: u64 = 1;
    let mut result: u64 = 1;

    if n == 1 {
        return 1u64;
    }
    while i < n {
        let temp = last;
        last = first + last;
        first = temp;
        // todos los números de las secuencia de fibonaci los sumarizo
        result = result + last;
        i = i + 1;
    }
    return result;
    // your code
}
