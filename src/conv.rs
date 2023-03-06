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
