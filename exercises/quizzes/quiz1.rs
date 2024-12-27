// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

// TODO: Write a function that calculates the price of an order of apples given
// the quantity bought.
const APPLE_PRICE: i32 = 2;
fn calculate_price_of_apples(quantity: i32) -> i32 {

    if quantity <= 40 {
        println!("{} apple(s) cost {} rustbuck(s)", quantity, quantity * APPLE_PRICE);
        quantity * APPLE_PRICE

    }  else {
        println!("{} apple(s) cost {} rustbuck(s)", quantity, quantity);
        quantity
    }


 }

fn main() {
    // You can optionally experiment here.
    calculate_price_of_apples(35);   
    calculate_price_of_apples(40);
    calculate_price_of_apples(41);
    calculate_price_of_apples(65);
}


// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
