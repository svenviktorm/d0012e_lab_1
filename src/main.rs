use std::{time::{Duration, Instant}, fs::File, io::prelude::*};
mod insertion_sort;
mod mod_merge_sort;

fn time_n_merge_sorts_size_m(n:u32,m:usize) -> Duration {
    let mut total_time: Duration = Duration::from_nanos(0);
    for _tests in 0..n{
        // print!("gen data for {:?} out of {:?}; ",tests+1,n);
        let mut test_ary: Vec<isize> = Vec::with_capacity(m);
        for _i in 0..m{
            test_ary.push(rand::random());
        };
        // println!("timing; ");
        let now = Instant::now();
        mod_merge_sort::sort(& test_ary,10);
        total_time = total_time + now.elapsed();
        // println!("done");
        // println!("current average: {:?}",total_time/(tests+1));
    }
    return total_time/(n);
}

fn time_n_ins_sorts_size_m(n:u32,m:usize) -> Duration {
    let mut total_time: Duration = Duration::from_nanos(0);
    for _tests in 0..n{
        // print!("gen data for {:?} out of {:?}; ",tests+1,n);
        let mut test_ary: Vec<isize> = Vec::with_capacity(m);
        for _i in 0..m{
            test_ary.push(rand::random());
        };
        // println!("timing; ");
        let now = Instant::now();
        insertion_sort::sort(&mut test_ary);
        total_time = total_time + now.elapsed();
        // println!("done");
        // println!("current average: {:?}",total_time/(tests+1));
    }
    return total_time/(n);
}



fn main() {
    // let mut my_array = [1,2,4,6,10,-10];
    // // let index = bi_find_insert_index(&my_array, 3);
    // // println!("found index {:?}",index);
    // insertion_sort(&mut my_array);
    // println!("{:?}",my_array);
    let mut data_file = File::create("data.csv").unwrap();
    _ = data_file.write(b"n, time_ins, time_merge\n");
    for i in 1..=1000{
        let time_ins: Duration = time_n_ins_sorts_size_m(100, i*1000);
        let time_merge: Duration = time_n_merge_sorts_size_m(100, i*1000);
        let formated_string: String = format!("{}, {:?}, {:?}\n",i*1000,time_ins,time_merge);
        _ = data_file.write(formated_string.as_bytes());
    }
    // let mut test = [1,-1,5,4,2,3,-2];
    // let sorted = mod_merge_sort::sort(&test, 4);
    // println!("{:?}",sorted)


}

#[cfg(test)]
mod unit_tests;
 