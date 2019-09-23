use std::boxed::Box;
use std::iter::Iterator;

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

pub struct SimpleLinkedList<T> {
    head: Link<T>,
}

impl<'a, T> SimpleLinkedList<T> {
    pub fn new() -> SimpleLinkedList<T> {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn iter(&'a self) -> NodeIter<'a, T> {
        NodeIter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;

        for _ in self.iter() {
            count += 1;
        }

        count
    }
}

// iterator is a separate struct for keeping state
// of the thing we want to iterate
pub struct NodeIter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T: 'a> Iterator for NodeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        //let next: Option<&'a Node<T>> = self.next;

        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| { 
                &**node
            });
            &node.data
        })
    }
}

#[cfg(test)]
mod test{
    use super::SimpleLinkedList;

    #[test]
    fn iterate_over_empty_list() {
        let the_list: SimpleLinkedList<u8> = SimpleLinkedList::new();
        let collected: Vec<&u8> = the_list.iter().collect();
        assert_eq!(collected.len(), 0);
    }
}
