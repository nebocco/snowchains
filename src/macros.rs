macro_rules! ensure_opt {
    ($x:expr) => {
        if !$x {
            return None;
        }
    };
}

macro_rules! println_plural {
    ($format:tt, $n:expr, $singular:expr, $plural:expr) => {
        println!(
            $format,
            format_args!("{} {}", $n, if $n > 1 { $plural } else { $singular })
        )
    };
}

#[cfg_attr(rustfmt, rustfmt_skip)]
macro_rules! derive_from {
    ($t:ident :: $v:ident <- $e:ty) => {
        impl From<$e> for $t {
            fn from(e: $e) -> Self {
                $t::$v(e.into())
            }
        }
    };
}
