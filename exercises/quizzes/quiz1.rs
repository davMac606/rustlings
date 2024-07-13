
// TODO: Write a function that calculates the price of an order of apples given
// the quantity bought.
fn calculate_price_of_apples(qtd: i32) -> i32 { 
    let mut apple = 2;
    if qtd > 40 {
        apple = 1;
    }
    let price = qtd * apple;
    return price;
}

fn main() {
    // You can optionally experiment here.
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
