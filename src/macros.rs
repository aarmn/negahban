#[macro_export]
macro_rules! hashset {
    ($( $x:expr ),*) => {{
         let mut set = ::std::collections::HashSet::new();
         $( set.insert($x); )*
         set
    }};
}