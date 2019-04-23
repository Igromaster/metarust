#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_char;

use cstr_macro::cstr;

/// Metamod binary interface version.
/// Any metamod implementation with interface lower
/// than this will fail to load plugin when used these bindings
/// Current version is "5:13"
pub const META_INTERFACE_VERSION: *const c_char = cstr!("5:13");

/// When metamod plugin can be loaded and unloaded
#[repr(u32)]
pub enum PLUG_LOADTIME {
    /// After loaded, should never be unloaded (?)
    PT_NEVER = 0,
    /// should only be loaded/unloaded at initial hlds execution
    PT_STARTUP = 1,
    /// can be loaded/unloaded between maps
    PT_CHANGELEVEL = 2,
    /// can be loaded/unloaded at any time
    PT_ANYTIME = 3,
    /// can be loaded/unloaded at any time, and can be "paused" during a map
    PT_ANYPAUSE = 4,
}

/// Basic information about plugin for metamod.
/// Contains information date for end user and plugin load/unload data
#[repr(C)]
pub struct plugin_info_t {
    /// meta_interface version. See [META_INTERFACE_VERSION](constant.META_INTERFACE_VERSION.html)
    pub ifvers: *const c_char,
    /// full name of plugin
    pub name: *const c_char,
    /// plugin version
    pub version: *const c_char,
    /// plugin date
    pub date: *const c_char,
    /// author name/email
    pub author: *const c_char,
    /// plugin URL
    pub url: *const c_char,
    /// log message prefix (unused right now)
    pub logtag: *const c_char,
    /// when plugin is loadable
    pub loadable: PLUG_LOADTIME,
    /// when plugin is unloadable
    pub unloadable: PLUG_LOADTIME,
}

/// Bindings are work in progress undone definitions are marked as unfinished (and not working of course)
type UNFINISHED_FUNCTION = unsafe extern "C" fn();

/// see UNFINISHED_FUNCTION
type UNFINISHED_FUNCTION_POINTER = Option<UNFINISHED_FUNCTION>;

#[repr(C)]
#[derive(Debug)]
pub struct META_FUNCTIONS {
    pub pfnGetEntityAPI: UNFINISHED_FUNCTION_POINTER,
    pub pfnGetEntityAPI_Post: UNFINISHED_FUNCTION_POINTER,
    pub pfnGetEntityAPI2: UNFINISHED_FUNCTION_POINTER,
    pub pfnGetEntityAPI2_Post: UNFINISHED_FUNCTION_POINTER,
    pub pfnGetNewDLLFunctions: UNFINISHED_FUNCTION_POINTER,
    pub pfnGetNewDLLFunctions_Post: UNFINISHED_FUNCTION_POINTER,
    pub pfnGetEngineFunctions: UNFINISHED_FUNCTION_POINTER,
    pub pfnGetEngineFunctions_Post: UNFINISHED_FUNCTION_POINTER,
}