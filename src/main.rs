use std::collections::btree_map::Range;

fn main() {
    let tup = (false, 1, 2.0, '3', "4");

    let (zero, one, two, three, four) = tup;

    println!("{} {} {} {} {}", zero, one, two, three, four);
    println!("{}", tup.4);

    // let a: Vec<T> = vec![zero, one, two, three, four];

    let a = ["zero", "one", "two", "three", "four"];

    let mut agen = a.rchunks(2);

    for _ in a {
        println!("{:#?}", agen.next());
    };
}
