use super::queue::Queue;
use std::fmt::Display;
use std::io::Write;
use std::ptr;

pub struct TNode<T> {
    pub data: T,
    pub left: *mut TNode<T>,
    pub right: *mut TNode<T>,
    pub height: i32,
}

impl<T> TNode<T> {
    pub fn new(data: T) -> Self {
        TNode { data, left: ptr::null_mut(), right: ptr::null_mut(), height: 1 }
    }
}

pub struct AVLTree<T> {
    pub root: *mut TNode<T>,
}

impl<T> AVLTree<T> {
    pub fn new() -> Self {
        AVLTree { root: ptr::null_mut() }
    }

    pub fn clear(&mut self) {
        self.clearFrom(self.root);
        self.root = ptr::null_mut();
    }

    fn clearFrom(&mut self, node: *mut TNode<T>) {
        if node.is_null() { return; }
        unsafe {
            self.clearFrom((*node).left);
            self.clearFrom((*node).right);
            drop(Box::from_raw(node));
        }
    }

    fn height(node: *mut TNode<T>) -> i32 {
        if node.is_null() { return 0; }
        unsafe { (*node).height }
    }

    fn updateHeight(node: *mut TNode<T>) {
        unsafe {
            let lh = Self::height((*node).left);
            let rh = Self::height((*node).right);
            (*node).height = (if lh > rh { lh } else { rh }) + 1;
        }
    }

    fn balanceFactor(node: *mut TNode<T>) -> i32 {
        unsafe { Self::height((*node).left) - Self::height((*node).right) }
    }

    fn rotateRight(node: *mut TNode<T>) -> *mut TNode<T> {
        unsafe {
            let newRoot = (*node).left;
            (*node).left = (*newRoot).right;
            (*newRoot).right = node;
            Self::updateHeight(node);
            Self::updateHeight(newRoot);
            newRoot
        }
    }

    fn rotateLeft(node: *mut TNode<T>) -> *mut TNode<T> {
        unsafe {
            let newRoot = (*node).right;
            (*node).right = (*newRoot).left;
            (*newRoot).left = node;
            Self::updateHeight(node);
            Self::updateHeight(newRoot);
            newRoot
        }
    }

    fn balance(node: *mut TNode<T>) -> *mut TNode<T> {
        Self::updateHeight(node);
        let bf = Self::balanceFactor(node);
        unsafe {
            if bf > 1 {
                if Self::balanceFactor((*node).left) < 0 {
                    (*node).left = Self::rotateLeft((*node).left);
                }
                return Self::rotateRight(node);
            }
            if bf < -1 {
                if Self::balanceFactor((*node).right) > 0 {
                    (*node).right = Self::rotateRight((*node).right);
                }
                return Self::rotateLeft(node);
            }
        }
        node
    }
}

impl<T: Clone + PartialOrd> AVLTree<T> {
    pub fn add(&mut self, data: &T) {
        self.root = self.addFrom(self.root, data);
    }

    fn addFrom(&mut self, node: *mut TNode<T>, data: &T) -> *mut TNode<T> {
        if node.is_null() {
            return Box::into_raw(Box::new(TNode::new(data.clone())));
        }
        unsafe {
            if *data < (*node).data {
                (*node).left = self.addFrom((*node).left, data);
            } else if *data > (*node).data {
                (*node).right = self.addFrom((*node).right, data);
            } else {
                return node;
            }
        }
        Self::balance(node)
    }

    pub fn find(&self, data: &T) -> bool {
        let mut cur = self.root;
        while !cur.is_null() {
            unsafe {
                if *data == (*cur).data { return true; }
                if *data < (*cur).data { cur = (*cur).left; }
                else { cur = (*cur).right; }
            }
        }
        false
    }

    pub fn remove(&mut self, data: &T) {
        self.root = self.removeFrom(self.root, data);
    }

    fn removeFrom(&mut self, node: *mut TNode<T>, data: &T) -> *mut TNode<T> {
        if node.is_null() { return ptr::null_mut(); }
        unsafe {
            if *data < (*node).data {
                (*node).left = self.removeFrom((*node).left, data);
            } else if *data > (*node).data {
                (*node).right = self.removeFrom((*node).right, data);
            } else {
                if (*node).left.is_null() || (*node).right.is_null() {
                    let child = if !(*node).left.is_null() { (*node).left } else { (*node).right };
                    drop(Box::from_raw(node));
                    return child;
                }
                let minNode = Self::findMin((*node).right);
                (*node).data = (*minNode).data.clone();
                let minData = (*minNode).data.clone();
                (*node).right = self.removeFrom((*node).right, &minData);
            }
            Self::balance(node)
        }
    }

    fn findMin(mut node: *mut TNode<T>) -> *mut TNode<T> {
        unsafe {
            while !(*node).left.is_null() {
                node = (*node).left;
            }
        }
        node
    }
}

impl<T: Clone + PartialEq> AVLTree<T> {
    pub fn bfs(&self) -> Queue<T> {
        let mut allNodes: Queue<T> = Queue::new();
        if self.root.is_null() { return allNodes; }
        let mut level: Queue<*mut TNode<T>> = Queue::new();
        level.push(self.root);
        while !level.isEmpty() {
            let cur = *level.front();
            level.pop();
            unsafe {
                allNodes.push((*cur).data.clone());
                if !(*cur).left.is_null() { level.push((*cur).left); }
                if !(*cur).right.is_null() { level.push((*cur).right); }
            }
        }
        allNodes
    }
}

impl<T: Display> AVLTree<T> {
    pub fn print(&self) {
        self.printFrom(self.root, String::new(), false);
    }

    fn printFrom(&self, node: *mut TNode<T>, prefix: String, isLeft: bool) {
        if node.is_null() { return; }
        unsafe {
            print!("{}", prefix);
            print!("{}", if isLeft { "|—" } else { "∟—" });
            println!("{}", (*node).data);

            let newPrefix = format!("{}{}", prefix, if isLeft { "│ " } else { "  " });
            let hasLeft = !(*node).left.is_null();
            let hasRight = !(*node).right.is_null();
            if hasLeft && hasRight {
                self.printFrom((*node).left, newPrefix.clone(), true);
                self.printFrom((*node).right, newPrefix, false);
            } else if hasLeft {
                self.printFrom((*node).left, newPrefix, false);
            } else if hasRight {
                self.printFrom((*node).right, newPrefix, false);
            }
        }
    }

    pub fn save(&self, os: &mut impl Write) where T: Clone + PartialEq {
        let mut q = self.bfs();
        while !q.isEmpty() {
            writeln!(os, "{}", q.front()).unwrap();
            q.pop();
        }
    }
}

impl<T> Drop for AVLTree<T> {
    fn drop(&mut self) {
        self.clear();
    }
}