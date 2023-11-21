use crate::insertion_sort;

pub fn sort(array: &[isize], sub_ary_len: usize) -> Vec<isize> {
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
    // println!("{:?}",sub_ary_vec);
    // sorting each vector
    for i in 0..sub_ary_amt{
        insertion_sort::sort(&mut sub_ary_vec[i]);
    }
    // println!("{:?}",sub_ary_vec);
    
    // merging each vector to the out vector
    let mut merging_vec: Vec<Vec<isize>> = sub_ary_vec;
    while merging_vec.len() > 1{
        let mut tmp_merging_vec: Vec<Vec<isize>> =Vec::with_capacity((sub_ary_amt/2)+1);
        for i in (0..merging_vec.len()).step_by(2) {
            if i+1 < merging_vec.len(){
                tmp_merging_vec.push(merge_sorted_vectors(&merging_vec[i], &merging_vec[i+1]))
            } else {
                tmp_merging_vec.push(merging_vec[i].to_vec());
            }
        }
        // println!("{:?}", tmp_merging_vec);
        merging_vec = tmp_merging_vec;
    }
    return merging_vec[0].to_vec();
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
