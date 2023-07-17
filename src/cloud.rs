use std::{
    cell::RefCell,
    hash::{Hash, Hasher, BuildHasher},
    fmt::{Debug, Display}, collections::{HashMap, VecDeque},
    ops::{Index, IndexMut}
};

use fxhash::{FxBuildHasher, FxHashMap};

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

pub trait IntoOwned<K, V, S = FxBuildHasher> {
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
    fn into_owned(&self) -> HashMap<K, V, S>;
}

pub trait CombineWith {
    fn combine_with(&self, others: Vec<Self>) -> Self
    where
        Self: Sized;
}

pub trait AsPointer {
    fn as_ptr(&self) -> *const Self;
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
pub struct DataCloud<'a, K: PartialEq + Eq + Hash, V: PartialEq + Eq, S = FxBuildHasher> {
    nodes: RefCell<HashMap<K, &'a V, S>>,
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

    /// Inserts a new key into the cloud.
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

    /// Inserts a new key into the cloud if the key doesn't already exist, and returns whether the key was inserted or not.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// let y = 3;
    /// cloud.or_insert("y".to_string(), &y);
    /// ```
    pub fn or_insert(&self, key: K, value: &'a V) -> bool {
        let mut nodes = self.nodes.borrow_mut();
        if !nodes.contains_key(&key) {
            nodes.insert(key, value);
            return false
        }
        true
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

    /// Merges in place the DataCloud with the other one by consuming the other DataCloud.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// let y = 3;
    /// cloud.insert("y".to_string(), &y);
    /// 
    /// let cloud2: DataCloud<'_, String, i32> = DataCloud::new();
    /// let x = 56;
    /// cloud2.insert("x".to_string(), &x);
    /// 
    /// cloud.merge_in_place(cloud2);
    /// ```
    pub fn merge_in_place(&self, other: DataCloud<'a, K, V>) {
        self.nodes.borrow_mut().extend(other.nodes.into_inner().into_iter())
    }

    /// Merges in place the other DataCloud with this one by consuming this DataCloud.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// let y = 3;
    /// cloud.insert("y".to_string(), &y);
    /// 
    /// let cloud2: DataCloud<'_, String, i32> = DataCloud::new();
    /// let x = 56;
    /// cloud2.insert("x".to_string(), &x);
    /// 
    /// cloud2.merge_with(&cloud);
    /// 
    /// println!("{:?}", cloud);
    /// ```
    pub fn merge_with(self, other: &DataCloud<'a, K, V>) {
        other.nodes.borrow_mut().extend(self.nodes.into_inner().into_iter())
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

    /// Returns the cloud into an iterator of `(K, *mut V)` key-value pairs in arbitrary order.
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
    ///     println!("({key}: {})", *value);
    /// }
    /// ```
    pub unsafe fn into_raw_pairs(self) -> IntoPairs<K, *mut V> {
        return IntoPairs::new(self.nodes.into_inner().into_iter().map(|(k, v)| (k, v as *const V as *mut V)).collect());
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

    /// Consumes the DataCloud and returns a vector of tuples containing `(K, &'a V)`.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// let y = 3;
    /// cloud.insert("y".to_string(), &y);
    /// 
    /// let mut vec: Vec<(String, &i32)> = cloud.into_vec();
    /// assert_eq!(vec, vec![("y".to_string(), &y)]);
    /// ```
    pub fn into_vec(self) -> Vec<(K, &'a V)> {
        let collected = self.nodes.into_inner().into_iter().collect::<Vec<_>>();
        collected
    }

    /// Consumes the DataCloud and returns a VecDeque of tuples containing `(K, &'a V)`.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// use std::collections::VecDeque;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// let y = 3;
    /// cloud.insert("y".to_string(), &y);
    /// 
    /// let mut vec: VecDeque<(String, &i32)> = cloud.into_vecdeque();
    /// ```
    pub fn into_vecdeque(self) -> VecDeque<(K, &'a V)> {
        let collected = self.nodes.into_inner().into_iter().collect::<VecDeque<_>>();
        collected
    }

    /// Inserts multiple elements at a time in the DataCloud.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// 
    /// let x = 42;
    /// let y = 24;
    /// 
    /// let pairs: Vec<(String, &i32)> = vec![("x".to_string(), &x), ("y".to_string(), &y)];
    /// cloud.insert_all(pairs);
    /// ```
    pub fn insert_all(&self, pairs: Vec<(K, &'a V)>) -> Vec<Option<&'a V>> {
        let mut out = Vec::new();
        for (key, value) in pairs {
            out.push(self.insert(key, value));
        }
        out
    }

    /// Gets multiple elements at a time in the DataCloud.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// 
    /// let x = 42;
    /// let y = 24;
    /// 
    /// let x_string = "x".to_string();
    /// let y_string = "y".to_string();
    /// 
    /// cloud.insert("x".to_string(), &x);
    /// cloud.insert("y".to_string(), &y);
    /// 
    /// let pairs = vec![&x_string, &y_string];
    /// 
    /// assert_eq!(cloud.get_all(pairs), vec![Some(&x), Some(&y)])
    /// ```
    pub fn get_all(&self, keys: Vec<&K>) -> Vec<Option<&'a V>> {
        let mut out = Vec::new();
        for key in keys {
            out.push(self.get(key));
        }
        out
    }

     /// Gets multiple elements at a time in the DataCloud as mutable references.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// 
    /// let mut x = 42;
    /// let mut y = 24;
    /// 
    /// let x_string = "x".to_string();
    /// let y_string = "y".to_string();
    /// 
    /// cloud.insert("x".to_string(), &x);
    /// cloud.insert("y".to_string(), &y);
    /// 
    /// let pairs = vec![&x_string, &y_string];
    /// 
    /// let mut_pairs: Vec<Option<&mut i32>> = cloud.get_mut_all(pairs);
    /// ```
    pub fn get_mut_all(&self, keys: Vec<&K>) -> Vec<Option<&'a mut V>> {
        let mut out = Vec::new();
        for key in keys {
            out.push(self.get_mut(key));
        }
        out
    }

    /// Returns the DataCloud as a constant pointer to a `DataCloud<'a, K, V>`.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// 
    /// let pointer: *const DataCloud<'_, String, i32> = cloud.as_ptr();
    /// ```
    pub fn as_ptr(&self) -> *const DataCloud<'a, K, V> {
        return self as *const DataCloud<'a, K, V>
    }

    /// Returns the DataCloud's inner HashMap as a boxed shared reference `Box<&HashMap<K, &'a V, S>`.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// use fxhash::FxHashMap;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// 
    /// let boxed_ref: Box<&FxHashMap<String, &i32>> = cloud.as_boxed_ref();
    /// ```
    pub fn as_boxed_ref(&self) -> Box<&FxHashMap<K, &'a V>> {
        return Box::new(unsafe { self.nodes.as_ptr().as_ref() }.unwrap())
    }

    /// Returns the DataCloud's inner HashMap as a boxed raw pointer `Box<*const HashMap<K, &'a V, S>`.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// use fxhash::FxHashMap;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// 
    /// let boxed_ptr: Box<*const FxHashMap<String, &i32>> = unsafe {
    ///     cloud.as_boxed_ptr()
    /// };
    /// ```
    pub unsafe fn as_boxed_ptr(&self) -> Box<*const FxHashMap<K, &'a V>> {
        return Box::new(self.nodes.as_ptr())
    }

