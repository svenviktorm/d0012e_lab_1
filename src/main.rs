use std::{time::{Duration, Instant}, fs::File, io::prelude::*};
mod insertion_sort;

fn run_n_sorts_size_m(n:u32,m:usize) -> Duration {
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

fn mod_merge_sort(array: &[isize], sub_ary_len: usize) -> Vec<isize> {
    let mut sub_ary_amt = array.len()/sub_ary_len;
    // if there is a partial array at the end
    if sub_ary_amt*sub_ary_len < array.len(){
        sub_ary_amt = sub_ary_amt + 1
    }
    
    // create a vector of vector with each* vector with size sub_ary_len
    let mut sub_ary_vec: Vec<Vec<isize>> = Vec::with_capacity(sub_ary_amt);
    for i in 0..sub_ary_amt{
        // to avoid 
        let mut slice_end = (i+1)*sub_ary_len;
        if slice_end > array.len(){
            slice_end = array.len();
        }
        sub_ary_vec.push(array[(i*sub_ary_len)..slice_end].to_vec());
    }
    println!("{:?}",sub_ary_vec);
    // sorting each vector
    for i in 0..sub_ary_amt{
        insertion_sort::sort(&mut sub_ary_vec[i]);
    }
    println!("{:?}",sub_ary_vec);
    
    // merging each vector to the out vector
    let mut out: Vec<isize> = vec![];
    for i in sub_ary_vec{
        out = merge_sorted_vectors(&out, &i)
    }
    return out;
}


fn merge_sorted_vectors(vec1: &Vec<isize>, vec2: &Vec<isize>) -> Vec<isize>{
    let mut out: Vec<isize> = Vec::with_capacity(vec1.len()+vec2.len());
    let mut vec1_index = 0;
    let mut vec2_index = 0;
    loop{
        if vec1_index == vec1.len(){
            add_rest(&mut out, & vec2[vec2_index..vec2.len()]);
            break;
        } else if vec2_index == vec2.len() {
            add_rest(&mut out, & vec1[vec1_index..vec1.len()]);
            break;
        } else if vec1[vec1_index] < vec2[vec2_index] {
            out.push(vec1[vec1_index]);
            vec1_index = vec1_index + 1;
        } else {
            out.push(vec2[vec2_index]);
            vec2_index = vec2_index + 1;
        }
    }
    return out;
}
fn add_rest(vec: &mut Vec<isize>,vec2: &[isize]){
    for i in vec2{
        vec.push(*i)
    }
}


fn main() {
    // let mut my_array = [1,2,4,6,10,-10];
    // // let index = bi_find_insert_index(&my_array, 3);
    // // println!("found index {:?}",index);
    // insertion_sort(&mut my_array);
    // println!("{:?}",my_array);
    // let mut data_file = File::create("data.log").unwrap();
    // _ = data_file.write(b"n, time\n");
    // for i in 1..=100{
    //     let time: Duration = run_n_sorts_size_m(100, i*1000);
    //     let formated_string: String = format!("{}, {:?}\n",i*1000,time);
    //     _ = data_file.write(formated_string.as_bytes());
    // }
    let mut test = [1,-1,5,4,2,3,-2];
    let sorted = mod_merge_sort(&test, 2);
    println!("{:?}",sorted)

    // let a = vec![1,3,5];
    // let b = vec![2,4,5];
    // println!("{:?}",merge_sorted_vectors(&a, &b));
}

#[cfg(test)]
mod unit_tests;
 