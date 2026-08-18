#[derive(PartialEq, Clone, Debug)]
pub struct MapNode<K, V> {
    pub key: K,
    pub value: V,
}


#[derive(PartialEq, Clone, Debug)] 
pub struct Map<K, V> {
    pub nodes: Vec<MapNode<K, V>>,
}

impl <K: PartialEq + Clone + PartialOrd, V: Clone + PartialEq> Map<K, V> {
    pub fn new() -> Self {
        Map {
            nodes: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        for node in &mut self.nodes {
            if node.key == key {
                node.value = value;
                return;
            }
        }
        self.nodes.push(MapNode { key, value });
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        for node in &self.nodes {
            if &node.key == key {
                return Some(&node.value);
            }
        }
        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.nodes.iter().position(|node| &node.key == key)?;
        Some(self.nodes.remove(index).value)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.nodes.iter().any(|node| &node.key == key)
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    pub fn keys(&self) -> Vec<&K> {
        self.nodes.iter().map(|node| &node.key).collect()
    }

    pub fn values(&self) -> Vec<&V> {
        self.nodes.iter().map(|node| &node.value).collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.nodes.iter().map(|node| (&node.key, &node.value))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
        self.nodes.iter_mut().map(|node| (&node.key, &mut node.value))
    }

    pub fn sort_by_key(&mut self) {
        self.nodes.sort_by(|a, b| a.key.partial_cmp(&b.key).unwrap());
    }

    pub fn sort_by_value(&mut self) {
        self.nodes.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
    }
    
}
