#![allow(unused)]

mod closures;
mod iterators;

fn main() {
    topic::introduce();
    closures::explain();
}

mod topic {
    pub fn introduce() {
        //Learned a couple tricks: hopefully the intro text is a bit better now
        println!("\
----------------------CHAPTER 13: FUNCTIONAL LANGUAGE FEATURES: ITERATORS AND CLOSURES----------------------
- Rust's design has taken inspiration from functional programming, where functions are used as values
- Here, they can be passed as arguments or returned, assigned to variables, and more
- Rust has a couple features that are similar to functional languages, like closures, iterators, enums, and pattern matching
");
    }
}