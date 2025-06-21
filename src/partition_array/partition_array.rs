pub fn partition_array(mut nums : Vec<i32>, k : i32){
    nums.sort();
    println!("Sorted Array is : {:?}", nums);
  
    let mut count = 0;
    let mut i = 0;
    while i < nums.len() {
        let start = nums[i];
        count+= 1;
        while i < nums.len() && nums[i] - start <= k{
            i += 1;
        }
    };

    println!("{:?}", count);
}