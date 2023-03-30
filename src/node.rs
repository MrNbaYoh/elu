#[derive(Debug, Clone)]
pub(crate) struct Node<V> {
    parent: Option<usize>,
    value: V,
}

impl<V> Node<V> {
    pub(crate) fn new_root(value: V) -> Self {
        Self {
            parent: None,
            value,
        }
    }

    pub(crate) fn set_parent(&mut self, parent: usize) {
        self.parent = Some(parent);
    }

    pub(crate) fn set_value(&mut self, value: V) {
        self.value = value;
    }

    pub(crate) fn parent(&self) -> &Option<usize> {
        &self.parent
    }

    pub(crate) fn value(&self) -> &V {
        &self.value
    }

    pub(crate) fn is_root(&self) -> bool {
        self.parent.is_none()
    }
}
