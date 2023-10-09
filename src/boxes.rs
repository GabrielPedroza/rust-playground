#![allow(dead_code)]

pub fn boxes() {
    let mut boxed_num = Box::new(10);
    println!("Boxed num: {}", boxed_num);

    *boxed_num += 10;
    println!("Boxed num: {}", boxed_num);

    let boxed_tup = (Box::new(10), Box::new(20));
    println!("Boxed tup: {:?}", boxed_tup);

    let var = 10;

    let mut some = var;

    some += 1;

    println!("{:?}", var);
    println!("{:?}", some);


    struct TreeNode<T> {
        key: T,
        left: Option<Box<TreeNode<T>>>,
        right: Option<Box<TreeNode<T>>>,
    }

    impl<T> TreeNode<T> {
        fn new(key: T) -> Self {
            TreeNode {
                key,
                left: None,
                right: None,
            }
        }

        fn left(mut self, left: TreeNode<T>) -> Self {
            self.left = Some(Box::new(left));
            self
        }

        fn right(mut self, right: TreeNode<T>) -> Self {
            self.right = Some(Box::new(right));
            self
        }
    }

    let _tree = TreeNode::new("root")
        .left(TreeNode::new("left"))
        .right(TreeNode::new("right"));
}