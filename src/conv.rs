use super::x as x;

impl From<wasi::Errno>
for x::Errno {
    fn from(value: wasi::Errno) -> Self {
       value.raw().into()
    }
}

impl From<wasi::Filetype>
for x::Filetype {
    fn from(value: wasi::Filetype) -> Self {
       value.raw().into()
    }
}

/// Unsafe
///
/// The Errno will be exactly the same size and should be compatible
///
impl From<x::Errno>
for wasi::Errno {
    fn from(value: x::Errno) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

/// Unsafe
///
/// The Errno will be exactly the same size and should be compatible
///
impl From<x::Filetype>
for wasi::Filetype {
    fn from(value: x::Filetype) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
