use std::{time::{Duration, Instant}, fs::{File, self}, path::Path, io::prelude::*, env};
mod insertion_sort;
mod mod_merge_sort;

fn time_n_merge_sorts_size_m(n:u32,m:usize,k: usize) -> Duration {
    let mut total_time: Duration = Duration::from_nanos(0);
    for _tests in 0..n{
        // print!("gen data for {:?} out of {:?}; ",tests+1,n);
        let mut test_ary: Vec<isize> = Vec::with_capacity(m);
        for _i in 0..m{
            test_ary.push(rand::random());
        };
        // println!("timing; ");
        let now = Instant::now();
        mod_merge_sort::sort(& test_ary,k);
        total_time = total_time + now.elapsed();
        // println!("done");
        // println!("current average: {:?}",total_time/(tests+1));
    }
    return total_time/(n);
}


fn main() {


    let args: Vec<String> = env::args().collect();
    let file_path_name: &String = &args[1];

    // Create folder for data
    _ = fs::create_dir_all(Path::new(file_path_name).parent().unwrap());
    
    // run test for 100, creating 100 csv files
    for i in 0..100{
        // create file
        let mut data_file = File::create(format!("{}.{}.csv",file_path_name,i)).unwrap();
        // write header
        _ = data_file.write(b"n, k, time\n");
        // test for n 1000 to 100_000 with a step of 1000
        // example n=1000, n=2000, n=3000
        for n in (1000..=100_000).step_by(1000){
            // test k values of 1 uppto 25
            for k in 1..=25{
                // run timming test 100 times and take avarege
                let avg_time: Duration = time_n_merge_sorts_size_m(100, n, k);
                // wire {n}, {k} {avrage time} to file
                let formated_string: String = format!("{}, {}, {:?}\n", n, k, avg_time);
                _ = data_file.write(formated_string.as_bytes());
            }

        }
    }
}

#[cfg(test)]
mod unit_tests;
 