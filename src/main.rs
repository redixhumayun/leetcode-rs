use leetcode_75_2130::ListNode;

// pub mod leetcode_75_1431;
// pub mod leetcode_75_2;
// pub mod leetcode_75_605;
// use leetcode_75_605::Solution;
pub mod leetcode_75_2130;

pub struct Solution {}

fn main() {
    let values = [1, 10000];
    let mut head = None;
    for &value in values.iter().rev() {
        let new_node = Box::new(ListNode {
            val: value,
            next: head,
        });
        head = Some(new_node);
    }
    let result = Solution::pair_sum(head);
    println!("result {}", result);
    // let mut head = new_head;
    // loop {
    //     if head.is_none() {
    //         break;
    //     }
    //     let curr = head.unwrap();
    //     print!("{} ", curr.val);
    //     head = curr.next;
    // }
}
