use std::{
    cell::RefCell,
    hash::{Hash, Hasher},
    fmt::{Debug, Display}
};

use fxhash::FxHashMap;

use crate::{
    iter::{
        IntoPairs,
        IntoIter,
        Map,
        Iter,
        IterMut
    }, 
    error::NullPointerError
};

pub trait IntoOwned {
    type Output;

    /// Returns an owned form of the object.
    /// 
    /// ```
    /// use cloudr::DataCloud;
    /// use fxhash::FxHashMap;
    /// use cloudr::IntoOwned;
    /// 
    /// let data: DataCloud<'_, String, String> = DataCloud::new();
    /// let x = "Hello master".to_string();
    /// let y = String::from("hello, world!");
    /// data.insert("x".to_string(), &x);
    /// data.insert("y".to_string(), &y);
    /// 
    /// let map: FxHashMap<String, String> = data.into_owned();
    /// ```
    fn into_owned(&self) -> Self::Output;
}

pub trait CombineWith {
    fn combine_with(&self, others: Vec<Self>) -> Self
    where
        Self: Sized;
}

/// An abstract data structure can store values without moving them.
/// 
/// # Examples
/// ```
/// use cloudr::DataCloud;
/// 
/// let data: DataCloud<'_, String, String> = DataCloud::new();
/// let x = "Hello master".to_string();
/// let y = String::from("hello, world!");
/// data.insert("x".to_string(), &x);
/// data.insert("y".to_string(), &y);
/// 
/// assert_eq!(&y, data.get(&"y".to_string()).unwrap());
/// ```
pub struct DataCloud<'a, K: PartialEq + Eq + Hash, V: PartialEq + Eq> {
    nodes: RefCell<FxHashMap<K, &'a V>>,
}

