// Es un array que contiene tuplas, donde en la tupla, el primer elemento es la cantidad de personas que suben al bus y el segundo es la cantidad de personas que bajan del bus
// Debe determinar cuántas personas quedaron dormitas en la última parada
fn main() {
    let result =number(&[(10,0),(3,5),(5,8)]);
    println!("{}",result);
}

#[test]
fn returns_expected() {
  assert_eq!(number(&[(10,0),(3,5),(5,8)]), 5);
  assert_eq!(number(&[(3,0),(9,1),(4,10),(12,2),(6,1),(7,10)]), 17);
  assert_eq!(number(&[(3,0),(9,1),(4,8),(12,2),(6,1),(7,8)]), 21);
}
fn number(bus_stops:&[(i32,i32)]) -> i32 {
    bus_stops.iter().fold(0, |acc, x| acc + x.0 - x.1)    
}
fn number_version_2(bus_stops:&[(i32,i32)]) -> i32 {
    return bus_stops.iter().map(|f| f.0 - f.1).sum();
}
fn number_version_1(bus_stops:&[(i32,i32)]) -> i32 {
    let mut current = 0;
    for  i in bus_stops{
        current = current + i.0 - i.1;
    }
    return current;
}