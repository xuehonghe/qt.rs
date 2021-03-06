// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtCore/qsavefile.h
// dst-file: /src/core/qsavefile.rs
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
use super::qfiledevice::*; // 773
use std::ops::Deref;
use super::qobject::*; // 773
use super::qstring::*; // 773
use super::qobjectdefs::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSaveFile_Class_Size() -> c_int;
  // proto:  void QSaveFile::cancelWriting();
  fn C_ZN9QSaveFile13cancelWritingEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QSaveFile::QSaveFile(QObject * parent);
  fn C_ZN9QSaveFileC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  void QSaveFile::QSaveFile(const QString & name, QObject * parent);
  fn C_ZN9QSaveFileC2ERK7QStringP7QObject(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  QString QSaveFile::fileName();
  fn C_ZNK9QSaveFile8fileNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSaveFile::QSaveFile(const QString & name);
  fn C_ZN9QSaveFileC2ERK7QString(arg0: *mut c_void) -> u64;
  // proto:  const QMetaObject * QSaveFile::metaObject();
  fn C_ZNK9QSaveFile10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QSaveFile::commit();
  fn C_ZN9QSaveFile6commitEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QSaveFile::~QSaveFile();
  fn C_ZN9QSaveFileD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSaveFile::setFileName(const QString & name);
  fn C_ZN9QSaveFile11setFileNameERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QSaveFile::directWriteFallback();
  fn C_ZNK9QSaveFile19directWriteFallbackEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QSaveFile::setDirectWriteFallback(bool enabled);
  fn C_ZN9QSaveFile22setDirectWriteFallbackEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
} // <= ext block end

