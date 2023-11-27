pub struct BinaryTree<T>
where
    T: std::fmt::Debug + Clone,
{
    elem: Option<Box<T>>,
    right: Option<Box<BinaryTree<T>>>,
    left: Option<Box<BinaryTree<T>>>,
}

pub type BinaryTreeNode<T> = BinaryTree<T>;

impl<T> BinaryTree<T>
where
    T: std::fmt::Debug + Clone + Ord,
{
    pub fn new(elem: T) -> BinaryTree<T> {
        BinaryTree {
            elem: Some(Box::new(elem)),
            right: None,
            left: None,
        }
    }

    pub fn delete(&mut self, del_elem: T) {
        if let Some(ref mut elem) = self.elem {
            if del_elem < **elem {
                if let Some(left) = &mut self.left {
                    left.delete(del_elem);
                }
            } else if del_elem > **elem {
                if let Some(right) = &mut self.right {
                    right.delete(del_elem);
                }
            } else {
                match (self.left.take(), self.right.take()) {
                    (None, right) => {
                        *self = BinaryTree {
                            elem: None,
                            left: None,
                            right: right,
                        };
                    }
                    (left, None) => {
                        *self = BinaryTree {
                            elem: None,
                            left: left,
                            right: None,
                        };
                    }
                    (Some(left), Some(right)) => {
                        let min_right = right.min().cloned();

                        *self = BinaryTree {
                            elem: min_right.map(Box::new),
                            left: Some(left),
                            right: Some(right),
                        };
                    }
                }
            }
        }
    }

    pub fn insert(&mut self, new_elem: T) {
        match &mut self.elem {
            Some(ref mut elem) => {
                if new_elem < **elem {
                    if let Some(left) = &mut self.left {
                        left.insert(new_elem);
                    } else {
                        self.left = Some(Box::new(BinaryTree::new(new_elem)));
                    }
                } else {
                    if let Some(right) = &mut self.right {
                        right.insert(new_elem);
                    } else {
                        self.right = Some(Box::new(BinaryTree::new(new_elem)));
                    }
                }
            }
            None => {
                self.elem = Some(Box::new(new_elem));
            }
        }
    }

    pub fn max(&self) -> Option<&T> {
        match &self.right {
            Some(right) => right.max(),
            None => self.elem.as_ref().map(|boxed_elem| &**boxed_elem),
        }
    }

    pub fn min(&self) -> Option<&T> {
        match &self.left {
            Some(left) => left.min(),
            None => self.elem.as_ref().map(|boxed_elem| &**boxed_elem),
        }
    }

    pub fn inorder(&self) {
        if let Some(left) = &self.left {
            left.inorder();
        }
        if let Some(elem) = &self.elem {
            println!("{:?}", elem);
        }
        if let Some(right) = &self.right {
            right.inorder();
        }
    }
}
