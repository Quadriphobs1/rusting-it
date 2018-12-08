fn main() {
    println!("Hello, world!");

    let mut array = vec![4, 1, 3, 7, 1];

    for j in 1..array.len() {
        let key = array[j];
        let mut i = j - 1;
        while i > 0 {
            println!("{}", array[i]);
            array.swap(i, i + 1);
            println!("{:?}", array);
            i = i - 1;
        }
        array[i + 1] = key;
    }

    let mut array2 = vec![3, 2, 4, 5, 2, 4, 5, 2, 4, 5, 2, 5];
    for i in 0..array2.len() {
        for j in (0..i).rev() {
            if array2[j] >= array2[j + 1] {
                array2.swap(j, j + 1);
            } else {
                break;
            }
        }
    }
    assert_eq!(array, [1, 1, 3, 4, 7]);
    assert_eq!(array2, [2, 2, 2, 2, 3, 4, 4, 4, 5, 5, 5, 5]);
}
