fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100; // 使用 y 后，y 的作用域结束
    let z = &mut x; // 此时可以创建新的可变引用
    *z += 1000;
    assert_eq!(x, 1200);
}