use super::{
    coord::Coord,
    node::QuadNode,
    region::Region,
    slot_map::{SlotId, SlotMap},
};

#[derive(Debug)]
pub struct QuadTree<T> {
    region_store: SlotMap<Region>,
    value_store: SlotMap<T>,
    root: Box<QuadNode>,
}

impl<T> QuadTree<T> {
    pub fn new(region: Region) -> Self {
        Self {
            region_store: SlotMap::new(),
            value_store: SlotMap::new(),
            root: Box::new(QuadNode::new(region, 0)),
        }
    }

    pub fn clear(&mut self) {
        self.region_store.clear();
        self.value_store.clear();
        self.root.clear();
    }

    pub fn query(&self, region: &Region, exclude : &Vec<SlotId>) -> Vec<&T> {
        self.root
            .query(&region, &self.region_store, exclude)
            .iter()
            .map(|id| self.value_store.get(id).unwrap())
            .collect()
    }

    pub fn size(&self) -> &Region {
        self.root.size()
    }

    pub fn get_regions(&self) -> Vec<&Region> {
        self.root.get_regions()
    }

    pub fn insert(&mut self, region: Region, values: T) -> SlotId {
        self.region_store.insert(region);
        let id = self.value_store.insert(values);
        self.root.insert(&id, &self.region_store);
        id
    }
}