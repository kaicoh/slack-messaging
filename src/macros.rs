macro_rules! pipe {
    ($val:expr => $f:path) => {{
        $f($val)
    }};
    ($val:expr => $f:path | $($g:path)|*) => {{
        pipe!($f($val) => $($g)|*)
    }};
}
