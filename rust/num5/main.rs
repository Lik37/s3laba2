use std::env;

#[derive(Clone)]
struct HNode {
    c: u8,
    w: usize,
    left: Option<Box<HNode>>,
    right: Option<Box<HNode>>,
}

impl HNode {
    fn new_leaf(c: u8, w: usize) -> Self {
        HNode { c, w, left: None, right: None }
    }

    fn new_internal(w: usize, l: Box<HNode>, r: Box<HNode>) -> Self {
        HNode { c: 0, w, left: Some(l), right: Some(r) }
    }

    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

fn get_code(node: &HNode, c: u8, path: &mut String, result: &mut String) -> bool {
    if node.is_leaf() {
        if node.c == c {
            if path.is_empty() {
                result.push('0');
            } else {
                result.push_str(path);
            }
            return true;
        }
        return false;
    }

    if let Some(ref left) = node.left {
        path.push('0');
        if get_code(left, c, path, result) {
            return true;
        }
        path.pop(); // откат
    }

    if let Some(ref right) = node.right {
        path.push('1');
        if get_code(right, c, path, result) {
            return true;
        }
        path.pop(); // откат
    }

    false
}

fn build_tree(s: &str) -> Option<Box<HNode>> {
    let mut cnts = [0usize; 256];
    for b in s.bytes() {
        cnts[b as usize] += 1;
    }

    let mut nodes: Vec<Box<HNode>> = Vec::new();
    for i in 0..256 {
        if cnts[i] > 0 {
            nodes.push(Box::new(HNode::new_leaf(i as u8, cnts[i])));
        }
    }

    if nodes.is_empty() {
        return None;
    }

    while nodes.len() > 1 {
        let mut i1 = 0;
        let mut i2 = 1;

        if nodes[i1].w > nodes[i2].w {
            std::mem::swap(&mut i1, &mut i2);
        }

        for i in 2..nodes.len() {
            if nodes[i].w < nodes[i1].w {
                i2 = i1;
                i1 = i;
            } else if nodes[i].w < nodes[i2].w {
                i2 = i;
            }
        }

        if i1 > i2 {
            std::mem::swap(&mut i1, &mut i2);
        }

        // Удаляем с конца, чтобы не сбить индексы
        let b = nodes.remove(i2);
        let a = nodes.remove(i1);

        nodes.push(Box::new(HNode::new_internal(a.w + b.w, a, b)));
    }

    Some(nodes.remove(0))
}

fn encode(s: &str, root: &HNode) -> String {
    let mut result = String::new();
    for b in s.bytes() {
        let mut path = String::new();
        let mut code = String::new();
        get_code(root, b, &mut path, &mut code);
        result.push_str(&code);
    }
    result
}

fn decode(bits: &str, root: &HNode) -> String {
    if root.is_leaf() {
        return std::iter::repeat(root.c as char).take(bits.len()).collect();
    }

    let mut result = String::new();
    let mut cur = root;

    for b in bits.bytes() {
        cur = if b == b'0' {
            cur.left.as_deref().unwrap()
        } else {
            cur.right.as_deref().unwrap()
        };

        if cur.is_leaf() {
            result.push(cur.c as char);
            cur = root;
        }
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: ./main <string>");
        return;
    }

    let s = &args[1];
    if s.is_empty() {
        println!("empty");
        return;
    }

    let root = build_tree(s).unwrap();
    let encoded = encode(s, &root);
    let decoded = decode(&encoded, &root);

    println!("encoded: {}", encoded);
    println!("decoded: {}", decoded);
}