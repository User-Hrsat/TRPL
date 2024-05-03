fn main() {
    // let tup: [u8; 3] = if true {
    //     let tup_inner: [u8; 3] = [1, 2, 3];
    //     tup_inner
    // } else {
    //     return;
    // };

    // // let mut couinter: u8 = 0;
    // // let round: u8 = loop {
    // //     if couinter < tup.len() as u8 {
    // //         println!("{}", tup[couinter as usize]);
    // //         couinter += 1;
    // //     } else {
    // //         break couinter;
    // //     }
    // // };

    // // println!("Loop for Round {}", round);

    // for element in tup {
    //     println!("{element}");
    // }
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}
