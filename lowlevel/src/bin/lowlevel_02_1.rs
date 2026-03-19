fn main() {
    let mut v = vec![3, 2, 1, 4, 5];
    bubble_sort(&mut v);
}

fn bubble_sort(vec: &mut Vec<i32>)  {
    let n  = vec.len();
    let mut swapped:bool;
    // primul loop
    for i in 0..n-1 {
        swapped = false;
        for j in 0..n-i-1 { // nr de elemente - iteratia - 1
            if vec[j] > vec[j+1] { // current > urmatorul
                let temp = vec[j]; //swap traditional cu temp
                vec[j] = vec[j+1]; //swap traditional cu temp
                vec[j+1] = temp; //swap traditional cu temp
                swapped = true; // a fost schimbat
            }
        }
        if !swapped { // din primul loop daca nu mai este nimic de schimbat atunci break
            break;
        }
    }
    println!("{:?}", vec);
}