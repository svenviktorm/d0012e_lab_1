fn insertion_sort(array: &mut [isize]){
    // for each elemnt in the array
    for i in 1..(array.len()){
        // find insertion index of array[i] in slice 0..i
        let index = find_insert_index(&array[0..i],array[i]);
        // insert the value, slice includes index i
        insert_last_at(&mut array[0..=i], index)
    }
}

fn find_insert_index(slice: &[isize], value: isize) -> usize {
    let mut i = slice.len();
    while i != 0 && value < slice[i-1]{
        i = i-1;
    }
    return i;
}

fn insert_last_at(slice: &mut [isize],index: usize){
    // geting last position index
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

fn main() {
    let mut my_array = [1,2,6,0];
    insertion_sort(&mut my_array);
    println!("{:?}",my_array);
}

#[cfg(test)]
mod unit_tests;
