use std::iter::FusedIterator;

pub struct IntoPairs<K, V> {
    pairs: Vec<(K, V)>,
}

impl<K, V> IntoPairs<K, V> {
    pub fn new(pairs: Vec<(K, V)>) -> IntoPairs<K, V> {
        return Self {
            pairs,
        }
    }
}

impl<'a, K, V> Iterator for IntoPairs<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        return self.pairs.pop();
    }
}

pub struct IntoIter<K, V> {
    pairs: Vec<(K, V)>,
}

impl<K, V> IntoIter<K, V> {
    pub fn new(pairs: Vec<(K, V)>) -> IntoIter<K, V> {
        return Self {
            pairs,
        }
    }
}

impl<'a, K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        return self.pairs.pop();
    }
}

impl<'a, K, V> ExactSizeIterator for IntoIter<K, V> {
    fn len(&self) -> usize {
        return self.pairs.len();
    }
}

impl<'a, K, V> FusedIterator for IntoIter<K, V> { }

pub struct Map<K, V> {
    pairs: Vec<(K, V)>,
}

impl<K, V> Map<K, V> {
    pub fn new(pairs: Vec<(K, V)>) -> Map<K, V> {
        return Self {
            pairs,
        }
    }
}

impl<'a, K, V> Iterator for Map<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        return self.pairs.pop();
    }
}

impl<'a, K, V> ExactSizeIterator for Map<K, V> {
    fn len(&self) -> usize {
        return self.pairs.len();
    }
}

impl<'a, K, V> FusedIterator for Map<K, V> { }

pub struct Iter<'a, K, V> {
    pairs: Vec<(&'a K, &'a &'a V)>,
}

impl<'a, K, V> Iter<'a, K, V> {
    pub fn new(pairs: Vec<(&'a K, &'a &'a V)>) -> Iter<'a, K, V> {
        return Self {
            pairs,
        }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        return self.pairs.pop();
    }
}

impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V> {
    fn len(&self) -> usize {
        return self.pairs.len();
    }
}

impl<'a, K, V> FusedIterator for Iter<'a, K, V> { }

pub struct IterMut<'a, K, V> {
    pairs: Vec<(&'a K, &'a mut &'a V)>,
}

impl<'a, K, V> IterMut<'a, K, V> {
    pub fn new(pairs: Vec<(&'a K, &'a mut &'a V)>) -> IterMut<K, V> {
        return Self {
            pairs,
        }
    }
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        return self.pairs.pop();
    }
}

impl<'a, K, V> ExactSizeIterator for IterMut<'a, K, V> {
    fn len(&self) -> usize {
        return self.pairs.len();
    }
}

impl<'a, K, V> FusedIterator for IterMut<'a, K, V> { }