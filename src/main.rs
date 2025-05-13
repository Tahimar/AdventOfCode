mod day1;

fn main() {
    let total_distance = day1::calculate_total_distance();
    println!("Total distance: {}", total_distance);
    let total_similarity = day1::calculate_total_similarity();
    println!("Total similarity: {}", total_similarity);
}
