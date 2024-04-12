fn insertion_sort(arr: &mut Vec<u8>) -> &[u8] {
    assert!(!arr.is_empty());

    arr.insert(0, 0); // add 0 for padding;

    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i - 1;

        while j > 0 && arr[j] > key {
            arr[j + 1] = arr[j];
            j -= 1;
        }

        arr[j + 1] = key;
    }

    arr.remove(0); // remove padding;

    arr
}

#[cfg(test)]
mod test {
    use super::insertion_sort;


    #[test]
    fn test_insertion_sort(){
        let mut arr = vec![5, 2, 4, 6, 1, 3];
        let mut arr2 = vec![31, 41, 59, 26, 41, 58];
        
        assert_eq!(insertion_sort(&mut arr), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(insertion_sort(&mut arr2), vec![26, 31, 41, 41, 58, 59]);
    }
}