pub fn bubble_sort(arr: Vec<i32>) -> Vec<i32> {
    let mut arr = arr;
    let n = arr.len();
    for i in 0..n {
        let mut temp = arr.clone();
        for j in 0..n - 1 - i {
            if temp[j] > temp[j + 1] {
                temp.swap(j, j + 1);
            }
        }
        arr = temp;
    }
    arr
}