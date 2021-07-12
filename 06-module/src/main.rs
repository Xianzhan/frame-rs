mod dir_mod;
mod file_mod;

use dir_mod::{dir_internel::dir_internel_say_hello, dir_say_hello};
use file_mod::file_say_hello;

fn main() {
    file_say_hello();
    dir_say_hello();
    dir_internel_say_hello();
}
