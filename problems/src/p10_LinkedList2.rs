#[derive(Debug)]
pub enum List {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
pub struct Node {
    value: i32,
    next: List,
}

pub fn p10_main() {
    let mut head = Node {
        value: 2,
        next: List::Empty,
    };
    let tail = Node {
        value: 3,
        next: List::Empty,
    };
    head.next = List::More(Box::new(tail));

    // let list = LinkedList { head, tail };
    let list = List::More(Box::new(head));
    println!("{:#?}", list);
}

// impl List {
//     fn from_array(arr: &[i32]) -> List {
//         if arr.len() == 0 {
//             return List::Empty;
//         }

//         let mut head = Node {
//             value: 2,
//             next: List::Empty,
//         };

//         // for i in 0..len {}
//         return List::More(Box::new(head));

//         // let tail = Node {
//         //     value: 3,
//         //     next: List::Empty,
//         // };
//         // head.next = List::More(Box::new(tail));
//     }
//     // fn _from_array(arr: &[i32], prev_node: &mut Node) -> List {
//     //     if arr.len() == 0 {
//     //         prev_node.next = List::Empty;
//     //     }

//     //     prev_node.next = List::More(Box::new(Node {value: arr[0], next: _from_array(arr[1..])}))

//     //     let mut head = Node {
//     //         value: 2,
//     //         next: List::Empty,
//     //     };

//     //     // for i in 0..len {}
//     //     return List::More(Box::new(head));

//     //     // let tail = Node {
//     //     //     value: 3,
//     //     //     next: List::Empty,
//     //     // };
//     //     // head.next = List::More(Box::new(tail));
//     // }
// }
