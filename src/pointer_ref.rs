pub fn run() {
    let arr = [1, 2, 3];
    let arr_2 = arr;
    println!("Values {:?}", (arr, arr_2));

    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;
    println!("Values {:?}", (&vec1, vec2));
}
