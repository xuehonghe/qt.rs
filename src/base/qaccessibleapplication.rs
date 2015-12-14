// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwindow::QWindow;
use super::qaccessibleinterface::QAccessibleInterface;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QAccessibleApplication::NewQAccessibleApplication();
  fn _ZN22QAccessibleApplicationC1Ev(qthis: *mut c_void) ;
  // proto:  QWindow * QAccessibleApplication::window();
  fn _ZNK22QAccessibleApplication6windowEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAccessibleInterface * QAccessibleApplication::child(int index);
  fn _ZNK22QAccessibleApplication5childEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  int QAccessibleApplication::childCount();
  fn _ZNK22QAccessibleApplication10childCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QAccessibleInterface * QAccessibleApplication::parent();
  fn _ZNK22QAccessibleApplication6parentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAccessibleInterface * QAccessibleApplication::focusChild();
  fn _ZNK22QAccessibleApplication10focusChildEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QAccessibleApplication::indexOfChild(const QAccessibleInterface * );
  fn _ZNK22QAccessibleApplication12indexOfChildEPK20QAccessibleInterface(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QAccessibleApplication)=16
pub struct QAccessibleApplication {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleApplication {
  pub fn NewQAccessibleApplication<T: QAccessibleApplication_NewQAccessibleApplication>(value: T) -> QAccessibleApplication {
    let rsthis = value.NewQAccessibleApplication();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleApplication_NewQAccessibleApplication {
  fn NewQAccessibleApplication(self) -> QAccessibleApplication;
}

// proto: void QAccessibleApplication::NewQAccessibleApplication();
impl<'a> /*trait*/ QAccessibleApplication_NewQAccessibleApplication for () {
  fn NewQAccessibleApplication(self) -> QAccessibleApplication {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QAccessibleApplicationC1Ev()};
    unsafe {_ZN22QAccessibleApplicationC1Ev(qthis)};
    let rsthis = QAccessibleApplication{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessibleApplication {
  pub fn window<T: QAccessibleApplication_window>(&mut self, value: T) -> QWindow {
    return value.window(self);
    // return 1;
  }
}

pub trait QAccessibleApplication_window {
  fn window(self, rsthis: &mut QAccessibleApplication) -> QWindow;
}

// proto:  QWindow * QAccessibleApplication::window();
impl<'a> /*trait*/ QAccessibleApplication_window for () {
  fn window(self, rsthis: &mut QAccessibleApplication) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QAccessibleApplication6windowEv()};
    let mut ret = unsafe {_ZNK22QAccessibleApplication6windowEv(rsthis.qclsinst)};
    let mut ret1 = QWindow{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleApplication {
  pub fn child<T: QAccessibleApplication_child>(&mut self, value: T) -> QAccessibleInterface {
    return value.child(self);
    // return 1;
  }
}

pub trait QAccessibleApplication_child {
  fn child(self, rsthis: &mut QAccessibleApplication) -> QAccessibleInterface;
}

// proto:  QAccessibleInterface * QAccessibleApplication::child(int index);
impl<'a> /*trait*/ QAccessibleApplication_child for (i32) {
  fn child(self, rsthis: &mut QAccessibleApplication) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QAccessibleApplication5childEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK22QAccessibleApplication5childEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleApplication {
  pub fn childCount<T: QAccessibleApplication_childCount>(&mut self, value: T) -> i32 {
    return value.childCount(self);
    // return 1;
  }
}

pub trait QAccessibleApplication_childCount {
  fn childCount(self, rsthis: &mut QAccessibleApplication) -> i32;
}

// proto:  int QAccessibleApplication::childCount();
impl<'a> /*trait*/ QAccessibleApplication_childCount for () {
  fn childCount(self, rsthis: &mut QAccessibleApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QAccessibleApplication10childCountEv()};
    let mut ret = unsafe {_ZNK22QAccessibleApplication10childCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleApplication {
  pub fn parent<T: QAccessibleApplication_parent>(&mut self, value: T) -> QAccessibleInterface {
    return value.parent(self);
    // return 1;
  }
}

pub trait QAccessibleApplication_parent {
  fn parent(self, rsthis: &mut QAccessibleApplication) -> QAccessibleInterface;
}

// proto:  QAccessibleInterface * QAccessibleApplication::parent();
impl<'a> /*trait*/ QAccessibleApplication_parent for () {
  fn parent(self, rsthis: &mut QAccessibleApplication) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QAccessibleApplication6parentEv()};
    let mut ret = unsafe {_ZNK22QAccessibleApplication6parentEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleApplication {
  pub fn focusChild<T: QAccessibleApplication_focusChild>(&mut self, value: T) -> QAccessibleInterface {
    return value.focusChild(self);
    // return 1;
  }
}

pub trait QAccessibleApplication_focusChild {
  fn focusChild(self, rsthis: &mut QAccessibleApplication) -> QAccessibleInterface;
}

// proto:  QAccessibleInterface * QAccessibleApplication::focusChild();
impl<'a> /*trait*/ QAccessibleApplication_focusChild for () {
  fn focusChild(self, rsthis: &mut QAccessibleApplication) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QAccessibleApplication10focusChildEv()};
    let mut ret = unsafe {_ZNK22QAccessibleApplication10focusChildEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleApplication {
  pub fn indexOfChild<T: QAccessibleApplication_indexOfChild>(&mut self, value: T) -> i32 {
    return value.indexOfChild(self);
    // return 1;
  }
}

pub trait QAccessibleApplication_indexOfChild {
  fn indexOfChild(self, rsthis: &mut QAccessibleApplication) -> i32;
}

// proto:  int QAccessibleApplication::indexOfChild(const QAccessibleInterface * );
impl<'a> /*trait*/ QAccessibleApplication_indexOfChild for (&'a  QAccessibleInterface) {
  fn indexOfChild(self, rsthis: &mut QAccessibleApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QAccessibleApplication12indexOfChildEPK20QAccessibleInterface()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK22QAccessibleApplication12indexOfChildEPK20QAccessibleInterface(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

