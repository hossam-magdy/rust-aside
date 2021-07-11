


// use std::rc::Rc;

// struct LinkedList<T> {
//     head: Rc<LinkedListItem<T>>,
//     tail: Rc<LinkedListItem<T>>,
// }

// // #[derive(Clone)]
// struct LinkedListItem<T> {
//     itemValue: T,
//     next: Option<Box<Self>>,
//     prev: Option<Box<Self>>,
// }

// trait LinkedListTrait<T> {
//     fn append(&mut self, item: T);
//     fn prepend(&self, item: T);
//     fn insert(&self, item: T, index: usize);
//     fn remove(&self, index: usize);
//     fn reverse(&self);
// }

// impl LinkedListTrait<i32> for LinkedList<i32> {
//     fn append(&mut self, item: i32) {
//         let old_tail = self.tail.to_owned(); // Copy happens
//         // self.tail = Rc::new(LinkedListItem {
//         //     itemValue: item,
//         //     next: None,
//         //     prev: Some(old_tail.into()),
//         // });
//     }

//     fn prepend(&self, item: i32) {
//         todo!()
//     }

//     fn insert(&self, item: i32, index: usize) {
//         todo!()
//     }

//     fn remove(&self, index: usize) {
//         todo!()
//     }

//     fn reverse(&self) {
//         todo!()
//     }
// }

// /**
// https://doc.rust-lang.org/std/collections/index.html

// - LinkedList (Single/Double):   Append, Prepend,    Insert,     Remove,     Reverse
// std::collections::LinkedList

// - Stack (FILO):                 Lookup, Pop,        Push,       Peek
// a stack may be implemented either on top of Vec or LinkedList (both feature pop_back and push_back)

// - Queue (FIFO):                 Lookup, Enqueue,    Dequeue,    Peek
// a queue may be implemented either on top of VecDeque or LinkedList (both feature pop_front and push_back)
// std::collections::VecDeque
// */

// pub fn try_linked_list() {}
