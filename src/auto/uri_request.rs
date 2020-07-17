// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_sys;

glib_wrapper! {
    pub struct URIRequest(Object<webkit2_sys::WebKitURIRequest, webkit2_sys::WebKitURIRequestClass, URIRequestClass>);

    match fn {
        get_type => || webkit2_sys::webkit_uri_request_get_type(),
    }
}

impl URIRequest {
    pub fn new(uri: &str) -> URIRequest {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(webkit2_sys::webkit_uri_request_new(uri.to_glib_none().0))
        }
    }
}

pub const NONE_URI_REQUEST: Option<&URIRequest> = None;

pub trait URIRequestExt: 'static {
    //fn get_http_headers(&self) -> /*Ignored*/Option<soup::MessageHeaders>;

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn get_http_method(&self) -> Option<GString>;

    fn get_uri(&self) -> Option<GString>;

    fn set_uri(&self, uri: &str);

    fn connect_property_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<URIRequest>> URIRequestExt for O {
    //fn get_http_headers(&self) -> /*Ignored*/Option<soup::MessageHeaders> {
    //    unsafe { TODO: call webkit2_sys:webkit_uri_request_get_http_headers() }
    //}

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn get_http_method(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_uri_request_get_http_method(self.as_ref().to_glib_none().0))
        }
    }

    fn get_uri(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_uri_request_get_uri(self.as_ref().to_glib_none().0))
        }
    }

    fn set_uri(&self, uri: &str) {
        unsafe {
            webkit2_sys::webkit_uri_request_set_uri(self.as_ref().to_glib_none().0, uri.to_glib_none().0);
        }
    }

    fn connect_property_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_uri_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_sys::WebKitURIRequest, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<URIRequest>
        {
            let f: &F = &*(f as *const F);
            f(&URIRequest::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::uri\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_uri_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for URIRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "URIRequest")
    }
}
