/**
 * Creates a [`HashSet`][`std::collections::HashSet`] instance with the given values.
 *
 * # Arguments
 *
 * * `$x:expr` - Values to be inserted into the new [`HashSet`][`std::collections::HashSet`]. <!-- TODO: can be done better -->
 *
 * # Example
 *
 * ```
 * use std::collections::HashSet;
 * use negahban::hashset;
 *
 * let set = hashset![1, 2, 3];
 *
 * assert!(set.contains(&1));
 * assert!(set.contains(&2));
 * assert!(set.contains(&3));
 * ```
 */
#[macro_export]
macro_rules! hashset {
    ($( $x:expr ),*) => {{
         let mut set = ::std::collections::HashSet::new();
         $( set.insert($x); )*
         set
    }};
}