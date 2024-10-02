use std::io;

fn main() {
    let mut user_input = String::new();
    
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let mut nums: Vec<i32> = parse_ints_divided_by_whitespace(&user_input);

    insert_sort(&mut nums);
    let result_string = join_nums(&nums);

    println!("{}", result_string);
}

fn join_nums(nums: &Vec<i32>) -> String {
    let mut result_string = String::from(nums[0].to_string());

    for num in &nums[1..] {
        result_string.push(' ');
        result_string += &num.to_string();
    }


    result_string
}

fn parse_ints_divided_by_whitespace(string_with_nums: &String) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::new();

    let splitted = string_with_nums.split_whitespace();

    for el in splitted {
        nums.push(el.parse().expect("Incorrect number"))
    }

    nums
}

fn insert_sort(nums: &mut Vec<i32>) {
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
