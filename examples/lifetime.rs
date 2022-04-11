enum Level {
    Error,
}
struct Logger<'a>(&'a str, Level);
fn configure_logger<T>(_t: T)
where
    T: Send + 'static,
{
    // configure the logger here
}
fn main() {
    let other: &'static str = "12345";
    println!("{:p}", other);
    configure_logger(other);
    Ok::<_,()>(());
}
