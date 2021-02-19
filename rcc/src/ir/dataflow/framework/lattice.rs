//! Traits used to represent [lattices] for use as the domain of a dataflow analysis.
//!
//! # Overview
//!
//! The most common lattice is a powerset of some set `S`, ordered by [set inclusion]. The [Hasse
//! diagram] for the powerset of a set with two elements (`X` and `Y`) is shown below. Note that
//! distinct elements at the same height in a Hasse diagram (e.g. `{X}` and `{Y}`) are
//! *incomparable*, not equal.
//!
//! ```text
//!      {X, Y}    <- top
//!       /  \
//!    {X}    {Y}
//!       \  /
//!        {}      <- bottom
//!
//! ```
//!
//! The defining characteristic of a lattice—the one that differentiates it from a [partially
//! ordered set][poset]—is the existence of a *unique* least upper and greatest lower bound for
//! every pair of elements. The lattice join operator (`∨`) returns the least upper bound, and the
//! lattice meet operator (`∧`) returns the greatest lower bound. Types that implement one operator
//! but not the other are known as semilattices. Dataflow analysis only uses the join operator and
//! will work with any join-semilattice, but both should be specified when possible.
//!
//! ## `PartialOrd`
//!
//! Given that they represent partially ordered sets, you may be surprised that [`JoinSemiLattice`]
//! and [`MeetSemiLattice`] do not have [`PartialOrd`][std::cmp::PartialOrd] as a supertrait. This
//! is because most standard library types use lexicographic ordering instead of set inclusion for
//! their `PartialOrd` impl. Since we do not actually need to compare lattice elements to run a
//! dataflow analysis, there's no need for a newtype wrapper with a custom `PartialOrd` impl. The
//! only benefit would be the ability to check that the least upper (or greatest lower) bound
//! returned by the lattice join (or meet) operator was in fact greater (or lower) than the inputs.
//!
//! [lattices]: https://en.wikipedia.org/wiki/Lattice_(order)
//! [set inclusion]: https://en.wikipedia.org/wiki/Subset
//! [Hasse diagram]: https://en.wikipedia.org/wiki/Hasse_diagram
//! [poset]: https://en.wikipedia.org/wiki/Partially_ordered_set

/// A [partially ordered set][poset] that has a [least upper bound][lub] for any pair of elements
/// in the set.
///
/// [lub]: https://en.wikipedia.org/wiki/Infimum_and_supremum
/// [poset]: https://en.wikipedia.org/wiki/Partially_ordered_set
pub trait JoinSemiLattice: Eq {
    /// Computes the least upper bound of two elements, storing the result in `self` and returning
    /// `true` if `self` has changed.
    ///
    /// The lattice join operator is abbreviated as `∨`.
    fn join(&mut self, other: &Self) -> bool;
}
