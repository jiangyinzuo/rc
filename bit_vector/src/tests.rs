use crate::BitVector;

#[test]
fn get_set_test() {
    let mut bv = BitVector::new(10);
    bv.set(2, true);
    bv.set(3, true);
    debug_assert!(bv.get(2).unwrap());
    debug_assert!(bv.get(3).unwrap());
    debug_assert!(!bv.get(0).unwrap());
    let mut bv2 = BitVector::new(10);
    bv2.set(1, true);
    bv2.set(3, true);
    bv.set_bitor(&bv2);

    debug_assert!(!bv.get(0).unwrap());
    for i in 1..=3 {
        debug_assert!(bv.get(i).unwrap());
    }
    for i in 4..10 {
        debug_assert!(!bv.get(i).unwrap());
    }
}