fn main() {
    let mut vec1 = fill_vec(); // 直接调用 fill_vec，无需预先创建向量

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88); // 可以继续向 vec1 中添加元素

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// 修改后的 fill_vec 函数：不接收参数，直接返回新创建的向量
fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new(); // 正确初始化新向量

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec // 返回填充好的向量
}