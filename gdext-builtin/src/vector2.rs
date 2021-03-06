use crate::PtrCallArg;

#[cfg(not(feature = "real_is_double"))]
pub type Vector2 = glam::f32::Vec2;
#[cfg(feature = "real_is_double")]
pub type Vector2 = glam::f64::DVec2;

impl PtrCallArg for Vector2 {
    unsafe fn from_ptr_call_arg(arg: *const gdext_sys::GDNativeTypePtr) -> Self {
        *(*arg as *const Vector2)
    }

    unsafe fn to_ptr_call_arg(self, arg: gdext_sys::GDNativeTypePtr) {
        *(arg as *mut Vector2) = self;
    }
}
