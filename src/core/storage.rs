use std::fmt::Debug;

pub type DynStorage = Box<dyn Storage>;

pub trait Storage: Debug + StorageClone + Send + Sync {}

pub trait StorageClone {
    fn clone_box(&self) -> Box<dyn Storage>;
}

impl<T> StorageClone for T
where
    T: 'static + Storage + Clone,
{
    fn clone_box(&self) -> Box<dyn Storage> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Storage> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
pub struct MemStorage {}

impl Storage for MemStorage {}
