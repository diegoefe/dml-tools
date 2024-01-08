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

#[macro_export]
macro_rules! grant_perms {
    ($proc:expr, $roles:expr, $opath:expr) => {
        add_owner!($proc, &($roles).rw, $opath);
        add_grant!($proc, GrantType::All, &($roles).rw, $opath);
        add_grant!($proc, GrantType::All, &($roles).upd, $opath);
        add_grant!($proc, GrantType::Select, &($roles).ro, $opath);
    }
}  

pub use add_to_proc;
pub use add_grant;
pub use add_owner;
pub use add_index;
pub use grant_perms;
