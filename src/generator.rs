
// A task contains the information for the user

use serde::Serialize;

#[derive(Serialize)]
pub struct Task{
    task_num:String,
    result:String,
    from_base: u8,
    to_base:u8
}



pub fn generate_convert(num:u32, from_base:u8, to_base:u8) -> Task{
    
    let task = format_radix(num, from_base as u32);
        
    let result = format_radix(num, to_base as u32);
    
    Task{
        task_num:task,
        result: result,
        from_base: from_base,
        to_base: to_base
    }
}


// https://stackoverflow.com/a/50278316
fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_format_radix() {
        assert_eq!(format_radix(16,16), "10");
        assert_eq!(format_radix(16,10), "16");
        assert_eq!(format_radix(16,2), "10000");
        assert_eq!(format_radix(16,8), "20");
        assert_eq!(format_radix(0xff,16), "ff");
        assert_eq!(format_radix(0xff,10), "255");
        assert_eq!(format_radix(0xff,2), "11111111");
        assert_eq!(format_radix(0xff,8), "377");
        assert_eq!(format_radix(1337,15), "5e2");
    }
}