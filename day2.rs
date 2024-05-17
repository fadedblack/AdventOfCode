fn main() {

    let contents = std::fs::read_to_string("./input_day1.txt").expect("Error: Couldn't read file");
    
    let mut line_num = 1;
    let mut sum_id = 0;

    let mut rnum : u8 = 0;
    let mut gnum : u8 = 0;
    let mut bnum : u8 = 0;

    let mut num = "";
    let mut name = "";

    let mut start_pos = 6;
    let mut end_pos   = contents.len();

    let mut flag = false;

    for pos in start_pos..end_pos{
        if contents.chars().nth(pos) == '\n' {
            if flag == false {
                sum_id += line_num;
            }
            line_num += 1;
            pos += 6;
        }
        if contents.chars().nth(pos) == ':' {
            pos = pos + 1;
            while contents.chars().nth(pos) != ';' {
               if contents.chars().nth(pos).is_ascii_digit() {
                    num.push(contents.chars().nth(pos));
                }

                else if contents.chars().nth(pos) != ',' && contents[pos] != ' ' {
                    name.push_str(contents.chars().nth(pos).unwrap().to_string());
                }

                if name == "red" {
                    rnum = num.to_string().parse().unwrap();
                }
                if name == "green" {
                    gnum = num.to_string().parse().unwrap();
                }
                if name == "blue" {
                    bnum = num.to_string().parse().unwrap();
                }

                if rnum <= 12 && gnum <= 13 && bnum <= 14 {
                    flag == false;
                } else {
                    flag == true;
                }
                
                pos += 1;

            }
        } 
    }
    println!("{sum_id}");
}
