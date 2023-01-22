use crate::types::MalVal;
use std::{borrow::Borrow, collections::HashMap, hash::Hash, rc::Rc};

#[derive(Debug, Default)]
pub struct Env {
    outer: Option<Rc<Env>>,
    data: HashMap<String, MalVal>,
}

impl Env {
    pub fn set<T: Into<String>>(&mut self, key: T, value: MalVal) {
        self.data.insert(key.into(), value);
    }

    fn find<T>(&self, key: &T) -> Option<&Env>
    where
        T: Hash + Eq,
        String: Borrow<T>,
    {
        match self.data.get(key) {
            Some(_) => Some(&self),
            None => match &self.outer {
                None => None,
                Some(e) => e.find(key),
            },
        }
    }

    pub fn get<T>(&self, key: &T) -> Result<&MalVal, String>
    where
        T: Hash + Eq,
        String: Borrow<T>,
    {
        match self.find(key) {
            Some(e) => Ok(&e.data[key]),
            None => Err("Environment not found".into()),
        }
    }
}
