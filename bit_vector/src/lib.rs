mod tests;

const WORD_BITS: usize = 64;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitVector {
    inner: Vec<u64>,
    len: usize,
}

impl BitVector {
    pub fn new(size: usize) -> BitVector {
        let inner = vec![0; (size + WORD_BITS - 1) / WORD_BITS];
        BitVector { inner, len: size }
    }

    pub fn set_all_true(&mut self) {
        for i in self.inner.iter_mut() {
            *i = u64::max_value();
        }
    }

    pub fn set_all_false(&mut self) {
        for i in self.inner.iter_mut() {
            *i = 0;
        }
    }

    pub fn set(&mut self, idx: usize, value: bool) {
        if idx >= self.len {
            panic!("out of range");
        }
        let vec_idx = idx / WORD_BITS;
        let bit_idx = idx % WORD_BITS;
        let word = self.inner.get_mut(vec_idx).unwrap();
        let num = 1 << (WORD_BITS - 1 - bit_idx);
        if value {
            *word |= num;
        } else {
            *word &= !num;
        }
    }

    pub fn get(&self, idx: usize) -> Option<bool> {
        if idx >= self.len {
            None
        } else {
            let vec_idx = idx / WORD_BITS;
            let bit_idx = idx % WORD_BITS;
            let word = self.inner.get(vec_idx).unwrap();
            let word = *word;
            Some((word >> (WORD_BITS - 1 - bit_idx)) % 2 == 1)
        }
    }
    
    pub fn set_bitor(&mut self, other: &BitVector) {
        assert_eq!(self.len, other.len);
        for (i, o) in self.inner.iter_mut().zip(other.inner.iter()) {
            *i |= *o;
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
