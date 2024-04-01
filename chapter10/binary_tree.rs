use self::BinaryTree::*;
let jupiter_tree = NonEmpty(Box::new(TreeNode{
    element:"Jupiter",
    left:Empty,
    right:Empty
}));

// larger tree can be built from smaller ones
let mars_tree = NonEmpty(Box::new(TreeNode{
    element:"Mars",
    left:jupiter_tree,
    right:mercury_ree 
}));

// 自然，此分配会将jupiter_node和mercury_node的所有权转移到它们的新母节点。
// 树的其余部分遵循相同的模式。根节点与其他节点没有什么不同：

let tree = NonEmpty(Box::new(TreeNode{
    element:"Saturn",
    left:mars_tree,
    right:uranus_tree
}))

// Later in this chapter, we’ll show how to implement an add method on the BinaryTree
// type, so that we can instead write:

let mut tree = BinaryTree::Empty;
for planet in planets{
    tree.add(planet);
}

