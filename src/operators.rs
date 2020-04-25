/**
 * Operators in rust
 *  +, - , *, /, %, >, <, >=, <=, ==, =, &&, ||, !
 */
pub fn run() {
    println!(" Addition 4 + 4 = {}", 4 + 4);
    println!(" Subtraction 8 - 2 = {}", 8 - 2);
    println!(" Multiplication 2 * 2 = {}", 2 * 2);
    println!(" Division 10 / 2 = {}", 10 / 2);
    println!(" Modulo 11 % 5 = {}", 11 % 5);
    println!(" Less than 12 < 15 = {}", 12 < 15);
    println!(" Greater than 12 > 15 = {}", 12 > 15);

    // testing floating-point with integer types
    // println!("The value of 4.2 - 4  = {}", 4.2 - 4); // types mis-match error

    println!(" Greater than equal to 15 >= 15 = {}", 15 >= 15);
    println!(" Less than equal to 15 <= 15 = {}", 15 <= 15);
    println!(" Equality 15 == 15 = {}", 15 == 15);
    println!(" && operators true && false = {}", true && false);
    println!(" || operators true || false = {}", true || false);
    // println!(" | operators true | false = {}", true | false); // understand about bitwise operators
    println!(" Not operators !true = {}", !2);
}
