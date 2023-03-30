use std::marker::PhantomData;

use crate::node::Node;
use crate::operation::{AssociativeOperation, DefaultOperation};
use crate::EvalLinkUpdate;

// A simple safe index type for identifying nodes in a compressed forest.
#[derive(Debug)]
pub struct Index<F>(usize, PhantomData<F>);

impl<F> From<Index<F>> for usize {
    fn from(i: Index<F>) -> usize {
        i.0
    }
}

impl<F> Clone for Index<F> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData)
    }
}
impl<F> Copy for Index<F> {}

impl<F> PartialEq for Index<F> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<F> Eq for Index<F> {}

impl<F> PartialOrd for Index<F> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl<F> Ord for Index<F> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

/// A simple EVAL-LINK-UPDATE forest structure that performs (unbalanced) path compression.
///
/// `V` is the value type associated to nodes in the forest and `O` is the associative operation applied when evaluating.
#[derive(Debug, Clone)]
pub struct CompressedForest<V, O = DefaultOperation>
where
    O: 'static,
{
    nodes: Vec<Node<V>>,
    _op: PhantomData<O>,
}

impl<V, O> Default for CompressedForest<V, O>
where
    V: Clone,
    O: AssociativeOperation<V>,
{
    #[inline]
    fn default() -> Self {
        Self {
            nodes: vec![],
            _op: PhantomData,
        }
    }
}

impl<V, O> CompressedForest<V, O>
where
    V: Clone,
    O: AssociativeOperation<V>,
{
    /// Creates a new empty forest.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new empty forest with a given capacity.
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity),
            _op: PhantomData,
        }
    }

    /// Reserve enough space for a given number of nodes.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.nodes.reserve(additional);
    }

    fn compress(&mut self, key: usize) -> Result<(), O::Error> {
        let current = &self.nodes[key];
        // assume it's not a root
        let parent_key = current.parent().unwrap();
        let parent = &self.nodes[parent_key];

        // while the parent is not a root
        if !parent.is_root() {
            //TODO: get rid of recursive call
            self.compress(parent_key)?;

            let current_val = self.nodes[key].value();
            let parent = &self.nodes[parent_key];
            let parent_val = parent.value();
            let parent_parent = parent.parent().unwrap();

            let merged_values = O::associate(parent_val, current_val)?;
            self.nodes[key].set_value(merged_values);
            self.nodes[key].set_parent(parent_parent);
        }

        Ok(())
    }
}

impl<V, O> EvalLinkUpdate for CompressedForest<V, O>
where
    V: Clone,
    O: 'static + AssociativeOperation<V>,
{
    type Id = Index<Self>;
    type Value = V;
    type Operation = O;

    #[must_use]
    fn new_root(&mut self, value: V) -> Index<Self> {
        let index = self.nodes.len();
        self.nodes.push(Node::new_root(value));
        Index(index, PhantomData)
    }

    fn try_link(&mut self, id_a: Index<Self>, id_b: Index<Self>) -> Result<(), O::Error> {
        let id_a: usize = id_a.into();
        let id_b: usize = id_b.into();

        let root_a_key = if self.nodes[id_a].is_root() {
            id_a
        } else {
            self.compress(id_a)?;
            self.nodes[id_a].parent().unwrap()
        };

        let root_b_key = if self.nodes[id_b].is_root() {
            id_b
        } else {
            self.compress(id_b)?;
            self.nodes[id_b].parent().unwrap()
        };

        self.nodes[root_b_key].set_parent(root_a_key);
        // if "node a" is not the root of it's tree
        // need to update the value of "node b"
        if root_a_key != id_a {
            let new_value = O::associate(self.nodes[id_a].value(), self.nodes[root_b_key].value())?;
            self.nodes[root_b_key].set_value(new_value);
        }

        Ok(())
    }

    fn try_update(&mut self, id: Index<Self>, value: V) -> Result<(), O::Error> {
        let key: usize = id.into();
        let node = &mut self.nodes[key];

        if node.is_root() {
            node.set_value(value);
        } else {
            self.compress(key)?;
            // node is not root and compress ensure parent is root
            let parent_key = self.nodes[key].parent().unwrap();
            let parent = &mut self.nodes[parent_key];
            parent.set_value(value);
        }

        Ok(())
    }

    fn try_eval(&mut self, id: Index<Self>) -> Result<V, O::Error> {
        let id: usize = id.into();

        let node = &self.nodes[id];
        if !node.is_root() {
            self.compress(id)?;
        }

        let node = &self.nodes[id];
        match node.parent() {
            None => Ok(node.value().clone()),
            Some(parent_key) => {
                let parent = &self.nodes[*parent_key];
                O::associate(parent.value(), node.value())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operation::*;

    #[test]
    fn add_forest() {
        let mut forest: CompressedForest<usize, CloneAdd> = CompressedForest::with_capacity(4);
        let v0 = forest.new_root(2);
        let v1 = forest.new_root(3);
        let v2 = forest.new_root(4);
        let v3 = forest.new_root(5);

        forest.try_link(v0, v1).unwrap();
        forest.try_link(v2, v3).unwrap();

        assert_eq!(5, forest.try_eval(v1).unwrap());
        assert_eq!(9, forest.try_eval(v3).unwrap());

        forest.try_link(v3, v0).unwrap();

        assert_eq!(11, forest.try_eval(v0).unwrap());
        assert_eq!(14, forest.try_eval(v1).unwrap());
    }

    #[test]
    fn mul_forest() {
        let mut forest: CompressedForest<usize, CloneMul> = CompressedForest::with_capacity(4);
        let v0 = forest.new_root(2);
        let v1 = forest.new_root(3);
        let v2 = forest.new_root(4);
        let v3 = forest.new_root(5);

        forest.link(v0, v1);
        forest.link(v2, v3);

        assert_eq!(6, forest.eval(v1));
        assert_eq!(20, forest.eval(v3));

        forest.link(v3, v0);

        assert_eq!(40, forest.eval(v0));
        assert_eq!(120, forest.eval(v1));
    }
}
