use shipyard::*;
use std::marker::PhantomData;

pub struct Parent <T>{
    pub num_children: usize,
    pub first_child: EntityId,
    marker: PhantomData<T>
}

impl <T> Parent<T> {
    pub fn new(num_children: usize, first_child: EntityId) -> Self {
        Self {
            num_children,
            first_child,
            marker: PhantomData
        }
    }
}

pub struct Child <T>{
    pub parent: EntityId,
    pub prev: EntityId,
    pub next: EntityId,
    marker: PhantomData<T>
}

impl <T> Child <T> {
    pub fn new(parent: EntityId, prev: EntityId, next: EntityId) -> Self {
        Self {
            parent, 
            prev, 
            next,
            marker: PhantomData
        }
    }
}

