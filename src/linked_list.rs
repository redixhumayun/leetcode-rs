// struct Node {
//     val: i32,
//     next: Option<Box<Node>>,
// }

// struct LinkedList {
//     head: Option<Box<Node>>,
// }

// impl LinkedList {
//     fn new() -> Self {
//         LinkedList { head: None }
//     }

//     fn append(&mut self, val: i32) {
//         let new_node = Box::new(Node { val, next: None });
//         match self.head.as_mut() {
//             Some(mut current) => {
//                 while let Some(ref mut next) = current.next {
//                     current = next;
//                 }
//                 current.next = Some(new_node);
//             }
//             None => {
//                 self.head = Some(new_node);
//             }
//         }
//     }
// }

pub trait Messenger {
    fn send(&mut self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a mut T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a mut T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}
