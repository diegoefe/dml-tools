#[macro_export]
macro_rules! add_to_proc {
    ($p:expr, $a:ident, $typ:tt, $met:tt, $($args:expr),*) => {
        let $a = $typ::$met($($args),*);
        $p.add(&$a);
    };
}

#[macro_export]
macro_rules! add_grant {
    // ($p:expr, $a:ident, $($args:expr),*) => {
    ($p:expr, $($args:expr),*) => {
        add_to_proc!($p, g, Grant, new, $($args),*);
    }
}

#[macro_export]
macro_rules! add_owner {
    // ($p:expr, $a:ident, $($args:expr),*) => {
    ($p:expr, $($args:expr),*) => {
        add_to_proc!($p, o, Owner, new, $($args),*);
    }
}

#[macro_export]
macro_rules! add_index {
    // ($p:expr, $a:ident, $($args:expr),*) => {
    ($p:expr, $($args:expr),*) => {
        add_to_proc!($p, i, Index, new, $($args),*);
    }
}

pub use add_to_proc;
pub use add_grant;
pub use add_owner;
pub use add_index;
