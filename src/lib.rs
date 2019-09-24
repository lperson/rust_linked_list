use std::boxed::Box;
use std::iter::Iterator;

// typr alias
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

impl<'a, T> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self { head: None }
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
    fn into(self) -> Vec<T> {
        let mut return_vec = Vec::new();
        let mut my_self = self;

        if my_self.head.is_some() {
            if my_self.head.as_ref().unwrap().next.is_none() {
                let popped = my_self.head.take().unwrap().data;
                return_vec.push(popped);
                return return_vec;
            }

            let mut curr = my_self.head.take().unwrap();
            loop {
                return_vec.push(curr.data);
                if curr.next.is_none() {
                    break;
                }
                curr = curr.next.take().unwrap();
            }
        } 

        return_vec
    }
}

trait Pushee<T> {
    fn push(&mut self, jawn: T);
    fn pop(&mut self) -> Option<T>;
    fn next(&self) -> &Link<T>;
    fn next_mut(&mut self) -> &mut Link<T>;
    fn peek(&self) -> Option<&T>;
}

impl<T> Pushee<T> for Node<T> {
    fn push(&mut self, jawn: T) {
        self.next = Some(Box::new(Node::new(jawn)));
    }

    fn pop(&mut self) -> Option<T> {
        if let Some(next) = &self.next {
            if next.next.is_none() {
                let jawn = self.next.take().unwrap().data;
                return Some(jawn);
            }
        }
        None
    }

    fn next(&self) -> &Link<T> {
        &self.next
    }

    fn next_mut(&mut self) -> &mut Link<T> {
        &mut self.next
    }

    fn peek(&self) -> Option<&T>
    {
        Some(&self.data)
    }
}

impl<T> Pushee<T> for SimpleLinkedList<T> {
    fn push(&mut self, jawn: T) {
        self.head = Some(Box::new(Node::new(jawn)));
    }

    fn pop(&mut self) -> Option<T> {
        if let Some(head) = &self.head {
            if head.next.is_none() {
                let jawn = self.head.take().unwrap().data;
                self.head = None;    
                return Some(jawn); 
            }
        }
        None
    }

    fn next(&self) -> &Link<T> {
        &self.head
    }

    fn next_mut(&mut self) -> &mut Link<T> {
        &mut self.head
    }

    fn peek(&self) -> Option<&T>
    {
        self.head.as_ref().map(|node| & node.data)
    }
}

impl<T> Pushee<T> for Box<Node<T>> {
    fn push(&mut self, jawn: T) {
        let unboxed = &mut **self;
        unboxed.push(jawn);
    }

    fn pop(&mut self) -> Option<T> {
        let unboxed = &mut **self;
        unboxed.pop()
    }

    fn next(&self) -> &Link<T> {
        let unboxed = &**self;
        unboxed.next()
    }

    fn next_mut(&mut self) -> &mut Link<T> {
        let unboxed = &mut **self;
        unboxed.next_mut()
    }

    fn peek(&self) -> Option<&T>
    {
        let unboxed = &**self;
        unboxed.peek()
    }
}

impl<'a, T> SimpleLinkedList<T> {
    pub fn new() -> SimpleLinkedList<T> {
        Default::default()
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

    pub fn push(&mut self, item: T) {
        let mut curr: &mut dyn Pushee<T> = self;
        while let Some(_) = curr.next_mut() {
            curr = curr.next_mut().as_mut().unwrap();
        }
        curr.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut curr: &mut dyn Pushee<T> = self;
        while let Some(next) = curr.next_mut() {
            if next.next.is_none() {
                break;
            }
            curr = curr.next_mut().as_mut().unwrap();
        }
        curr.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        let mut curr: &dyn Pushee<T> = self;
        while let Some(_) = curr.next() {
            curr = curr.next().as_ref().unwrap();
        }
        curr.peek()
    }

    fn reverser(new_list: &mut SimpleLinkedList<T>, current_node: &mut Link<T>) {
        let next_node = &mut current_node.as_mut().unwrap().next;
        if next_node.is_some() {
            Self::reverser(new_list, next_node);
        }

        new_list.push(current_node.take().unwrap().data)
    }

    pub fn rev(&mut self) -> SimpleLinkedList<T> {
        if self.head.is_none() {
            return Default::default();
        }

        if self.head.as_ref().unwrap().next.is_none() {
            return SimpleLinkedList {
                head: Some(self.head.take().unwrap()),
            };
        }

        let mut new_list: SimpleLinkedList<T> = Default::default();
        Self::reverser(&mut new_list, &mut self.head);

        new_list
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
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.data
        })
    }
}

#[cfg(test)]
mod test {
    use super::SimpleLinkedList;

    #[test]
    fn iterate_over_empty_list() {
        let the_list: SimpleLinkedList<u8> = SimpleLinkedList::new();
        let collected: Vec<&u8> = the_list.iter().collect();
        assert_eq!(collected.len(), 0);
    }
}
