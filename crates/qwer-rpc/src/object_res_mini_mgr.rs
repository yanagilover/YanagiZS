use std::sync::atomic::{self, AtomicU64};

use dashmap::{mapref::one::Ref, DashMap};

pub struct ObjectResMiniMgr<T> {
    uid_counter: AtomicU64,
    objects: DashMap<u64, T>,
}

impl<T> ObjectResMiniMgr<T>
where
    T: ResObj,
{
    pub fn new() -> Self {
        Self {
            uid_counter: AtomicU64::new(1),
            objects: DashMap::new(),
        }
    }

    pub fn insert(&self, obj: T) -> u64 {
        let uid = self.uid_counter.fetch_add(1, atomic::Ordering::SeqCst);
        obj.set_uid(uid);
        self.objects.insert(uid, obj);

        uid
    }

    pub fn iter(&self) -> dashmap::iter::Iter<'_, u64, T> {
        self.objects.iter()
    }

    pub fn get(&self, uid: u64) -> Option<Ref<u64, T>> {
        self.objects.get(&uid)
    }

    pub fn release(&self, uid: u64) -> Option<(u64, T)> {
        self.objects.remove(&uid)
    }
}

pub trait ResObj {
    fn set_uid(&self, uid: u64);
    fn get_uid(&self) -> u64;
}
