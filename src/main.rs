mod circular_list;
use circular_list::*;

fn main() {
    let mut l1 = list![1,2,3,4];
    let l2 = l1.clone();

    println!("l1: {}", l1);
    println!("l2: {}", l2);

    println!("deleting l1 first");
    l1.del(0);
    println!("setting cycle");
    l1.save_cycle();
    println!();

    println!("n elements from circular l1:");
    for i in l1.iter().take(12) {
        println!("{}", i);
    }

    println!("l1: {}", l1);
    println!("l2: {}", l2);
}
