fn main(){
    let mut sum = 0;

    for line in std::fs::read_to_string("./src/text.txt").
    unwrap().lines() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let cube_groups = line.split_once(": ").unwrap();
        
        for cubes in cube_groups.1.split("; "){
            for cube in cubes.split(", "){
                let part = cube.split_once(" ").unwrap();
                let n:i32 = part.0.parse().unwrap();
            

                if part.1 == "red" {
                    if red < n {
                        red = n;
                    }
                }else if part.1 == "green"{
                    if green < n {
                        green = n;
                    } 
                }else {
                    if blue < n {
                        blue = n;
                    }
                }
            }
        }

        sum += red*green*blue;

    }

    println!("{}", sum);
}