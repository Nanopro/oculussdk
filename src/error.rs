use {
    std::{
        mem::{
            MaybeUninit,
        },
        ffi::{
            CStr,
        }
    },
    oculussdk_sys::{
        ovrResult, ovrErrorInfo, ovrErrorType, ovr_GetLastErrorInfo,
    }
};




pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone,)]
pub struct Error{
    message: String,
    code: i32,
}

impl From<ovrResult> for Error{
    fn from(er: ovrResult) -> Self{
        if er == 0 {
            panic!("This is not an error");
        }

        let mut error_info = MaybeUninit::uninit();
        unsafe{
            ovr_GetLastErrorInfo(error_info.as_mut_ptr())
        };

        let error = unsafe{
            error_info.assume_init()
        };

        Self{
            message: unsafe { CStr::from_ptr(error.ErrorString.as_ptr()) }.to_str().unwrap_or( "Unknown Message").to_owned(),
            code: error.Result,
        }

    }
}

impl Error{
    pub fn message(&self) -> &str{
        self.message.as_str()
    }
}














