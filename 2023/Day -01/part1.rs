use std::fs;

fn main(){
    let mut sum = 0;

    for line in fs::read_to_string("./src/text.txt")
    .unwrap().lines(){
        
        let mut digit = 0;
        let n = line.len();

        for ch in line.chars(){
            if ch >= '0' && ch <= '9' {
                digit = ((ch as i32) - 48)*10;
                break;
            }
        }

        for ch in line.chars().rev(){
            if ch >= '0' && ch <= '9' {
                digit += (ch as i32) - 48;
                break;
            }
        }
        
        sum += digit;
    }

    println!("{}", sum);
}