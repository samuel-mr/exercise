fn main() {
    let result = fibo(4);
    println!("result: {result}");
}
#[test]
fn fibo_test() {
    assert_eq!(0, fibo(1));
    assert_eq!(1, fibo(2));
    assert_eq!(1, fibo(3));
    assert_eq!(2, fibo(4));
    assert_eq!(3, fibo(5));
    assert_eq!(5, fibo(6));
    assert_eq!(8, fibo(7));
    assert_eq!(13, fibo(8));
}

fn fibo(n: i32) -> i64 {
    // let mut result = 0i64;
    let mut last = 0i64;
    let mut result = 1i64;
    let mut i = 2;
    if n == 1 {
        return 0i64;
    }
    if n == 2 {
        return 1i64;
    }
    while i < n {
        println!("i:{i}: {result}");
        //last = result;
        let temp = result;
        result = result + last;

        last = temp;
        i = i + 1;
    }
    return result;
}
