use std::collections::HashMap;

fn pow(base:i32, exp:i32) -> i32 {
    let mut sum = 1; 
    if exp == -1 {
        return 0;
    }
    for i in 0..exp {
        sum *= base;
    }
    sum
}
fn main(){
    let mut sum = 0;

    for line in std::fs::read_to_string("input.txt")
    .unwrap()
    .lines() {
        
        let mut map:HashMap<&str, i32> = HashMap::new();
        let mut count = 0;
        
        let games = line.split_once(": ").unwrap();
        let part = games.1.split_once(" | ").unwrap();

        for i in part.0.split_whitespace() {
            map.insert(i, 1);
        }

        for i in part.1.split_whitespace() {
            if map.contains_key(i) {
                count += 1;
            }
        }
      
        sum += pow(2, count-1);
    }

    println!("{}", sum);

}
