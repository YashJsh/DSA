//Maximum difference by remapping a digit

pub fn min_max_diff(num: i32) -> i32 {
    let num_str = num.to_string();
    let chars : Vec<char> = num_str.chars().collect();

    let first_char = num_str.chars().next().unwrap();

    let digit_to_replace_max = chars
        .iter()
        .find(|&&c| c != '9')
        .copied();

    let replaced_max: String = chars
        .iter()
        .map(|&c| if Some(c) == digit_to_replace_max { '9' } else { c })
        .collect();


    let replaced_min: String = num_str
        .chars()
        .map(|c| if c == first_char { '0' } else { c })
        .collect();


    let max_num = replaced_max.parse::<i32>().unwrap();
    let min_num = replaced_min.parse::<i32>().unwrap();
    println!("{}", max_num);
    println!("{}", min_num);
    max_num - min_num
}
