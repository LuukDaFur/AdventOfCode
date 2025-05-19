fn main() {
    //Variables
    let mut list1 = true;
    let mut totalDistance = 0;
    //Read Input file
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input file");
    //Split the input into lines
    let lines: Vec<&str> = input.lines().collect();
    //Create vectors to store the values
    let mut values1: Vec<i32> = Vec::new();
    let mut values2: Vec<i32> = Vec::new();
    let mut distance: Vec<i32> = Vec::new();
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
    values1.sort();
    values2.sort();
    //iterate over the vectors, calculate how far apart the values are and push them to the distance vector
    for i in 0..values1.len() {
        let mut diff = values2[i] - values1[i];
        diff = diff.abs();
        distance.push(diff);
    }
    //Print the distance vector
    for i in 0..distance.len() {
        totalDistance += distance[i];
    }
    //Print the total distance
    println!("Total distance: {}", totalDistance);
}