impl<'a, K: PartialEq + Eq + Hash, V: PartialEq + Eq> DataCloud<'a, K, V> {
    /// Returns a new instance of a DataCloud.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// ```
    pub fn new() -> DataCloud<'a, K, V> {
        return DataCloud {
            nodes: RefCell::new(FxHashMap::default()),
        }
    }

    /// Inserts a new key into the cloud
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// let y = 3;
    /// let inserted_before: Option<&i32> = cloud.insert("y".to_string(), &y);
    /// 
    /// ```
    pub fn insert(&self, key: K, value: &'a V) -> Option<&'a V> {
        let mut nodes = self.nodes.borrow_mut();
        nodes.insert(key, value)
    }

    /// Gets the reference stored in the cloud.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// let y = 3;
    /// cloud.insert("y".to_string(), &y);
    /// 
    /// let y_ref: Option<&i32> = cloud.get(&"y".to_string());
    /// assert_eq!(&y, y_ref.unwrap());
    /// ```
    pub fn get(&self, key_to_search_for: &K) -> Option<&'a V> {
        let nodes = self.nodes.borrow();
        for (key, value) in nodes.iter() {
            if key == key_to_search_for {
                return Some(*value)
            } else {
                continue;
            }
        }
        return None;
    }

    /// Gets the reference stored in the cloud as a mutable reference.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// let mut y = 3;
    /// cloud.insert("y".to_string(), &mut y);
    /// 
    /// 
    /// let y_ref: Option<&mut i32> = cloud.get_mut(&"y".to_string());
    /// ```
    pub fn get_mut(&self, key_to_search_for: &K) -> Option<&'a mut V> {
        let nodes = self.nodes.borrow();
        for (key, value) in nodes.iter() {
            if key == key_to_search_for {
                return Some(unsafe { {*value as *const V as *mut V}.as_mut().unwrap() })
            } else {
                continue;
            }
        }
        return None;
    }

    /// Removes the reference stored in the cloud and returns it if it exists.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// let mut y = 3;
    /// cloud.insert("y".to_string(), &mut y);
    /// 
    /// let y_ref = cloud.remove(&"y".to_string()).unwrap();
    /// ```
    pub fn remove(&self, key: &K) -> Option<&'a V> {
        match self.nodes.borrow_mut().remove(key) {
            Some(value) => {
                return Some(value)
            }
            None => return None,
        }
    }

    /// Returns if the cloud contains a reference indexed by this key.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// let mut x = 63;
    /// cloud.insert("x".to_string(), &mut x);
    /// 
    /// assert!(cloud.contains_key(&"x".to_string()));
    /// ```
    pub fn contains_key(&self, key: &K) -> bool {
        return self.nodes.borrow().contains_key(key)
    }

    /// Returns if the cloud contains the specified reference as a value.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// let x = 63;
    /// cloud.insert("x".to_string(), &x);
    /// 
    /// assert!(cloud.contains_value(&x));
    /// ```
    pub fn contains_value(&self, key: &V) -> bool {
        return self.nodes.borrow().values().collect::<Vec<_>>().contains(&&key)
    }

    /// Returns if the cloud does not contain any key-value pairs.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// 
    /// assert!(cloud.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        return self.nodes.borrow().is_empty();
    }

    /// Returns the cloud into an iterator of `(K, &'a V)` key-value pairs in arbitrary order.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// 
    /// let mut x = 63;
    /// cloud.insert("x".to_string(), &mut x);
    /// 
    /// for (key, value) in cloud.into_pairs() {
    ///     println!("({key}: {value})");
    /// }
    /// ```
    pub fn into_pairs(self) -> IntoPairs<K, &'a V> {
        return IntoPairs::new(self.nodes.into_inner().into_iter().collect());
    }

    /// Clears the `DataCloud`'s key-value pairs into a fresh, new one.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// 
    /// let cloud: DataCloud<'_, usize, i128> = DataCloud::new();
    /// 
    /// let v = 54;
    /// 
    /// cloud.insert(3, &v);
    /// 
    /// cloud.clear();
    /// 
    /// assert_eq!(DataCloud::new(), cloud);
    /// ```
    pub fn clear(&self) {
        self.nodes.borrow_mut().clear();
    }

    /// Inserts a new key into the cloud from a raw pointer
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// use cloudr::error::NullPointerError;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// let y = 3;
    /// let inserted_before: Result<Option<&i32>, NullPointerError> = unsafe {
    ///     cloud.insert_from_raw("y".to_string(), &y as *const i32)
    /// };
    /// 
    /// ```
    pub unsafe fn insert_from_raw(&self, key: K, value: *const V) -> Result<Option<&'a V>, NullPointerError> {
        if value.is_null() {
            return Err(NullPointerError(String::from("Tried to insert null pointer in DataCloud")));
        }
        let reference = unsafe { value.as_ref().unwrap() };
        Ok(self.insert(key, reference))
    }

    /// Gets the reference stored in the cloud as a mutable raw pointer.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// let y = 3;
    /// cloud.insert("y".to_string(), &y);
    /// 
    /// let pointer: Option<*mut i32> = unsafe {
    ///     cloud.get_as_raw(&"y".to_string())
    /// };
    /// ```
    pub unsafe fn get_as_raw(&self, key_to_search_for: &K) -> Option<*mut V> {
        let nodes = self.nodes.borrow();
        for (key, value) in nodes.iter() {
            if key == key_to_search_for {
                return Some(*value as *const V as *mut V)
            } else {
                continue;
            }
        }
        return None;
    }

    /// Builds a new DataCloud from a `FxHashMap<K, &'a V>`.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// use fxhash::FxHashMap;
    /// 
    /// let mut map: FxHashMap<String, &i32> = FxHashMap::default();
    /// let y = 3;
    /// map.insert("y".to_string(), &y);
    /// 
    /// let cloud = DataCloud::from_hashmap(map);
    /// ```
    pub fn from_hashmap(hashmap: FxHashMap<K, &'a V>) -> DataCloud<'a, K, V> {
        return Self {
            nodes: RefCell::new(hashmap),
        }
    }

    /// Calls a function for each key-value pair of the cloud and returns an iterator over the resulted pairs.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// use cloudr::iter::Map;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// let y = 3;
    /// cloud.insert("y".to_string(), &y);
    /// 
    /// let iterator: Map<String, i32> = cloud.map(|(k, v)| { return ( k.to_uppercase(), **v - 1 ) });
    /// ```
    pub fn map<F>(&self, f: F) -> Map<K, V>
    where
        F: Fn((&K, &&'a V)) -> (K, V) {
            let collected = self.nodes.borrow().iter().map(f).collect::<Vec<(K, V)>>();
            return Map::new(collected);
    }

    /// Returns an iterator over the elements of the cloud as `&'a` references.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// use cloudr::iter::Iter;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// let y = 3;
    /// cloud.insert("y".to_string(), &y);
    /// 
    /// let mut iterator: Iter<'_, String, i32> = cloud.iter();
    /// 
    /// assert_eq!((&"y".to_string(), &&3), iterator.next().unwrap());
    /// ```
    pub fn iter(&'a self) -> Iter<'a, K, V> {
        let collected = unsafe { self.nodes.as_ptr().as_ref().unwrap() }.iter().collect::<Vec<_>>();
        return Iter::new(collected);
    }

    /// Returns an iterator over the elements of the cloud as `&'a mut` references.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// use cloudr::iter::IterMut;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// let y = 3;
    /// cloud.insert("y".to_string(), &y);
    /// 
    /// let mut iterator: IterMut<'_, String, i32> = cloud.iter_mut();
    /// 
    /// assert_eq!((&"y".to_string(), &mut &3), iterator.next().unwrap());
    /// ```
    pub fn iter_mut(&'a self) -> IterMut<'a, K, V> {
        let collected = unsafe { self.nodes.as_ptr().as_mut().unwrap() }.iter_mut().collect::<Vec<(&'a K, &mut &'a V)>>();
        return IterMut::new(collected);
    }
}

impl<'a, K: PartialEq + Eq + Hash + Clone, V: PartialEq + Eq> Clone for DataCloud<'a, K, V> {
    fn clone(&self) -> Self {
        return DataCloud {
            nodes: self.nodes.clone(),
        }
    }
}

impl<'a, K: PartialEq + Eq + Hash + Clone + Debug, V: PartialEq + Eq + Debug> Debug for DataCloud<'a, K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::from("Cloud {\n");
        for (k, v) in self.nodes.borrow().iter() {
            output.push_str(&format!("    ({:?}: {:?}), \n", k, v));
        }
        output.remove(output.len() - 1);
        output.push_str("}");
        write!(f, "{}", output)
    }
}

impl<'a, K: PartialEq + Eq + Hash + Clone + Display, V: PartialEq + Eq + Display> Display for DataCloud<'a, K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::from("{\n");
        for (k, v) in self.nodes.borrow().iter() {
            output.push_str(&format!("    ({}: {}), \n", k, v));
        }
        output.remove(output.len() - 1);
        output.push_str("}");
        write!(f, "{}", output)
    }
}

