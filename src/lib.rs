use std::boxed::Box;
use std::iter::Iterator;

pub struct Node<'a, T> {
    data: Option<T>,
    next: Option<Box<Node<'a, T>>>,
    _ghost: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Node<'a, T> {
    pub fn new() -> Node<'a, T> {
        Node{ data: Option::None, next: Option::None, _ghost: std::marker::PhantomData}
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_none()
    }

    pub fn iter(&self) -> NodeIter<'a,T> {
        NodeIter::new(Box::new(*self))
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
    current:Option<Box<Node<'a, T>>>
}

impl<'a, T> NodeIter<'a, T> {
    fn new(current: Box<Node<'a, T>>) -> Self {
        Self {
            current:Option::Some(current)
        }
    }
}

impl<'a, T: 'a> Iterator for NodeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // save the data so we can return it 32
        let current: Option<Box<Node<'a, T>>> = self.current;

        let temp_node = &self.current.as_ref().unwrap();
        let next: Option<Box<Node<'a, T>>>  = temp_node.next.as_ref()ls
        ;

        self.current = next;

        if let Some(data) = &current.as_ref().unwrap().data {
            Some(data)
        } else {
            None
        }
    }

}