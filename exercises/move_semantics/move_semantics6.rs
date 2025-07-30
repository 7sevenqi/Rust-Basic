fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); // 传递引用

    string_uppercase(data); // 转移所有权
}

// 接收引用，不获取所有权
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// 接收 String，获取所有权
fn string_uppercase(mut data: String) {
    data = data.to_uppercase(); // 修改数据

    println!("{}", data);
}