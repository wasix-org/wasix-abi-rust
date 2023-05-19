use super::x as x;

impl From<wasi::Errno>
for x::Errno {
    fn from(value: wasi::Errno) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl From<wasi::Filetype>
for x::Filetype {
    fn from(value: wasi::Filetype) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl From<wasi::Ciovec>
for x::Ciovec {
    fn from(value: wasi::Ciovec) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl From<wasi::Iovec>
for x::Iovec {
    fn from(value: wasi::Iovec) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl From<x::Errno>
for wasi::Errno {
    fn from(value: x::Errno) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl From<x::Filetype>
for wasi::Filetype {
    fn from(value: x::Filetype) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl From<x::Ciovec>
for wasi::Ciovec {
    fn from(value: x::Ciovec) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl From<x::Iovec>
for wasi::Iovec {
    fn from(value: x::Iovec) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
