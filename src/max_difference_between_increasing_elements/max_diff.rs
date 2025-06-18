// Now let's solve like with time Complexity with  O(n)

// 💡 Intuition Recap:
// We're not trying every pair
// We're asking:
// "What's the best number I could have seen before this one to subtract from it?"
// That’s min_val
// If current number is bigger than min_val, it might give us a better answer!

// you can also understand with like best time to buy stocks, when the stock is lower price as compared to today's price

pub fn max_diff(nums: Vec<i32>){
    let mut max_diff = -1;
    let mut min_value = nums[0];
    
    for i in 1..nums.len(){
        if nums[i] > min_value{  //like best price to buy
            let diff = nums[i] - min_value;
            if max_diff < diff{
                max_diff = diff;
            }
        }else{ // can buy lower as well 
            min_value = nums[i];
        }
    }
    println!("Maximum difference is : {}", max_diff);
}


pub fn maximum_difference(nums: Vec<i32>) { //Time Comp = O(n**2)
    let mut max_diff = -1;
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            if nums[j] > nums[i] {
                let diff = nums[j] - nums[i];
                if diff > max_diff {
                    max_diff = diff;
                }
            }
        }
    }
    println!("Max Difference is {}", max_diff);
}
