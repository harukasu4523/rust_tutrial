fn plus_one(x: i32) -> i32 {
    x + 1;
}

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

// rustの戻り値は関数ブロックの最後の式の値と同義
// セミコロンなしの関数なのでこれ自体が式と見なされる。
// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();
//     println!("The value of x is: {x}");
// }
