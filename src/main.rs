mod max_pairs;
mod max_difference;
mod max_difference_between_increasing_elements;
mod longest_substring;
mod rotate_array;
mod divide_array;
mod partition_array;
mod max_manhattan;
mod min_deletions;
mod divide_string_group_size_k;

fn main() {
   // let v = vec![1,1,3,4,7,10];
   // let result = max_pairs::min_diff_pair::min_max(v, 2);
   // println!("{}", result);

   let diff = max_difference::max_diff::min_max_diff(990);
   println!("{}", diff);

   let v = vec![9,4,3,2];
   max_difference_between_increasing_elements::max_diff::max_diff(v.clone());
   max_difference_between_increasing_elements::max_diff::maximum_difference(v);

  longest_substring::longest_sub::longest_substr(String::from("pwwkew"));

  let mut nums : Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
  //rotate_array::rotate_array::left_rotate(&mut nums, 3);
  //rotate_array::rotate_array::right_rotate(&mut nums, 5);
  rotate_array::rotate_array::right_rotate_optimize(&mut nums, 4);

  let arr  = vec! [1,3,4,8,7,9,3,5,1];
  divide_array::divide_array_max_diff::divide_array(arr, 2);
  
  let mut nums2 : Vec<i32> = vec![2,2,4,5];
  partition_array::partition_array::partition_array(nums2, 0);


  let manstr = String::from("NWSE");
  max_manhattan::max_manhattan::max_manhattan(manstr, 1);


  let mindel = String::from("aabcaba");
  min_deletions::minimum_deletions::minimum_deletions(mindel, 0);

  let s = String::from("ctoyjrwtngqwt");
  let fill = 'x';
  divide_string_group_size_k::divide_string::divide_string(s, 8, fill);

}
