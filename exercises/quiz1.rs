// quiz1.rs
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If

// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// Write a function that calculates the price of an order of apples given
// the quantity bought. No hints this time!

// Put your function here!
// the soluation will be simple easy 
// calculate_price_of_apples is going to take param of quantity that data types of i:32 also the
// function will return int type 
// then I'll initial Variable name price also with type of i:32 
// then if-else if- else  
fn calculate_price_of_apples(quantity:i32) -> i32 {
    //define my var
    let mut price:i32 = 0 ;// :i32 is optional by defualt rust will assign to 32 bits
    if quantity <= 40 {
        price = quantity * 2;
    }
    else {
       price = quantity; 

    }
    return price;
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
