pub mod leetcode_75_332;

pub struct Solution {}

fn main() {
    let result = Solution::find_itinerary(vec![
        vec!["JFK".to_string(), "SFO".to_string()],
        vec!["JFK".to_string(), "ATL".to_string()],
        vec!["SFO".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "JFK".to_string()],
        vec!["ATL".to_string(), "SFO".to_string()],
    ]);
    println!("Result {:?}", result);
}
