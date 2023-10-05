fn main() {
    // Simple
    // for y in 1..10 {
    //     for x in 1..10 {
    //         print!("{:4}", x * y);
    //     }
    //     println!("");
    // }

    for y in 1..10 {
        let s = (1..10)
            .map(|x| format!("{:4}", x * y))
            .collect::<Vec<String>>()
            .join("");
        println!("{}", s);
    }
}
