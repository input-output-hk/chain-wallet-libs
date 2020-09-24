use thiserror::Error;

#[derive(Debug, Error)]
#[error("null pointer")]
pub(crate) struct NulPtr;

macro_rules! non_null {
    ( $obj:expr ) => {
        if let Some(obj) = $obj.as_ref() {
            obj
        } else {
            return crate::Error::invalid_input(stringify!($expr))
                .with(crate::c::commons::NulPtr)
                .into();
        }
    };
}

macro_rules! non_null_mut {
    ( $obj:expr ) => {
        if let Some(obj) = $obj.as_mut() {
            obj
        } else {
            return crate::Error::invalid_input(stringify!($expr))
                .with(crate::c::commons::NulPtr)
                .into();
        }
    };
}

///return C array as an out parameter
pub(crate) fn out_return_buffer(
    mut to_return: Box<[u8]>,
    out_pointer: &mut *const u8,
    out_size: &mut usize,
) {
    let pointer = to_return.as_mut_ptr();
    let length = to_return.len();

    //To avoid running the destructor
    std::mem::forget(to_return);

    //Write the array length
    unsafe { std::ptr::write(out_size, length) }

    //Copy the pointer to the out parameter
    unsafe { std::ptr::write(out_pointer, pointer) };
}
