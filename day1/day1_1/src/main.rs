use std::fs::read_to_string;

fn main() {
    let lines = read_lines("input.txt");

    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();


    for line in lines {
        let v: Vec<&str> = line.split("   ").collect();
        v1.push(v[0].parse::<i32>().unwrap());
        v2.push(v[1].parse::<i32>().unwrap());
    }

    v1.sort();
    v2.sort();

    let mut index1 = 0;
    let mut index2 = 0;

    let mut similarity = 0;

    let mut count = 0;

    while index1 < v1.len() && index2 < v2.len() {

        if v2[index2] < v1[index1] {
            index2 += 1;
        }
        else if v2[index2] == v1[index1] {
            count += 1;
            index2 += 1;
        }
        else {
            similarity += v1[index1] * count;
            index1 += 1;
            count = 0;
        }
    }
    
    println!("The similarity is {similarity}");

}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}