// body block begin =>
// class sizeof(QSaveFile)=1
#[derive(Default)]
pub struct QSaveFile {
  qbase: QFileDevice,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSaveFile {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSaveFile {
    return QSaveFile{qbase: QFileDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSaveFile {
  type Target = QFileDevice;

  fn deref(&self) -> &QFileDevice {
    return & self.qbase;
  }
}
impl AsRef<QFileDevice> for QSaveFile {
  fn as_ref(& self) -> & QFileDevice {
    return & self.qbase;
  }
}
  // proto:  void QSaveFile::cancelWriting();
impl /*struct*/ QSaveFile {
  pub fn cancelWriting<RetType, T: QSaveFile_cancelWriting<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cancelWriting(self);
    // return 1;
  }
}

pub trait QSaveFile_cancelWriting<RetType> {
  fn cancelWriting(self , rsthis: & QSaveFile) -> RetType;
}

  // proto:  void QSaveFile::cancelWriting();
impl<'a> /*trait*/ QSaveFile_cancelWriting<()> for () {
  fn cancelWriting(self , rsthis: & QSaveFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFile13cancelWritingEv()};
     unsafe {C_ZN9QSaveFile13cancelWritingEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSaveFile::QSaveFile(QObject * parent);
impl /*struct*/ QSaveFile {
  pub fn new<T: QSaveFile_new>(value: T) -> QSaveFile {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSaveFile_new {
  fn new(self) -> QSaveFile;
}

  // proto:  void QSaveFile::QSaveFile(QObject * parent);
impl<'a> /*trait*/ QSaveFile_new for (Option<&'a QObject>) {
  fn new(self) -> QSaveFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFileC2EP7QObject()};
    let ctysz: c_int = unsafe{QSaveFile_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QSaveFileC2EP7QObject(arg0)};
    let rsthis = QSaveFile{qbase: QFileDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSaveFile::QSaveFile(const QString & name, QObject * parent);
impl<'a> /*trait*/ QSaveFile_new for (&'a QString, &'a QObject) {
  fn new(self) -> QSaveFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFileC2ERK7QStringP7QObject()};
    let ctysz: c_int = unsafe{QSaveFile_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QSaveFileC2ERK7QStringP7QObject(arg0, arg1)};
    let rsthis = QSaveFile{qbase: QFileDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QSaveFile::fileName();
impl /*struct*/ QSaveFile {
  pub fn fileName<RetType, T: QSaveFile_fileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileName(self);
    // return 1;
  }
}

pub trait QSaveFile_fileName<RetType> {
  fn fileName(self , rsthis: & QSaveFile) -> RetType;
}

  // proto:  QString QSaveFile::fileName();
impl<'a> /*trait*/ QSaveFile_fileName<QString> for () {
  fn fileName(self , rsthis: & QSaveFile) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSaveFile8fileNameEv()};
    let mut ret = unsafe {C_ZNK9QSaveFile8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSaveFile::QSaveFile(const QString & name);
impl<'a> /*trait*/ QSaveFile_new for (&'a QString) {
  fn new(self) -> QSaveFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFileC2ERK7QString()};
    let ctysz: c_int = unsafe{QSaveFile_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QSaveFileC2ERK7QString(arg0)};
    let rsthis = QSaveFile{qbase: QFileDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QSaveFile::metaObject();
impl /*struct*/ QSaveFile {
  pub fn metaObject<RetType, T: QSaveFile_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSaveFile_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSaveFile) -> RetType;
}

  // proto:  const QMetaObject * QSaveFile::metaObject();
impl<'a> /*trait*/ QSaveFile_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QSaveFile) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSaveFile10metaObjectEv()};
    let mut ret = unsafe {C_ZNK9QSaveFile10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QSaveFile::commit();
impl /*struct*/ QSaveFile {
  pub fn commit<RetType, T: QSaveFile_commit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.commit(self);
    // return 1;
  }
}

pub trait QSaveFile_commit<RetType> {
  fn commit(self , rsthis: & QSaveFile) -> RetType;
}

  // proto:  bool QSaveFile::commit();
impl<'a> /*trait*/ QSaveFile_commit<i8> for () {
  fn commit(self , rsthis: & QSaveFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFile6commitEv()};
    let mut ret = unsafe {C_ZN9QSaveFile6commitEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QSaveFile::~QSaveFile();
impl /*struct*/ QSaveFile {
  pub fn free<RetType, T: QSaveFile_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSaveFile_free<RetType> {
  fn free(self , rsthis: & QSaveFile) -> RetType;
}

  // proto:  void QSaveFile::~QSaveFile();
impl<'a> /*trait*/ QSaveFile_free<()> for () {
  fn free(self , rsthis: & QSaveFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFileD2Ev()};
     unsafe {C_ZN9QSaveFileD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSaveFile::setFileName(const QString & name);
impl /*struct*/ QSaveFile {
  pub fn setFileName<RetType, T: QSaveFile_setFileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFileName(self);
    // return 1;
  }
}

pub trait QSaveFile_setFileName<RetType> {
  fn setFileName(self , rsthis: & QSaveFile) -> RetType;
}

  // proto:  void QSaveFile::setFileName(const QString & name);
impl<'a> /*trait*/ QSaveFile_setFileName<()> for (&'a QString) {
  fn setFileName(self , rsthis: & QSaveFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFile11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QSaveFile11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QSaveFile::directWriteFallback();
impl /*struct*/ QSaveFile {
  pub fn directWriteFallback<RetType, T: QSaveFile_directWriteFallback<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.directWriteFallback(self);
    // return 1;
  }
}

pub trait QSaveFile_directWriteFallback<RetType> {
  fn directWriteFallback(self , rsthis: & QSaveFile) -> RetType;
}

  // proto:  bool QSaveFile::directWriteFallback();
impl<'a> /*trait*/ QSaveFile_directWriteFallback<i8> for () {
  fn directWriteFallback(self , rsthis: & QSaveFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSaveFile19directWriteFallbackEv()};
    let mut ret = unsafe {C_ZNK9QSaveFile19directWriteFallbackEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QSaveFile::setDirectWriteFallback(bool enabled);
impl /*struct*/ QSaveFile {
  pub fn setDirectWriteFallback<RetType, T: QSaveFile_setDirectWriteFallback<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDirectWriteFallback(self);
    // return 1;
  }
}

pub trait QSaveFile_setDirectWriteFallback<RetType> {
  fn setDirectWriteFallback(self , rsthis: & QSaveFile) -> RetType;
}

  // proto:  void QSaveFile::setDirectWriteFallback(bool enabled);
impl<'a> /*trait*/ QSaveFile_setDirectWriteFallback<()> for (i8) {
  fn setDirectWriteFallback(self , rsthis: & QSaveFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFile22setDirectWriteFallbackEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN9QSaveFile22setDirectWriteFallbackEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

