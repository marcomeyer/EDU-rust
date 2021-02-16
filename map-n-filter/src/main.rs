
/// This is a documentation header
/// # Docs
///
/// This is a project to test out `rustdoc`.
///
/// [Here is a link!](https://www.rust-lang.org)
///
/// ## Example
/// 
/// ```rust
/// fn foo() -> i32 {
///     1 + 1
/// }
/// ```

fn main() {
    let items = (1..10).into_iter();
    let other_items: Vec<i32> = items
        .filter(|x|{ x % 2 == 0 })
        .map(|x| x + 1)
        .collect();

    for elem in other_items {
        println!("{}", elem);
    }
}
