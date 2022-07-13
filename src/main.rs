struct Solution;
impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        let mut my_number = num as usize;
        let dict = "0123456789abcdef";
        let mut result:String = String::from("");
        while my_number != 0 {
            let b: u8 = dict.as_bytes()[my_number % 16];
            let c: char = b as char;
            result.push(c);
            my_number /= 16;
        }
        let mut index = 0;
        let mut fin_result:String = String::from("");
        while index < 8 && index < result.chars().count() {
            let b: u8 = result.as_bytes()[index];
            let c: char = b as char;
            fin_result.push(c);
            index += 1;
        }
        fin_result.chars().rev().collect()
    }
}
fn main() {
    let res = Solution::to_hex(-2);
    println!("Result = {res}");
}

