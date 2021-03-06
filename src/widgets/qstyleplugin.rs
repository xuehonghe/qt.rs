// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtWidgets/qstyleplugin.h
// dst-file: /src/widgets/qstyleplugin.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::super::core::qobject::*; // 771
use std::ops::Deref;
use super::super::core::qstring::*; // 771
use super::qstyle::*; // 773
use super::super::core::qobjectdefs::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QStylePlugin_Class_Size() -> c_int;
  // proto:  QStyle * QStylePlugin::create(const QString & key);
  fn C_ZN12QStylePlugin6createERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QStylePlugin::metaObject();
  fn C_ZNK12QStylePlugin10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStylePlugin::QStylePlugin(QObject * parent);
  fn C_ZN12QStylePluginC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  void QStylePlugin::~QStylePlugin();
  fn C_ZN12QStylePluginD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QStylePlugin)=1
#[derive(Default)]
pub struct QStylePlugin {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QStylePlugin {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStylePlugin {
    return QStylePlugin{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QStylePlugin {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QStylePlugin {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  QStyle * QStylePlugin::create(const QString & key);
impl /*struct*/ QStylePlugin {
  pub fn create<RetType, T: QStylePlugin_create<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QStylePlugin_create<RetType> {
  fn create(self , rsthis: & QStylePlugin) -> RetType;
}

  // proto:  QStyle * QStylePlugin::create(const QString & key);
impl<'a> /*trait*/ QStylePlugin_create<QStyle> for (&'a QString) {
  fn create(self , rsthis: & QStylePlugin) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStylePlugin6createERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN12QStylePlugin6createERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QStyle::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QStylePlugin::metaObject();
impl /*struct*/ QStylePlugin {
  pub fn metaObject<RetType, T: QStylePlugin_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QStylePlugin_metaObject<RetType> {
  fn metaObject(self , rsthis: & QStylePlugin) -> RetType;
}

  // proto:  const QMetaObject * QStylePlugin::metaObject();
impl<'a> /*trait*/ QStylePlugin_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QStylePlugin) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStylePlugin10metaObjectEv()};
    let mut ret = unsafe {C_ZNK12QStylePlugin10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStylePlugin::QStylePlugin(QObject * parent);
impl /*struct*/ QStylePlugin {
  pub fn new<T: QStylePlugin_new>(value: T) -> QStylePlugin {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStylePlugin_new {
  fn new(self) -> QStylePlugin;
}

  // proto:  void QStylePlugin::QStylePlugin(QObject * parent);
impl<'a> /*trait*/ QStylePlugin_new for (Option<&'a QObject>) {
  fn new(self) -> QStylePlugin {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStylePluginC2EP7QObject()};
    let ctysz: c_int = unsafe{QStylePlugin_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN12QStylePluginC2EP7QObject(arg0)};
    let rsthis = QStylePlugin{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStylePlugin::~QStylePlugin();
impl /*struct*/ QStylePlugin {
  pub fn free<RetType, T: QStylePlugin_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QStylePlugin_free<RetType> {
  fn free(self , rsthis: & QStylePlugin) -> RetType;
}

  // proto:  void QStylePlugin::~QStylePlugin();
impl<'a> /*trait*/ QStylePlugin_free<()> for () {
  fn free(self , rsthis: & QStylePlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStylePluginD2Ev()};
     unsafe {C_ZN12QStylePluginD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

