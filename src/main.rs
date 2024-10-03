use std::io;

fn main() {
    let mut user_input = String::new();
    
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let mut nums: Vec<i16> = parse_ints_divided_by_whitespace(&user_input);

    insert_sort(&mut nums);
    let result_string = nums.iter().map( |&id| id.to_string()).collect::<Vec<String>>().join(" ");

    println!("{}", result_string);
}


fn parse_ints_divided_by_whitespace(string_with_nums: &String) -> Vec<i16> {
    let mut nums: Vec<i16> = Vec::new();

    let splitted = string_with_nums.split_whitespace();

    for el in splitted {
        nums.push(el.parse().expect("Incorrect number"))
    }

    nums
}

fn insert_sort(nums: &mut Vec<i16>) {
    for i in 1..nums.len() {
        let mut j = i;
        let key = nums[i];

        while j > 0 && key < nums[j - 1]  {
            nums[j] = nums[j - 1];

            j -= 1;
        }

        nums[j] = key;
    }
}
