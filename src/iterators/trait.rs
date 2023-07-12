pub fn explain() {
    println!("All iterators implement the Iterator trait");
    super::back_to_main();
}

//Here is the definition of Iterator:
pub trait _Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

//The type keyword defines an associated type (we'll talk abt it in chapter 19)
//The Item type will be returned by the iterator

//The iterator trait's only required method is next
