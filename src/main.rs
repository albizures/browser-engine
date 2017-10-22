
pub mod dom;
pub mod html;

fn main() {
    println!("{:?}", html::parse("<html><body>Hello, world!</body></html>".to_string()));
}