    /// Returns the DataCloud's inner HashMap as a boxed mutable reference `Box<&mut HashMap<K, &'a V, S>`.
    /// 
    /// # Examples
    /// ```
    /// use cloudr::DataCloud;
    /// use fxhash::FxHashMap;
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::new();
    /// 
    /// let boxed_ref: Box<&mut FxHashMap<String, &i32>> = unsafe {
    ///     cloud.as_boxed_mut()
    /// };
    /// ```
    pub unsafe fn as_boxed_mut(&self) -> Box<&mut FxHashMap<K, &'a V>> {
        return Box::new(unsafe { self.nodes.as_ptr().as_mut().unwrap() })
    }

    /// Retains only the elements specified by the predicate function.
    ///
    /// # Examples
    ///
    /// ```
    /// use cloudr::DataCloud;
    ///
    /// let cloud: DataCloud<String, i32> = DataCloud::new();
    /// cloud.insert("x".to_string(), &42);
    /// cloud.insert("y".to_string(), &24);
    ///
    /// cloud.retain(|key, value| {
    ///     *value > &30
    /// });
    ///
    /// assert!(cloud.contains_key(&"x".to_string()));
    /// assert!(!cloud.contains_key(&"y".to_string()));
    /// ```
    pub fn retain<F>(&self, mut predicate: F)
    where
        F: FnMut(&K, &&'a V) -> bool,
    {
        let mut nodes = self.nodes.borrow_mut();
        nodes.retain(|key, value| predicate(key, value));
    }

    /// Returns a new DataCloud from the given vector of keys and values `Vec<(K, &'a V)>`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cloudr::DataCloud;
    ///
    /// let mut vector = Vec::new();
    /// vector.push((String::from("Banana"), &1));
    /// 
    /// let cloud: DataCloud<'_, String, i32> = DataCloud::from_vec(vector);
    /// ```
    pub fn from_vec<T: Into<Vec<(K, &'a V)>>>(vec: T) -> Self {
        let mut hash = FxHashMap::default();

        for (k, v) in vec.into() {
            hash.insert(k, v);
        }

        Self::from_hashmap(hash)
    }
}

