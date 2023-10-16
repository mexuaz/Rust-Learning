
pub fn div_float(a: f32, b: f32) -> f32 {
    return a / b;
}

fn div_integer(a: i32, b: i32) -> i32 {
    return a / b;
}

pub fn div_fi(a: f32, b: f32) -> i32 {
    return div_integer(a as i32, b as i32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = div_fi(2.3, 2);
        assert_eq!(result, 1);
    }
}