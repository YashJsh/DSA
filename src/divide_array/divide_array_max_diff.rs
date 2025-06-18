pub fn divide_array(mut nums: Vec<i32>, k: i32){
    println!("Array is : {:?}", nums);
    nums.sort();
    let n = nums.len();
    let no_arrays = n/3;
    let mut final_array : Vec<Vec<i32>>  = Vec::new();

    println!("Array should be divided into {}", no_arrays);
    println!("Sorted array is : {:?}", nums);

    let mut j = 0;

    for _ in 0..no_arrays{
        if nums[j+2] - nums[j] > k {
            return
        }
        final_array.push(nums[j..j+3].to_vec());
        j += 3;
    }

    println!("Final Array is : {:?}", final_array);
}