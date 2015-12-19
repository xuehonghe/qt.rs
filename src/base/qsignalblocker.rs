// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QSignalBlocker::unblock();
  fn _ZN14QSignalBlocker7unblockEv(qthis: *mut c_void);
  // proto:  void QSignalBlocker::QSignalBlocker(QObject & o);
  fn _ZN14QSignalBlockerC1ER7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSignalBlocker::QSignalBlocker(QObject * o);
  fn _ZN14QSignalBlockerC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSignalBlocker::QSignalBlocker(const QSignalBlocker & );
  fn _ZN14QSignalBlockerC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSignalBlocker::reblock();
  fn _ZN14QSignalBlocker7reblockEv(qthis: *mut c_void);
  // proto:  void QSignalBlocker::~QSignalBlocker();
  fn _ZN14QSignalBlockerD0Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QSignalBlocker)=16
pub struct QSignalBlocker {
  pub qclsinst: *mut c_void,
}

  // proto:  void QSignalBlocker::unblock();
impl /*struct*/ QSignalBlocker {
  pub fn unblock<RetType, T: QSignalBlocker_unblock<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unblock(self);
    // return 1;
  }
}

pub trait QSignalBlocker_unblock<RetType> {
  fn unblock(self , rsthis: &mut QSignalBlocker) -> RetType;
}

  // proto:  void QSignalBlocker::unblock();
impl<'a> /*trait*/ QSignalBlocker_unblock<()> for () {
  fn unblock(self , rsthis: &mut QSignalBlocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSignalBlocker7unblockEv()};
     unsafe {_ZN14QSignalBlocker7unblockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSignalBlocker::QSignalBlocker(QObject & o);
impl /*struct*/ QSignalBlocker {
  pub fn NewQSignalBlocker<T: QSignalBlocker_NewQSignalBlocker>(value: T) -> QSignalBlocker {
    let rsthis = value.NewQSignalBlocker();
    return rsthis;
    // return 1;
  }
}

pub trait QSignalBlocker_NewQSignalBlocker {
  fn NewQSignalBlocker(self) -> QSignalBlocker;
}

  // proto:  void QSignalBlocker::QSignalBlocker(QObject & o);
impl<'a> /*trait*/ QSignalBlocker_NewQSignalBlocker for (QObject) {
  fn NewQSignalBlocker(self) -> QSignalBlocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSignalBlockerC1ER7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QSignalBlockerC1ER7QObject(qthis, arg0)};
    let rsthis = QSignalBlocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSignalBlocker::QSignalBlocker(const QSignalBlocker & );
impl<'a> /*trait*/ QSignalBlocker_NewQSignalBlocker for (QSignalBlocker) {
  fn NewQSignalBlocker(self) -> QSignalBlocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSignalBlockerC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QSignalBlockerC1ERKS_(qthis, arg0)};
    let rsthis = QSignalBlocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSignalBlocker::reblock();
impl /*struct*/ QSignalBlocker {
  pub fn reblock<RetType, T: QSignalBlocker_reblock<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.reblock(self);
    // return 1;
  }
}

pub trait QSignalBlocker_reblock<RetType> {
  fn reblock(self , rsthis: &mut QSignalBlocker) -> RetType;
}

  // proto:  void QSignalBlocker::reblock();
impl<'a> /*trait*/ QSignalBlocker_reblock<()> for () {
  fn reblock(self , rsthis: &mut QSignalBlocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSignalBlocker7reblockEv()};
     unsafe {_ZN14QSignalBlocker7reblockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSignalBlocker::~QSignalBlocker();
impl /*struct*/ QSignalBlocker {
  pub fn FreeQSignalBlocker<RetType, T: QSignalBlocker_FreeQSignalBlocker<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQSignalBlocker(self);
    // return 1;
  }
}

pub trait QSignalBlocker_FreeQSignalBlocker<RetType> {
  fn FreeQSignalBlocker(self , rsthis: &mut QSignalBlocker) -> RetType;
}

  // proto:  void QSignalBlocker::~QSignalBlocker();
impl<'a> /*trait*/ QSignalBlocker_FreeQSignalBlocker<()> for () {
  fn FreeQSignalBlocker(self , rsthis: &mut QSignalBlocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSignalBlockerD0Ev()};
     unsafe {_ZN14QSignalBlockerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