impl<'a, K: PartialEq + Eq + Hash + Clone, V: PartialEq + Eq + Clone> DataCloud<'a, K, V> {
    /// Merges the DataCloud with another and returns the resulting one.
    /// The other DataCloud will always have priority. So, if
    /// there are two conflicting keys, the other one will always have priority.
    /// 
    /// # Examples
    ///
    /// ```
    /// use cloudr::DataCloud;
    ///
    /// let cloud: DataCloud<String, i32> = DataCloud::new();
    /// cloud.insert("x".to_string(), &42);
    /// cloud.insert("y".to_string(), &24);
    ///
    /// let cloud2: DataCloud<String, i32> = DataCloud::new();
    /// cloud2.insert("z".to_string(), &64);
    /// 
    /// assert!(cloud.merge(&cloud2).contains_key(&"z".to_string()));
    /// ```
    pub fn merge(&self, other: &DataCloud<'a, K, V>) -> DataCloud<'a, K, V> {
        let new_cloud = DataCloud::new();
        for (key, value) in self.nodes.borrow().iter() {
            new_cloud.insert(key.clone(), value.clone());
        }

        for (key, value) in other.nodes.borrow().iter() {
            new_cloud.insert(key.clone(), value.clone());
        }

        new_cloud
    }

    /// Merges the DataCloud with other instances and returns the resulting one.
    /// The last element of the vector `others` will always have priority. So, if
    /// there are two conflicting keys, the last one will always have priority.
    /// 
    /// # Examples
    ///
    /// ```
    /// use cloudr::DataCloud;
    ///
    /// let cloud: DataCloud<String, i32> = DataCloud::new();
    /// cloud.insert("x".to_string(), &42);
    /// cloud.insert("y".to_string(), &24);
    ///
    /// let cloud2: DataCloud<String, i32> = DataCloud::new();
    /// cloud2.insert("z".to_string(), &64);
    /// 
    /// assert!(cloud.merge_all(vec![&cloud2]).contains_key(&"z".to_string()));
    /// ```
    pub fn merge_all(&self, others: Vec<&DataCloud<'a, K, V>>) -> DataCloud<'a, K, V> {
        let new_cloud = DataCloud::new();
        for (key, value) in self.nodes.borrow().iter() {
            new_cloud.insert(key.clone(), value.clone());
        }

        for other in others {
            for (key, value) in other.nodes.borrow().iter() {
                new_cloud.insert(key.clone(), value.clone());
            }
        }

        new_cloud
    }
}

impl<'a, K: PartialEq + Eq + Hash, V: PartialEq + Eq> Index<&K> for DataCloud<'a, K, V> {
    type Output = V;

    fn index(&self, index: &K) -> &Self::Output {
        return self.get(index).unwrap()
    }
}

impl<'a, K: PartialEq + Eq + Hash, V: PartialEq + Eq> IndexMut<&K> for DataCloud<'a, K, V> {
    fn index_mut(&mut self, index: &K) -> &mut Self::Output {
        return self.get_mut(index).unwrap()
    }
}

impl<'a, K: PartialEq + Eq + Hash, V: PartialEq + Eq> Extend<(K, &'a V)> for DataCloud<'a, K, V> {
    fn extend<T: IntoIterator<Item = (K, &'a V)>>(&mut self, iter: T) {
        let mut nodes = self.nodes.borrow_mut();
        let mut iter = iter.into_iter();
        while let Some((k, v)) = iter.next() {
            nodes.insert(k, v);
        }
    }
}

