// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QStyleOptionFrame::QStyleOptionFrame();
  fn _ZN17QStyleOptionFrameC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionFrame::QStyleOptionFrame(const QStyleOptionFrame & other);
  fn _ZN17QStyleOptionFrameC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionFrame::QStyleOptionFrame(int version);
  fn _ZN17QStyleOptionFrameC1Ei(qthis: *mut c_void, arg0: c_int);
}

// body block begin
// class sizeof(QStyleOptionFrame)=1
pub struct QStyleOptionFrame {
  pub qclsinst: *mut c_void,
}

  // proto:  void QStyleOptionFrame::QStyleOptionFrame();
impl /*struct*/ QStyleOptionFrame {
  pub fn NewQStyleOptionFrame<T: QStyleOptionFrame_NewQStyleOptionFrame>(value: T) -> QStyleOptionFrame {
    let rsthis = value.NewQStyleOptionFrame();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionFrame_NewQStyleOptionFrame {
  fn NewQStyleOptionFrame(self) -> QStyleOptionFrame;
}

  // proto:  void QStyleOptionFrame::QStyleOptionFrame();
impl<'a> /*trait*/ QStyleOptionFrame_NewQStyleOptionFrame for () {
  fn NewQStyleOptionFrame(self) -> QStyleOptionFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QStyleOptionFrameC1Ev()};
    unsafe {_ZN17QStyleOptionFrameC1Ev(qthis)};
    let rsthis = QStyleOptionFrame{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionFrame::QStyleOptionFrame(const QStyleOptionFrame & other);
impl<'a> /*trait*/ QStyleOptionFrame_NewQStyleOptionFrame for (QStyleOptionFrame) {
  fn NewQStyleOptionFrame(self) -> QStyleOptionFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QStyleOptionFrameC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QStyleOptionFrameC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionFrame{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionFrame::QStyleOptionFrame(int version);
impl<'a> /*trait*/ QStyleOptionFrame_NewQStyleOptionFrame for (i32) {
  fn NewQStyleOptionFrame(self) -> QStyleOptionFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QStyleOptionFrameC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN17QStyleOptionFrameC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionFrame{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}
