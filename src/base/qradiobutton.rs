// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qsize::QSize;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  const QMetaObject * QRadioButton::metaObject();
  fn _ZNK12QRadioButton10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QRadioButton::NewQRadioButton(QWidget * parent);
  fn _ZN12QRadioButtonC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSize QRadioButton::sizeHint();
  fn _ZNK12QRadioButton8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QRadioButton::minimumSizeHint();
  fn _ZNK12QRadioButton15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRadioButton::FreeQRadioButton();
  fn _ZN12QRadioButtonD0Ev(qthis: *mut c_void) ;
  // proto:  void QRadioButton::NewQRadioButton(const QRadioButton & );
  fn _ZN12QRadioButtonC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRadioButton::NewQRadioButton(const QString & text, QWidget * parent);
  fn _ZN12QRadioButtonC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
}

// body block begin
// class sizeof(QRadioButton)=1
pub struct QRadioButton {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRadioButton {
  pub fn metaObject<T: QRadioButton_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QRadioButton_metaObject {
  fn metaObject(self, rsthis: &mut QRadioButton) ;
}

// proto:  const QMetaObject * QRadioButton::metaObject();
impl<'a> /*trait*/ QRadioButton_metaObject for () {
  fn metaObject(self, rsthis: &mut QRadioButton)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QRadioButton10metaObjectEv()};
     unsafe {_ZNK12QRadioButton10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QRadioButton {
  pub fn NewQRadioButton<T: QRadioButton_NewQRadioButton>(value: T) -> QRadioButton {
    let rsthis = value.NewQRadioButton();
    return rsthis;
    // return 1;
  }
}

pub trait QRadioButton_NewQRadioButton {
  fn NewQRadioButton(self) -> QRadioButton;
}

// proto: void QRadioButton::NewQRadioButton(QWidget * parent);
impl<'a> /*trait*/ QRadioButton_NewQRadioButton for (&'a mut QWidget) {
  fn NewQRadioButton(self) -> QRadioButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QRadioButtonC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QRadioButtonC1EP7QWidget(qthis, arg0)};
    let rsthis = QRadioButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRadioButton {
  pub fn sizeHint<T: QRadioButton_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QRadioButton_sizeHint {
  fn sizeHint(self, rsthis: &mut QRadioButton) -> QSize;
}

// proto:  QSize QRadioButton::sizeHint();
impl<'a> /*trait*/ QRadioButton_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QRadioButton) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QRadioButton8sizeHintEv()};
    let mut ret = unsafe {_ZNK12QRadioButton8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRadioButton {
  pub fn minimumSizeHint<T: QRadioButton_minimumSizeHint>(&mut self, value: T) -> QSize {
    return value.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QRadioButton_minimumSizeHint {
  fn minimumSizeHint(self, rsthis: &mut QRadioButton) -> QSize;
}

// proto:  QSize QRadioButton::minimumSizeHint();
impl<'a> /*trait*/ QRadioButton_minimumSizeHint for () {
  fn minimumSizeHint(self, rsthis: &mut QRadioButton) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QRadioButton15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK12QRadioButton15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRadioButton {
  pub fn FreeQRadioButton<T: QRadioButton_FreeQRadioButton>(&mut self, value: T)  {
     value.FreeQRadioButton(self);
    // return 1;
  }
}

pub trait QRadioButton_FreeQRadioButton {
  fn FreeQRadioButton(self, rsthis: &mut QRadioButton) ;
}

// proto:  void QRadioButton::FreeQRadioButton();
impl<'a> /*trait*/ QRadioButton_FreeQRadioButton for () {
  fn FreeQRadioButton(self, rsthis: &mut QRadioButton)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QRadioButtonD0Ev()};
     unsafe {_ZN12QRadioButtonD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QRadioButton::NewQRadioButton(const QRadioButton & );
impl<'a> /*trait*/ QRadioButton_NewQRadioButton for (&'a  QRadioButton) {
  fn NewQRadioButton(self) -> QRadioButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QRadioButtonC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QRadioButtonC1ERKS_(qthis, arg0)};
    let rsthis = QRadioButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QRadioButton::NewQRadioButton(const QString & text, QWidget * parent);
impl<'a> /*trait*/ QRadioButton_NewQRadioButton for (&'a  QString, &'a mut QWidget) {
  fn NewQRadioButton(self) -> QRadioButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QRadioButtonC1ERK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QRadioButtonC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let rsthis = QRadioButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

