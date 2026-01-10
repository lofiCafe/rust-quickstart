fn main() {
    let arr = [10, 20, 30, 40, 50];
    let slice = &arr[1..4]; // This creates a slice containing elements at index 1, 2, and 3

    println!("Slice: {:?}", slice); // Output: Slice: [20, 30, 40]

    assert_eq!(slice, &[20, 30, 40]);
    assert_eq!(slice.len(), 3);
    assert_eq!(slice[0], 20);
    assert_eq!(slice.is_empty(), false);

    let arr = &mut [1,2,3];
    arr[1] = 7;
    assert_eq!(arr, &mut [1,7,3]);
}