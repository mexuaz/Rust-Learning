// This is a comment in Rust

const WEEK_DAYS: i32 = 7;

fn main() {
    let days_year = 365i32;

    println!("There are {} days in one year.", days_year);

    println!("Also There are {weeks} weeks \
        and {hours} hours in one year.",
        weeks=days_year/WEEK_DAYS,
        hours=days_year*24);


    let mut x: i32 = 3;
    const LB2KG: f32 = 0.45359237;

    println!("{0} pounds is {1} kilograms", x, x as f32 * LB2KG);

    // Shadowing x variable
    let x: f64 = 4.3945834;
    println!("{0} pounds is {1} kilograms", x, x * LB2KG as f64);
}
