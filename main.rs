
fn main() {
    let mut number = 17; //enter the number here
    let mut modulus = 43; //enter the modulus here
    let mut x = modulus;
    //using the euclidean table formula
    let mut quotient; //intializing the quotient
    let mut remainder = 1; //collecting the remainder
    let mut t1 = 0;
    let mut t2 = 1;
    let mut t;
    //the table is complete when the remainder is equals to zero
    while  remainder != 0{
        quotient = modulus / number;//using the formula
        remainder = modulus - (number * quotient);
        t = t1 - (quotient*t2);
        modulus = number;
        number = remainder;
        t1 = t2;
        t2 = t;
    }
    //the solution is negative; to make it possible you add the modulus to it
    x = x + t1;
    println!("The value of x is {}", x);
}
