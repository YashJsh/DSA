use std::{cmp::max, collections::HashMap, i32};

pub fn minimum_deletions(word : String, k : i32){
    let mut hashmap:HashMap<char, i32> = HashMap::new();
    for i in word.chars(){
        println!("Char is : {}", i);
       *hashmap.entry(i).or_insert(0)+=1;
       println!("Hashmap: {:?}", hashmap);
    }
    println!("{:?}", hashmap);
    let mut vec : Vec<i32> = Vec::new();
    for (c,i) in hashmap{
        vec.push(i);
    }
    
    vec.sort();
    println!("{:?}", vec);
    let mut min_deletions = i32::MAX;

    for i in 0..vec.len(){
        let base = vec[i];
        let mut deletions = 0;

        for &f in &vec{
            if f < base {
                deletions += f;
            } else if f > base + k{
                deletions += f - (base + k);
            }
        }

        min_deletions = min_deletions.min(deletions);
    }
    
    println!("{:?}", vec);

}