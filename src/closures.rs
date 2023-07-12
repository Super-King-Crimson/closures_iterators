pub mod type_inferences;
pub mod captures;
pub mod lifetimes;

pub fn explain() {
    println!("A closure is an anonymous function.");
    println!("It can be stored in a variable or passed as an argument to other functions.");
    println!("They capture values from the scope where they were defined.");

    let store_inventory = Inventory {
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Blue,
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Blue,
            ShirtColor::Red,
        ]
    };

    let user_pref1 = None;
    println!("Gave away a {:?} shirt!", store_inventory.giveaway(user_pref1));

    let user_pref2 = Some(ShirtColor::Red);
    println!("Gave away a {:?} shirt!", store_inventory.giveaway(user_pref2));
    
    type_inferences::explain();
    captures::explain();
    lifetimes::explain();
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        //The || marks the beginning of a closure, params would go between the bars
        //The closure calls and returns the result of self.most_stocked if user_preference is None

        //Note how the closure knows about the self variable and the most_stocked method on Inventory
        //Functions we call wouldn't know about self unless we specifically passed it in, but the closure captured it
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}