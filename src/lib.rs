#![feature(libc, std_misc, path)]
extern crate libc;
extern crate gl;

#[allow(dead_code)]
pub mod ffi;

use std::path::{Path};
use std::ffi::{CString};
use std::result::{Result};

#[cfg(target_os="macos")]
#[link(name = "OpenGL", kind = "framework")]
extern {}

pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    return (r as u32) | ((g as u32) << 8) | ((b as u32) << 16) | ((a as u32) << 24);
}

#[allow(missing_copy_implementations)]
pub struct UIRoot;
#[allow(missing_copy_implementations)]
pub struct UINode;

pub const MOUSE_UNPRESSED: u8 = 0x00;
pub const MOUSE_LEFT_PRESSED: u8 = 0x01;
pub const MOUSE_RIGHT_PRESSED: u8 = 0x02;

pub const ALIGN_LEFT: i32 = 0;
pub const ALIGN_CENTER: i32 = 1;
pub const ALIGN_RIGHT: i32 = 2;

impl UIRoot {
    pub fn begin<F>(&self, mouse_pos: (i32, i32), mouse_btn: u8, mouse_scroll: i32, child: F) where F: FnOnce(UINode) {
        let (x, y) = mouse_pos;
        unsafe { ffi::imguiBeginFrame(x, y, mouse_btn, mouse_scroll); }
        child(UINode);
        unsafe { ffi::imguiEndFrame(); }
    }

    pub fn render(&self, width: i32, height: i32) {
        unsafe { ffi::imguiRenderGLDraw(width, height); }
    }

    pub fn draw_rect(&self, pos: (i32, i32), size: (i32, i32), color: u32) {
        let (x, y) = pos;
        let (w, h) = size;
        unsafe { ffi::imguiDrawRect(x as f32, y as f32, w as f32, h as f32, color); }
    }

    pub fn draw_text(&self, pos: (i32, i32), align: i32, text: &str, color: u32) {
        let (x, y) = pos;
        let c_text = CString::from_slice(text.as_bytes());
        unsafe { ffi::imguiDrawText(x, y, align, c_text.as_ptr(), color); }
    }

    pub fn draw_line(&self, a: (i32, i32), b: (i32, i32), stroke: f32, color: u32) {
        let (x0, y0) = a;
        let (x1, y1) = b;
        unsafe { ffi::imguiDrawLine(x0 as f32, y0 as f32, x1 as f32, y1 as f32, stroke, color); }
    }

    pub fn draw_rounded_rect(&self, pos: (i32, i32), size: (i32, i32), radius: f32, color: u32) {
        let (x, y) = pos;
        let (w, h) = size;
        unsafe { ffi::imguiDrawRoundedRect(x as f32, y as f32, w as f32, h as f32, radius, color); }
    }

}

impl Drop for UIRoot {
    fn drop(&mut self) {
        unsafe { ffi::imguiRenderGLDestroy(); }
    }
}

impl UINode {

    pub fn scroll_area<F>(&self, text: &str, pos: (i32, i32), size: (i32, i32), scroll: &mut i32, child: F) where F: FnOnce(UINode) {
        let c_text = CString::from_slice(text.as_bytes());
        let (x, y) = pos;
        let (w, h) = size;
        unsafe { ffi::imguiBeginScrollArea(c_text.as_ptr(), x, y, w, h, scroll as *mut i32); }
        child(UINode);
        unsafe { ffi::imguiEndScrollArea(); }
    }

    pub fn separator(&self) {
        unsafe { ffi::imguiSeparator(); }
    }

    pub fn separator_line(&self) {
        unsafe { ffi::imguiSeparatorLine(); }
    }

    pub fn indent(&self) {
        unsafe { ffi::imguiIndent(); }
    }

    pub fn unindent(&self) {
        unsafe { ffi::imguiUnindent(); }
    }

    pub fn label(&self, text: &str) {
        let c_text = CString::from_slice(text.as_bytes());
        unsafe { ffi::imguiLabel(c_text.as_ptr()); }
    }

    pub fn value(&self, text: &str) {
        let c_text = CString::from_slice(text.as_bytes());
        unsafe { ffi::imguiValue(c_text.as_ptr()); }
    }

    pub fn button(&self, text: &str, enabled: bool) -> bool {
        let c_text = CString::from_slice(text.as_bytes());
        unsafe {
            let result = ffi::imguiButton(c_text.as_ptr(), enabled as u32);
            if result == 0 {
                false
            } else {
                true
            }
        }
    }

    pub fn item(&self, text: &str, enabled: bool) -> bool {
        let c_text = CString::from_slice(text.as_bytes());
        unsafe {
            let result = ffi::imguiItem(c_text.as_ptr(), enabled as u32);
            if result == 0 {
                false
            } else {
                true
            }
        }
    }

    pub fn check(&self, text: &str, checked: bool, enabled: bool) -> bool {
        let c_text = CString::from_slice(text.as_bytes());
        unsafe {
            let result = ffi::imguiCheck(c_text.as_ptr(), checked as u32, enabled as u32);
            if result == 0 {
                false
            } else {
                true
            }
        }
    }

    pub fn collapse(&self, text: &str, checked: bool, enabled: bool) -> bool {
        let c_text = CString::from_slice(text.as_bytes());
        unsafe {
            let result = ffi::imguiCollapse(c_text.as_ptr(), checked as u32, enabled as u32);
            if result == 0 {
                false
            } else {
                true
            }
        }
    }

    pub fn slider(&self, text: &str, value: &mut f32, min: f32, max: f32, inc: f32, enabled: bool) {
        let c_text = CString::from_slice(text.as_bytes());
        unsafe { ffi::imguiSlider(c_text.as_ptr(), value as *mut f32, min, max, inc, enabled as u32); }
    }

}

pub fn init(font: &Path) -> Result<UIRoot, &'static str> {
    let c_font = CString::from_slice(font.as_str().unwrap().as_bytes());
    unsafe {
        if ffi::imguiRenderGLInit(c_font.as_ptr()) == 0 {
            Err("Unable to initialize imgui!")
        } else {
            Ok(UIRoot)
        }
    }
}

#[test]
fn rgba_test() {
    let red = rgba(255, 0, 0, 255);
    assert!(red == 0xFF0000FF);
}
