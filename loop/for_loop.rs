pub fn for_loop() -> String {
    for i in 0..5 {
        println!("i: {i}");
    }
    let arr = [10, 20, 30, 40, 50];
    for (index, value) in arr.iter().enumerate() {
        println!("index: {index}, value: {value}");
    }
    "for_loop executed".into()
}