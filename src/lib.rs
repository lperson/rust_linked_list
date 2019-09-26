use std::boxed::Box;
use std::iter::{Iterator, IntoIterator};

// typr alias
type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(data: T, next: Link<T>) -> Self {
        Self { data, next }
    }
}

pub struct SimpleLinkedList<T> {
    head: Link<T>,
    len: usize,
}

impl<'a, T> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self { head: None, len: 0 }
    }
}

impl<T> From<&[T]> for SimpleLinkedList<T>
where
    T: Copy,
{
    fn from(array: &[T]) -> Self {
        let mut new_list: SimpleLinkedList<T> = Default::default();
        for x in array.iter() {
            new_list.push(*x);
        }
        new_list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut return_vec = Vec::with_capacity(self.len());
        let mut current = self.head.take();
        while let Some(mut unwrapped_current) = current {
            return_vec.insert(0, unwrapped_current.data);
            current = unwrapped_current.next.take();
        }
        return_vec
    }
}

pub struct IntoIter<T>(SimpleLinkedList<T>);

impl<T> IntoIterator for SimpleLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> Drop for SimpleLinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut unwrapped_current) = current {
            current = unwrapped_current.next.take();
        }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> SimpleLinkedList<T> {
        Default::default()
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn iter(&self) -> NodeIter<T> {
        NodeIter {
            next: self.head.as_ref().map::<&Node<T>, _>(|node| node),
        }
    }

    pub fn mut_iter(&mut self) -> MutNodeIter<T> {
        MutNodeIter {
            next: self.head.as_mut().map::<&mut Node<T>, _>(|node| node),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, jawn: T) {
        let new_jawn = Box::new(Node::new(jawn, self.head.take()));
        self.head = Some(new_jawn);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(old_head) = self.head.take() {
            self.head = old_head.next;
            self.len -= 1;
            return Some(old_head.data);
        }
        None
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(head) = self.head.as_ref() {
            return Some(&head.data);
        }
        None
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut new_list: SimpleLinkedList<T> = Default::default();

        let mut current = self.head.take();
        while let Some(mut unwrapped_current) = current {
            new_list.push(unwrapped_current.data);
            current = unwrapped_current.next.take();
        }
        new_list
    }
}

pub struct NodeIter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct MutNodeIter<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T: 'a> Iterator for NodeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.data
        })
    }
}

impl<'a, T: 'a> Iterator for MutNodeIter<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.data
        })
    }
}

#[cfg(test)]
mod test {
    use super::SimpleLinkedList;

    #[test]
    fn test_iterate_over_empty_list() {
        let the_list: SimpleLinkedList<u8> = SimpleLinkedList::new();
        let collected: Vec<&u8> = the_list.iter().collect();
        assert_eq!(collected.len(), 0);
    }

    #[test]
    fn test_iterate() {
        let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let iterated_list: Vec<&u32> = list.iter().collect();
        assert_eq!(iterated_list[0], &3);
        assert_eq!(iterated_list[1], &2);
        assert_eq!(iterated_list[2], &1);
    }

    #[test]
    fn test_into_iterator() {
        let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let iterated_list: Vec<u32> = list.into_iter().collect();
        assert_eq!(iterated_list[0], 3);
        assert_eq!(iterated_list[1], 2);
        assert_eq!(iterated_list[2], 1);
    }

    #[test]
    fn test_mut_iterator() {
        let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let iterated_list: Vec<&mut u32> = list.mut_iter().collect();
        assert_eq!(iterated_list[0], &mut 3);
        assert_eq!(iterated_list[1], &mut 2);
        assert_eq!(iterated_list[2], &mut 1);

        let mut iterated_list = list.mut_iter();
        let value = iterated_list.next().unwrap();
        *value = 4u32;

        assert_eq!(list.peek(), Some(&4) );
    }
}
