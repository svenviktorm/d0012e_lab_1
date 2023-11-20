pub fn sort(array: &mut [isize]){
    // for each element in the array
    for i in 1..(array.len()){
        // find insertion index of array[i] in slice 0..i
        let index = bi_find_insert_index(&array[0..i],array[i]);
        // insert the value, slice includes index i
        insert_last_at(&mut array[0..=i], index)
    }
}

fn _find_insert_index(slice: &[isize], value: isize) -> usize {
    let mut i = slice.len();
    while i != 0 && value < slice[i-1]{
        i = i-1;
    }
    return i;
}

fn bi_find_insert_index(sub_list: &[isize], value: isize) -> usize {
    if sub_list.len() == 0 {
        return 0;
    }
    let i: usize = sub_list.len()/2;
    if value < sub_list[i] {
        return bi_find_insert_index(&sub_list[0..i], value);
    }
    else if value > sub_list[i] {
        return i+1 + bi_find_insert_index(&sub_list[(i+1)..], value);
    } else {
        return i;
    }
}

fn insert_last_at(slice: &mut [isize],index: usize){
    // getting last position index
    let mut j: usize =slice.len()-1;
    // saving our value at the last position
    let value = slice[j];
    // shifting all values after index one step to the right, overwriting the last element
    while j > index{
        slice[j] = slice[j-1];
        j = j-1;
    }
    // setting what was our last value to the index position
    slice[j] = value;
}
