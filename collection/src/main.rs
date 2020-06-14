fn main() {
    //    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    // let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("Third! {}", third),
        None => print!("No third"),
    }
    // panic! at the disco!
    //    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    let first = &v[0];
    println!("The first element is: {}", first);

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }
    println!("After add 50");
    for i in &v {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
