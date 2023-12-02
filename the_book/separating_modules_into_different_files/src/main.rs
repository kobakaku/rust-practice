pub mod second;
pub mod third;

use second::hello as second_hello;
use third::return_third::hello as third_hello;

fn main() {
    println!("Hello, first!");
    second_hello();
    third_hello();
}