impl<'a, K: PartialEq + Eq + Hash, V: PartialEq + Eq> FromIterator<(K, &'a V)> for DataCloud<'a, K, V> {
    fn from_iter<T: IntoIterator<Item = (K, &'a V)>>(iter: T) -> Self {
        return Self::from_vec(iter.into_iter().collect::<Vec<_>>())
    }
}

impl<'a, K: PartialEq + Eq + Hash, V: PartialEq + Eq> Into<Vec<(K, &'a V)>> for DataCloud<'a, K, V> {
    fn into(self) -> Vec<(K, &'a V)> {
        return self.into_vec()
    }
}

impl<'a, K: PartialEq + Eq + Hash, V: PartialEq + Eq> Into<VecDeque<(K, &'a V)>> for DataCloud<'a, K, V> {
    fn into(self) -> VecDeque<(K, &'a V)> {
        return self.into_vecdeque()
    }
}

impl<'a, K: PartialEq + Eq + Hash, V: PartialEq + Eq> From<FxHashMap<K, &'a V>> for DataCloud<'a, K, V> {
    fn from(value: FxHashMap<K, &'a V>) -> Self {
        return Self::from_hashmap(value)
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

impl<'a, K: PartialEq + Eq + Hash + Clone, V: PartialEq + Eq + Clone, S: BuildHasher + Default> IntoOwned<K, V, S> for DataCloud<'a, K, V> {
    fn into_owned(&self) -> HashMap<K, V, S> {
        let mut new_map = HashMap::with_hasher(S::default());

        for (k, v) in self.nodes.borrow().iter() {
            new_map.insert(k.clone(), v.clone().clone());
        }

        new_map
    }
}

impl<'a, K: PartialEq + Eq + Hash + Clone, V: PartialEq + Eq> CombineWith for DataCloud<'a, K, V> {
    /// Enables the DataCloud to combine with other instances of the same type
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
        let new_cloud = DataCloud::<K, V, FxBuildHasher>::new();
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

unsafe impl<'a, K: PartialEq + Eq + Hash, V: PartialEq + Eq> Send for DataCloud<'a, K, V> {}
unsafe impl<'a, K: PartialEq + Eq + Hash + Send + Sync, V: PartialEq + Eq + Send + Sync> Sync for DataCloud<'a, K, V> {}

impl<'a, K: PartialEq + Eq + Hash, V: PartialEq + Eq> !Copy for DataCloud<'a, K, V> {}
