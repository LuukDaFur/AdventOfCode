use std::vec;

fn main() {
    //Variables
    let mut list1 = true;
    let mut totalList: Vec<i32> = Vec::new();
    let mut similarityScore: i32 = 0;
    //Read Input file
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input file");
    //Split the input into lines
    let lines: Vec<&str> = input.lines().collect();
    //Create vectors to store the values
    let mut values1: Vec<i32> = Vec::new();
    let mut values2: Vec<i32> = Vec::new();
    //Iterate over the lines
    for line in lines {
        //Split the line into words
        let words: Vec<&str> = line.split_whitespace().collect();
        //Iterate over the words
        for word in words {
            //Parse the word as an i32 and push it to the vector
            if let Ok(value) = word.parse::<i32>() {
                println!("Parsed value: {}", value);
                if list1 {
                    //If the value is in the first list, add it to the vector
                    values1.push(value);
                } else {
                    //If the value is in the second list, add it to the vector
                    values2.push(value);   
                }
                list1 = !list1;
            }
        }
    }
    //Iterate over the first vector and sort it while removing duplicates
    values1.sort();
    values1.dedup();
    println!("Amount of values in list 1: {}", values1.len());
    println!("Amount of values in list 2: {}", values2.len());
    //iterate over values1, multiplying each value by the amount of times it appears in vector 2 and saving it to totalList
    for value in &values1 {
        let mut count = 0;
        for value2 in &values2 {
            if value == value2 {
                count += 1;
            }
        }
        totalList.push(value * count);
    }
    println!("Total List: {:?}", totalList);
    //Iterate over totalList and sum the values
    for value in &totalList {
        similarityScore += value;
    }
    println!("Similarity Score: {}", similarityScore);
}
