use std::time::{Duration, Instant};

fn insertion_sort(array: &mut [isize]){
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

fn run_n_sorts_size_m(n:u32,m:usize) -> Duration {
    let mut total_time: Duration = Duration::from_nanos(0);
    for tests in 0..n{
        print!("gen data for {:?} out of {:?}; ",tests+1,n);
        let mut test_ary: Vec<isize> = Vec::with_capacity(m);
        for _i in 0..m{
            test_ary.push(rand::random());
        };
        println!("timing; ");
        let now = Instant::now();
        insertion_sort(&mut test_ary);
        total_time = total_time + now.elapsed();
        println!("done");
        println!("current average: {:?}",total_time/(tests+1));
    }
    return total_time/(n.try_into().unwrap());
}

fn main() {
    // let mut my_array = [1,2,4,6,10,-10];
    // // let index = bi_find_insert_index(&my_array, 3);
    // // println!("found index {:?}",index);
    // insertion_sort(&mut my_array);
    // println!("{:?}",my_array);
    println!("average time: {:?}", run_n_sorts_size_m(1,1_000_000));
}

#[cfg(test)]
mod unit_tests;
 