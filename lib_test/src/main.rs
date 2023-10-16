fn main() {
    println!("{} / {} = {}", 32.2, 4.1, calc::div_float(32.2, 4.1));
    println!("{} / {} = {}", 32.2, 4.1, calc::div_fi(32.2, 4.1));
}
