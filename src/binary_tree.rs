// Behavior methods that are the same accross all types of binary trees
pub trait BinaryTreeBehavior {
    fn get_left_child(index: usize) -> usize {
        2 * index
    }

    fn get_right_child(index: usize) -> usize {
        (2 * index) + 1
    }
}

// What we are dealing with is a complete binary tree, a complete binary tree
// is where every level is completely filled,
// except for possibly the last level, which is filled from left to right.
pub struct BinaryTree {
    ds: Vec<Option<u32>>,
}
impl BinaryTree {
    // Create a binary tree represented in array form with a single root node
    pub fn new(root_value: u32) -> Self {
        // We are always going to occupy the first index with a None value in order to make index calculations a breeze
        BinaryTree {
            ds: vec![None, Some(root_value)],
        }
    }

    pub fn get_array_representation(&self) -> Vec<Option<u32>> {
        self.ds.clone()
    }

    pub fn add(&mut self, value: u32) {
        self.ds.push(Some(value))
    }
    pub fn get(&self, index: u32) -> Option<u32> {
        self.ds[index as usize]
    }

    // Exercise 1:
    /// Returns the index of the binary tree node given a depth and offset with the name given them
    ///
    /// # Arguments
    ///
    /// * `depth` - An integer indicating this node's depth in the tree
    /// * `offset` - An integer indicating this node's offset in at its depth in the tree
    ///
    pub fn get_node_index(&self, depth: u32, offset: u32) -> u32 {
        let base: u32 = 2;
        // Raise 2 to the power of the depth and add the offset and you have the index of the node in questio
        base.pow(depth) + offset
    }

    // Exercise 2.1:
    /// Returns the depth and offset tuple for a given index of a binary tree node
    ///
    /// # Arguments
    ///
    /// * `index` - An integer indicating the a nodes index in the array representation of the binary tree
    ///
    pub fn get_depth_and_offset(&self, index: u32) -> (u32, u32) {
        // By same approach as to finding the height we can find the depth for this node
        // Remember we are starting our index count from one. We want to abstract this away from our users
        if index == 0 {
            panic!("This binary tree uses one based indexing")
        }
        let node_count = index - 1;
        let depth = (node_count + 1).ilog2();
        // We know that each depth level starts at index 2^d where d is depth
        // So by subtracting the depth from the inde x we get the offset from
        let base: u32 = 2;
        let start_index_at_depth = base.pow(depth);

        let offset = index - start_index_at_depth;
        (depth, offset)
    }

    // Exercise 2.2:
    /// Returns the index of the parent node in the binary tree node a child's index
    ///
    /// # Arguments
    ///
    /// * `index` - An integer indicating the node's index in the array representation of the binary tree
    ///
    pub fn get_parent(index: u32) -> u32 {
        index / 2
    }

    // Exercise 2.3:
    /// Returns the index of the left most child of a node with a given index
    ///
    /// # Arguments
    ///
    /// * `index` - An integer indicating the a nodes index in the array representation of the binary tree
    ///
    pub fn get_left_child(index: u32) -> u32 {
        2 * index
    }

    /// Returns the index of the right most child of a node with a given index
    ///
    /// # Arguments
    ///
    /// * `index` - An integer indicating the a nodes index in the array representation of the binary tree
    ///
    pub fn get_right_child(index: u32) -> u32 {
        (2 * index) + 1
    }

    // The height of a array represented, complete, binary tree is the node count
    pub fn height(&self) -> u32 {
        // This could have been a one liner but prioritizing clarity before brevity
        let node_count = self.ds.len() - 1;
        (node_count + 1).ilog2()
    }
}

impl BinaryTreeBehavior for BinaryTree {}

#[cfg(test)]
mod tests {
    use crate::binary_tree;

    use super::*;

    fn create_complete_binary_tree() -> BinaryTree {
        let mut bt = BinaryTree::new(0);
        for value in 1..14 {
            bt.add(value)
        }
        bt
    }

    #[test]
    fn should_create_new_binary_tree_with_inital_value() {
        let bt = BinaryTree::new(0);

        assert_eq!(
            bt.ds.len(),
            2,
            "Initial size of binary tree should be 2, because we mark the first slot off"
        );
        assert_eq!(
            bt.ds,
            vec![None, Some(0)],
            "Binary tree should contain the root node after instantiation"
        );
    }

    #[test]
    fn should_add_new_node_to_binary() {
        let mut bt = BinaryTree::new(0);
        bt.add(1);

        assert_eq!(
            bt.ds.len(),
            3,
            "Array representation should be of length 3, the two nodes and the first slot reserved"
        );
        assert_eq!(bt.ds, vec![None, Some(0), Some(1)]);
    }

    #[test]
    fn should_get_node_index_from_depth_offset_pair() {
        let bt = create_complete_binary_tree();

        let root_node_index = bt.get_node_index(0, 0);

        assert_eq!(root_node_index, 1);

        let last_node_index = bt.get_node_index(3, 7);
        assert_eq!(last_node_index, 15);
    }

    #[test]
    fn should_get_depth_and_offset_from_index() {
        let bt = create_complete_binary_tree();

        let root_depth_and_offset = bt.get_depth_and_offset(1);

        assert_eq!(root_depth_and_offset, (0, 0));

        let last_node_depth_and_offset = bt.get_depth_and_offset(15);
        assert_eq!(last_node_depth_and_offset, (3, 7));
    }

    #[test]
    fn should_return_parent_index() {
        let bt = create_complete_binary_tree();

        let root_as_parent = binary_tree::BinaryTree::get_parent(2);
        assert_eq!(root_as_parent, 1);

        // In a complete binary tree the last node of three depths has a parent with value 6, stored ad index 7
        let parent_of_last_node = binary_tree::BinaryTree::get_parent(15);
        assert_eq!(parent_of_last_node, 7);
        assert_eq!(bt.get(7).unwrap(), 6);
    }

    #[test]
    fn should_return_left_child_index() {
        let left_child_of_root = binary_tree::BinaryTree::get_left_child(1);
        assert_eq!(left_child_of_root, 2);

        let left_most_child = binary_tree::BinaryTree::get_left_child(4);
        assert_eq!(left_most_child, 8);
    }
}
