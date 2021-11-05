pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!(">>>으악!")
            }
        }
    }
}

use a::series::of;

fn main() {
    of::nested_modules();
}