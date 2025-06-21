use std::{cmp::{max, min}};

pub fn max_manhattan(s: String, k: i32){
    let mut maxMD = 0;
    let mut north = 0;
    let mut south = 0;
    let mut east = 0;
    let mut west = 0;

    for (i,c) in s.chars().enumerate(){
        if c == 'E'{east+=1}
        else if c == 'W'{west+=1}
        else if c == 'N'{north+=1}
        else {south+=1}

        let currentMD = i32::abs(east - west) + i32::abs(north - south);

        let steps = i as i32 +1;
        let wasted = steps - currentMD; 

        let mut extra = 0;
        if wasted!=0 {
            extra = min(2*k, wasted)
        }
        let final_curr_MD = currentMD + extra;

        maxMD = max(maxMD, final_curr_MD)
    }
    println!("MAX manhattan distance is : {}", maxMD);
}