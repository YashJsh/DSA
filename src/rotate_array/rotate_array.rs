//189. Rotate Array

pub fn left_rotate(arr: &mut Vec<i32>, k: i32) { //left rotate
    let n = arr.len();
    let mut first = arr[0];

    for _ in 0..k {
        for i in 0..n - 1 {
            arr[i] = arr[i + 1];
        }
        arr[n - 1] = first;
        first = arr[0]
    }
    println!("Array is : {:?}", arr);
}
pub fn right_rotate(arr: &mut Vec<i32>, k: i32) { //right rotate
    let n = arr.len();
    let mut last = arr[n-1];

    for _ in 0..k{
        for i in 0..n-1{
            arr[n-i-1] = arr[n-i-2];
        }
        arr[0] = last;
        last = arr[n-1];
    }
    
    println!("Array is : {:?}", arr);
}

//optimized solution
pub fn right_rotate_optimize(arr: &mut Vec<i32>, k: i32) {
    let n  = arr.len();
    let k = k as usize % n ;

    arr.reverse();
    arr[0..k].reverse();
    arr[k..].reverse();

    println!("Rotated optimized Solution is : {:?}", arr);
}