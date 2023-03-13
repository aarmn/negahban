/**
 * Creates a `HashSet` instance with the given values.
 *
 * # Arguments
 *
 * * `$x:expr` - Values to be inserted into the new `HashSet`. # TODO: can be done better
 *
 * # Example
 *
 * ```
 * # #[macro_use] extern crate std; // TODO: is needed
 * use std::collections::HashSet;
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