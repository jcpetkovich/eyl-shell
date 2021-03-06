use std::ptr;

use libc::{c_int, c_void, uint32_t};

use raw;
use raw::types::objects;
use raw::types::listeners;
use raw::types::utils;

pub static WL_REGISTRY_BIND: uint32_t = 0;

#[inline]
pub unsafe fn wl_registry_add_listener(
    wl_registry: *mut objects::wl_registry,
    listener: *const listeners::wl_registry_listener,
    data: *mut c_void
) -> c_int {
    raw::wl_proxy_add_listener(
        wl_registry as *mut objects::wl_proxy,
        listener as *mut extern fn(),
        data
    )
}

#[inline]
pub unsafe fn wl_registry_bind(
    wl_registry: *mut objects::wl_registry,
    name: uint32_t,
    interface: *const utils::wl_interface,
    version: uint32_t
) -> *mut c_void {
    let id = raw::wl_proxy_marshal_constructor(
        wl_registry as *mut objects::wl_proxy,
        WL_REGISTRY_BIND,
        interface,
        name,
        (*interface).name,
        version,
        ptr::null_mut::<c_void>()
    );
    id as *mut c_void
}

#[inline]
pub unsafe fn wl_registry_destroy(wl_registry: *mut objects::wl_registry) {
    raw::wl_proxy_destroy(wl_registry as *mut objects::wl_proxy);
}
