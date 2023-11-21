use rand::{self, Rng};
use crate::{insertion_sort, mod_merge_sort};


#[test]
fn one_element_array(){
    for _i in 0..1000{
        let x: isize = rand::random();
        let mut test_ary = [x];
        let test_ary_copy= [x]; 
        insertion_sort::sort(&mut test_ary);
        assert_eq!(test_ary,test_ary_copy)
    }

}

#[test]
fn already_sorted(){
    for _i in 0..1000{
        let mut test_ary: [isize; 10] = [0; 10];
        let mut test_ary_copy: [isize; 10]= [0; 10]; 
        for i in 0..10{
            let x = rand::random();
            test_ary[i] = x;
            test_ary_copy[i] = x;
        };
        test_ary.sort();
        test_ary_copy.sort();
        insertion_sort::sort(&mut test_ary);
        assert_eq!(test_ary,test_ary_copy)
    }
}

#[test]
fn array_testing_n10(){
    for _i in 0..1000{
        let mut test_ary: [isize; 10] = [0; 10];
        let mut test_ary_copy: [isize; 10]= [0; 10]; 
        for i in 0..10{
            let x = rand::random();
            test_ary[i] = x;
            test_ary_copy[i] = x;
        };
        test_ary_copy.sort();
        insertion_sort::sort(&mut test_ary);
        assert_eq!(test_ary,test_ary_copy)
    }
}

#[test]
fn works_with_vec_n10(){
    for _i in 0..1000{
        let mut test_ary: Vec<isize> = Vec::with_capacity(10);
        let mut test_ary_copy: Vec<isize>; 
        for _i in 0..10{
            test_ary.push(rand::random());
        };
        test_ary_copy = test_ary.to_vec();
        test_ary_copy.sort();
        insertion_sort::sort(&mut test_ary);
        assert_eq!(test_ary,test_ary_copy)
    }
}

#[test]
fn works_with_random_vec_len(){
    for _i in 0..1000{
        let n: usize = rand::thread_rng().gen_range(1..100);
        let mut test_ary: Vec<isize> = Vec::with_capacity(n);
        let mut test_ary_copy: Vec<isize>; 
        for _i in 0..n{
            test_ary.push(rand::random());
        };
        test_ary_copy = test_ary.to_vec();
        test_ary_copy.sort();
        insertion_sort::sort(&mut test_ary);
        assert_eq!(test_ary,test_ary_copy)
    }
}

#[test]
fn mod_merge_works_with_random_vec_len(){
    for _i in 0..1000{
        let n: usize = rand::thread_rng().gen_range(1..100);
        let mut test_ary: Vec<isize> = Vec::with_capacity(n);
        let test_ary_merge: Vec<isize>; 
        for _i in 0..n{
            test_ary.push(rand::random());
        };
        test_ary_merge = mod_merge_sort::sort(&test_ary, 3);
        test_ary.sort();
        assert_eq!(test_ary,test_ary_merge)
    }
}
