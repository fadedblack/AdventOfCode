fn main() {
    let contents = std::fs::read_to_string("./input_day1.txt").expect("Error: Couldn't read file");

    let nums : [(String, char);10] = [
        ( "zero".to_string(), '0'),
        ("one".to_string(), '1'),
        ("two".to_string(), '2'),
        ("three".to_string(), '3'),
        ("four".to_string(), '4'),
        ("five".to_string(), '5'),
        ("six".to_string(), '6'),
        ("seven".to_string(), '7'),
        ("eight".to_string(), '8'),
        ("nine".to_string(), '9'),
    ];
    let mut sum = 0;
    let mut num1 = -1;
    let mut num2 = -1;
    let mut str1 = "";
    let mut str2 = "";
    for chars in contents.chars() {
       if chars.is_ascii_digit() {
            if num1 < 0 {
                if let Some(pos) = nums.iter().position(|(word,digit)| *digit==chars) {
                    num1 = (pos as i32);
                    num2 = (pos as i32);
                } 
            }else {
                if let Some(pos) = nums.iter().position(|(word,digit)| *digit==chars) {
                    num2 = (pos as i32);
                } 
            }
        } else {
            if str1 == "" {
                if let Some(pos) = nums.iter().position(|(word,digit)| *word==chars) {
                    num2 = (pos as i32);
                } 
            }
        }
        if chars == '\n' {
            let num = num1*10 + num2;
            sum += num;

            num1 = -1;
            num2 = -1;
            println!("{num}");
        }
    }
    println!("{sum}");
}
