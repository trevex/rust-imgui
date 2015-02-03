use super::libc::{c_char, c_int, c_uchar, c_uint, c_float};

extern "C" {

    pub fn imguiBeginFrame(mx: c_int, my: c_int, mbut: c_uchar, scroll: c_int) -> ();
    pub fn imguiEndFrame() -> ();
    pub fn imguiBeginScrollArea(name: *const c_char, x: c_int, y: c_int, w: c_int, h: c_int, scroll: *mut c_int) -> c_uint;
    pub fn imguiEndScrollArea() -> ();
    pub fn imguiIndent() -> ();
    pub fn imguiUnindent() -> ();
    pub fn imguiSeparator() -> ();
    pub fn imguiSeparatorLine() -> ();
    pub fn imguiButton(text: *const c_char, enabled: c_uint) -> c_uint;
    pub fn imguiItem(text: *const c_char, enabled: c_uint) -> c_uint;
    pub fn imguiCheck(text: *const c_char, checked: c_uint, enabled: c_uint) -> c_uint;
    pub fn imguiCollapse(text: *const c_char, checked: c_uint, enabled: c_uint) -> c_uint;
    pub fn imguiLabel(text: *const c_char) -> ();
    pub fn imguiValue(text: *const c_char) -> ();
    pub fn imguiSlider(text: *const c_char, val: *mut c_float, vmin: c_float, vmax: c_float, vinc: c_float, enabled: c_uint) -> c_uint;

    pub fn imguiDrawText(x: c_int, y: c_int, align: c_int, text: *const c_char, color: c_uint) -> ();
    pub fn imguiDrawLine(x0: c_float, y0: c_float, x1: c_float, y1: c_float, r: c_float, color: c_uint) -> ();
    pub fn imguiDrawRoundedRect(x: c_float, y: c_float, w: c_float, h: c_float, r: c_float, color: c_uint) -> ();
    pub fn imguiDrawRect(x: c_float, y: c_float, w: c_float, h: c_float, color: c_uint) -> ();

    pub fn imguiRenderGLInit(fontpath: *const c_char) -> c_uint;
    pub fn imguiRenderGLDestroy() -> ();
    pub fn imguiRenderGLDraw(width: c_int, height: c_int) -> ();
}
