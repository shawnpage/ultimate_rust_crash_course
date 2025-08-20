use std::collections::HashMap;

fn main() {

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    let x = v.pop().unwrap();
    println!("second item is {}", v[1]);
    println!("x is {:?}", x);

    let mut v2 = vec![1, 2, 3];
    v2.pop();
    println!("v2 is {:?}", v2);

    let mut h: HashMap<u8, bool> = HashMap::new();
    h.insert(2, true);
    h.insert(4, false);
    h.insert(5, true);
    println!("h is {:?}", h);

    let take_five = h.remove(&5).unwrap();
    println!("take_five is {}", take_five);
}
