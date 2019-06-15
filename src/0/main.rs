/*
 * Rustのif let。
 * CreatedAt: 2019-06-16
 */
fn main() {
    let v1 = Some(10);
    match v1 {
        Some(10) => println!("ten!! {:?}", v1),
        _ => (),
    }

    // 略記
    let v2 = Some(10);
//    let v2 = None;
    if let Some(10) = v2 { println!("ten!! {:?}", v2); }
    if let Some(11) = v2 { println!("eleven! {:?}", v2); } else {println!("not eleven {:?}", v2);}

    // ifのほうが簡単……
    let v3 = Some(10);
//    let v3 = None;
    if Some(10) == v3 { println!("ten!! {:?}", v3); }
}

