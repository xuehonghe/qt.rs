// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qstyle::QStyle;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: static QStyle * QStyleFactory::create(const QString & );
  fn _ZN13QStyleFactory6createERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto: static QStringList QStyleFactory::keys();
  fn _ZN13QStyleFactory4keysEv();
}

// body block begin
// class sizeof(QStyleFactory)=1
pub struct QStyleFactory {
  pub qclsinst: *mut c_void,
}

  // proto: static QStyle * QStyleFactory::create(const QString & );
impl /*struct*/ QStyleFactory {
  pub fn create_s<RetType, T: QStyleFactory_create_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.create_s();
    // return 1;
  }
}

pub trait QStyleFactory_create_s<RetType> {
  fn create_s(self ) -> RetType;
}

  // proto: static QStyle * QStyleFactory::create(const QString & );
impl<'a> /*trait*/ QStyleFactory_create_s<QStyle> for (QString) {
  fn create_s(self ) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStyleFactory6createERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QStyleFactory6createERK7QString(arg0)};
    let mut ret1 = QStyle{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static QStringList QStyleFactory::keys();
impl /*struct*/ QStyleFactory {
  pub fn keys_s<RetType, T: QStyleFactory_keys_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.keys_s();
    // return 1;
  }
}

pub trait QStyleFactory_keys_s<RetType> {
  fn keys_s(self ) -> RetType;
}

  // proto: static QStringList QStyleFactory::keys();
impl<'a> /*trait*/ QStyleFactory_keys_s<()> for () {
  fn keys_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStyleFactory4keysEv()};
     unsafe {_ZN13QStyleFactory4keysEv()};
    // return 1;
  }
}
