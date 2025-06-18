use std::collections::HashMap;
pub fn longest_substr(s : String){
    let mut map: HashMap<char, i32> = HashMap::new();
    let mut left : i32 = 0;
    let mut max_length = i32::MIN;
    let chars : Vec<char> = s.chars().collect();

    for right in 0..chars.len(){
        let current_char = chars[right];

        if let Some(&prev_index) = map.get(&current_char){
            if prev_index >= left{
                left = prev_index+1;
            }
        }

        map.insert(current_char, right as i32); 
        let len  = right as i32 - left + 1;
        if len > max_length{
            max_length = len
        };
    };
    
    
    println!("Map is {:?}", map);
    println!("Max length of substring is : {}", max_length);
}