impl<'a, K: PartialEq + Eq + Hash, V: PartialEq + Eq> PartialEq for DataCloud<'a, K, V> {
    fn eq(&self, other: &Self) -> bool {
        return self.nodes.borrow().iter().zip(other.nodes.borrow().iter()).all(|(kv1, kv2)| kv1 == kv2)
    }
}

impl<'a, K: PartialEq + Eq + Hash, V: PartialEq + Eq> IntoIterator for DataCloud<'a, K, V> {
    type IntoIter = IntoIter<K, &'a V>;
    type Item = (K, &'a V);

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.nodes.into_inner().into_iter().collect())
    }
}

impl<'a, K: PartialEq + Eq + Hash + PartialOrd, V: PartialEq + Eq + PartialOrd> PartialOrd for DataCloud<'a, K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.nodes.borrow().iter().zip(other.nodes.borrow().iter())
            .map(|(kv1, kv2)| kv1.partial_cmp(&kv2))
            .fold(Some(std::cmp::Ordering::Equal), |acc, ord| Some(acc?.then(ord?)));
    }
}

impl<'a, K: PartialEq + Eq + Hash, V: PartialEq + Eq> Eq for DataCloud<'a, K, V> {}

impl<'a, K: PartialEq + Eq + Hash, V: PartialEq + Eq + Hash> Hash for DataCloud<'a, K, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the length of the map
        self.nodes.borrow().len().hash(state);
        // Hash each key-value pair in the map
        for (k, v) in self.nodes.borrow().iter() {
            k.hash(state);
            v.hash(state);
        }
    }
}

impl<'a, K: PartialEq + Eq + Hash, V: PartialEq + Eq> Default for DataCloud<'a, K, V> {
    fn default() -> Self {
        return Self::new();
    }
}

impl<'a, K: PartialEq + Eq + Hash + Clone, V: PartialEq + Eq + Clone> IntoOwned for DataCloud<'a, K, V> {
    type Output = FxHashMap<K, V>;

    fn into_owned(&self) -> Self::Output {
        let mut new_map = FxHashMap::default();

        for (k, v) in self.nodes.borrow().iter() {
            new_map.insert(k.clone(), v.clone().clone());
        }

        new_map
    }
}

impl<'a, K: PartialEq + Eq + Hash + Clone, V: PartialEq + Eq> CombineWith for DataCloud<'a, K, V> {
    /// Enables the data cloud to combine with other instances of the same type
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// use cloudr::CombineWith;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// let y = 3;
    /// cloud.insert("y".to_string(), &y);
    /// 
    /// let cloud2: DataCloud<'_, String, i32> = DataCloud::new();
    /// let x = 4;
    /// cloud2.insert("x".to_string(), &x);
    /// 
    /// let final_cloud: DataCloud<'_, String, i32> = cloud.combine_with(vec![cloud2]);
    /// 
    /// let cloud3: DataCloud<'_, String, i32> = DataCloud::new();
    /// cloud3.insert("y".to_string(), &y);
    /// cloud3.insert("x".to_string(), &x);
    /// 
    /// assert_eq!(final_cloud, cloud3);
    /// ```
    fn combine_with(&self, others: Vec<Self>) -> Self
        where
            Self: Sized {
        let new_cloud: DataCloud<'_, _, _> = DataCloud::new();
        for cloud in others.into_iter() {
            let mut iter = cloud.into_iter();
            while let Some((key, val)) = iter.next() {
                new_cloud.insert(key, val);
            }
        }
        for (k, v) in self.nodes.borrow().iter() {
            new_cloud.insert(k.clone(), *v);
        }
        new_cloud
    }
}

unsafe impl<'a, K: PartialEq + Eq + Hash + Send, V: PartialEq + Eq + Send> Send for DataCloud<'a, K, V> {}
unsafe impl<'a, K: PartialEq + Eq + Hash + Send + Sync, V: PartialEq + Eq + Send + Sync> Sync for DataCloud<'a, K, V> {}