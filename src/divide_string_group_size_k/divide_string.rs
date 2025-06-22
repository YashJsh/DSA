//ctoyjrwtngqwt
pub fn divide_string(s: String, k: i32, fill: char) {
    let chars : Vec<char> = s.chars().collect();
    let n = chars.len();

    let mut strings: Vec<String> = Vec::new();
    let elements_to_fill = k - n as i32 % k;

    println!("Elements to fill {}", elements_to_fill);

    let mut i = 0;
    while i + k <= n as i32 {
        let substring: String = chars[i as usize..(i+k) as usize].iter().collect();
        strings.push(substring);
        i += k;
    }

    if n % k as usize == 0 {
        println!("{:?}", strings);
        return;
    }

    println!("Value of i is ; {}", i);
    let mut j : usize = 0;
    let mut last_str = String::new();
    while j < k as usize{
        if let Some(ch) = chars.get(i as usize){
            last_str.insert(j, *ch);
        } else{
            last_str.insert(j, fill);
        }
        i += 1;
        j += 1;
    }
    strings.push(last_str);

    println!("{:?}", strings);
}
