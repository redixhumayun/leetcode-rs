pub mod doubly_linked_list;
use doubly_linked_list::DoublyLinkedList;

pub struct Solution {}

fn main() {
    let mut dll = DoublyLinkedList::new();
    dll.append(1).unwrap();
    dll.append(2).unwrap();
    dll.append(3).unwrap();
    dll.append(4).unwrap();
    dll.append(5).unwrap();

    for value in dll.iter() {
        print!("{} -> ", value);
    }
    print!("None");
    println!();

    let node = dll.find(3).unwrap();
    println!("node {:?}", node);

    dll.move_to_end(node).unwrap();
    for value in dll.iter() {
        print!("{} -> ", value);
    }
    print!("None");

    // let mut list = LinkedList::new();
    // list.append(1);
    // list.append(2);
    // list.append(3);
    // list.append(4);
    // list.append(5);

    // // Print the LinkedList
    // for value in list.iter() {
    //     print!("{} -> ", value);
    // }
    // println!("None");

    // list.remove_middle();
    // for value in list.iter() {
    //     print!("{} -> ", value);
    // }
    // println!("None");
}
