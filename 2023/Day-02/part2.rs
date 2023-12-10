fn main(){
    let mut sum = 0;
    let mut i = 0;

    for line in std::fs::read_to_string("./src/text.txt").
    unwrap().lines() {
        let mut flag = true;
        i += 1;

        let cube_groups = line.split_once(": ").unwrap();
        
        for cubes in cube_groups.1.split("; "){
            for cube in cubes.split(", "){
                let part = cube.split_once(" ").unwrap();
                let n:i32 = part.0.parse().unwrap();
            

                if part.1 == "red" {
                    if n > 12 {
                        flag = false;
                        break;
                    }
                }else if part.1 == "green"{
                    if n > 13 {
                        flag = false;
                        break;
                    } 
                }else {
                    if n > 14 {
                        flag = false;
                        break;
                    }
                }
            }

            if flag == false {
                break;
            }
        }
        
        if flag == true {
            sum += i;
        }

    }

    println!("{}", sum);
}