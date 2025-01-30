fn median(a: Vec<f32>) -> Option<f32> {
    let mut median :f32 = 0.0;
    let elements :usize = a.len();
    println!("Amount of elements {:?}", elements);
    if elements.eq(&0){return None;}

    let mut list = a;
    list.sort_by( |a, b| a.partial_cmp(b).unwrap());
    let middle = list.len() / 2;

    median = if elements % 2 == 0 {
        (list[middle-1]+list[middle])/2.0
    } else {
        list[elements / 2]
    };

    Some(median)



}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
