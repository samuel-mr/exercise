fn main() {
    println!("{}", ex1(10));
    println!("{}", ex1_version_b(10));
    println!("{}", ex1_version_c(15));
}

// Obtener la suma de todos los números que son múltiplos de 3 o 5
// ejem: 10 result: 23
fn ex1(num: i32) -> i32 {
    if num <= 0 {
        return 0;
    }
    let mut i = 0;
    let mut result = 0;
    while i < num {
        if i % 3 == 0 || i % 5 == 0 {
            result = result + i
        }
        i = i + 1;
    }
    result
}

#[test]
fn test_ex1() {
    assert_eq!(ex1(10), 23);
}

fn ex1_version_b(num: i32) -> i32 {
    (1..num).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}
#[test]
fn test_ex1_version_b() {
    assert_eq!(ex1_version_b(10), 23);
}

fn ex1_version_c(num: i32) -> i32 {
    let mut result = 0;
    for i in 1..num + 1 {
        println!("{}", i);
        if i % 3 == 0 || i % 5 == 0 {
            result = result + i;
        }
    }
    result
}

#[test]
fn test_ex1_version_c() {
    assert_eq!(ex1_version_c(10), 233);
}
