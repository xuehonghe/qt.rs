// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtCore/qregularexpression.h
// dst-file: /src/core/qregularexpression.rs
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
use std::ops::Deref;
// use super::qregularexpression::QRegularExpressionMatch; // 773
// use super::qregularexpression::QRegularExpression; // 773
use super::qstring::*; // 773
use super::qstringlist::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QRegularExpressionMatchIterator_Class_Size() -> c_int;
  // proto:  bool QRegularExpressionMatchIterator::hasNext();
  fn C_ZNK31QRegularExpressionMatchIterator7hasNextEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QRegularExpressionMatchIterator::isValid();
  fn C_ZNK31QRegularExpressionMatchIterator7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QRegularExpressionMatch QRegularExpressionMatchIterator::peekNext();
  fn C_ZNK31QRegularExpressionMatchIterator8peekNextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QRegularExpressionMatchIterator::QRegularExpressionMatchIterator();
  fn C_ZN31QRegularExpressionMatchIteratorC2Ev() -> u64;
  // proto:  QRegularExpression QRegularExpressionMatchIterator::regularExpression();
  fn C_ZNK31QRegularExpressionMatchIterator17regularExpressionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QRegularExpressionMatchIterator::QRegularExpressionMatchIterator(const QRegularExpressionMatchIterator & iterator);
  fn C_ZN31QRegularExpressionMatchIteratorC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  void QRegularExpressionMatchIterator::~QRegularExpressionMatchIterator();
  fn C_ZN31QRegularExpressionMatchIteratorD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QRegularExpressionMatch QRegularExpressionMatchIterator::next();
  fn C_ZN31QRegularExpressionMatchIterator4nextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QRegularExpressionMatchIterator::swap(QRegularExpressionMatchIterator & other);
  fn C_ZN31QRegularExpressionMatchIterator4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QRegularExpression_Class_Size() -> c_int;
  // proto:  int QRegularExpression::patternErrorOffset();
  fn C_ZNK18QRegularExpression18patternErrorOffsetEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QString QRegularExpression::pattern();
  fn C_ZNK18QRegularExpression7patternEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QRegularExpression::~QRegularExpression();
  fn C_ZN18QRegularExpressionD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QRegularExpression::optimize();
  fn C_ZNK18QRegularExpression8optimizeEv(qthis: u64 /* *mut c_void*/);
  // proto: static QString QRegularExpression::escape(const QString & str);
  fn C_ZN18QRegularExpression6escapeERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QRegularExpression::QRegularExpression();
  fn C_ZN18QRegularExpressionC2Ev() -> u64;
  // proto:  void QRegularExpression::swap(QRegularExpression & other);
  fn C_ZN18QRegularExpression4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QRegularExpression::errorString();
  fn C_ZNK18QRegularExpression11errorStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QRegularExpression::isValid();
  fn C_ZNK18QRegularExpression7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QRegularExpression::QRegularExpression(const QRegularExpression & re);
  fn C_ZN18QRegularExpressionC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  QStringList QRegularExpression::namedCaptureGroups();
  fn C_ZNK18QRegularExpression18namedCaptureGroupsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QRegularExpression::captureCount();
  fn C_ZNK18QRegularExpression12captureCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QRegularExpression::setPattern(const QString & pattern);
  fn C_ZN18QRegularExpression10setPatternERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QRegularExpressionMatch_Class_Size() -> c_int;
  // proto:  int QRegularExpressionMatch::lastCapturedIndex();
  fn C_ZNK23QRegularExpressionMatch17lastCapturedIndexEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QRegularExpressionMatch::QRegularExpressionMatch();
  fn C_ZN23QRegularExpressionMatchC2Ev() -> u64;
  // proto:  bool QRegularExpressionMatch::isValid();
  fn C_ZNK23QRegularExpressionMatch7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QRegularExpressionMatch::capturedLength(int nth);
  fn C_ZNK23QRegularExpressionMatch14capturedLengthEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  int QRegularExpressionMatch::capturedLength(const QString & name);
  fn C_ZNK23QRegularExpressionMatch14capturedLengthERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  QStringRef QRegularExpressionMatch::capturedRef(int nth);
  fn C_ZNK23QRegularExpressionMatch11capturedRefEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  int QRegularExpressionMatch::capturedEnd(const QString & name);
  fn C_ZNK23QRegularExpressionMatch11capturedEndERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  QString QRegularExpressionMatch::captured(const QString & name);
  fn C_ZNK23QRegularExpressionMatch8capturedERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QStringList QRegularExpressionMatch::capturedTexts();
  fn C_ZNK23QRegularExpressionMatch13capturedTextsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QRegularExpressionMatch::QRegularExpressionMatch(const QRegularExpressionMatch & match);
  fn C_ZN23QRegularExpressionMatchC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  void QRegularExpressionMatch::swap(QRegularExpressionMatch & other);
  fn C_ZN23QRegularExpressionMatch4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QRegularExpressionMatch::~QRegularExpressionMatch();
  fn C_ZN23QRegularExpressionMatchD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QRegularExpressionMatch::capturedEnd(int nth);
  fn C_ZNK23QRegularExpressionMatch11capturedEndEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  QStringRef QRegularExpressionMatch::capturedRef(const QString & name);
  fn C_ZNK23QRegularExpressionMatch11capturedRefERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QRegularExpressionMatch::hasMatch();
  fn C_ZNK23QRegularExpressionMatch8hasMatchEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QRegularExpressionMatch::capturedStart(const QString & name);
  fn C_ZNK23QRegularExpressionMatch13capturedStartERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  QRegularExpression QRegularExpressionMatch::regularExpression();
  fn C_ZNK23QRegularExpressionMatch17regularExpressionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QRegularExpressionMatch::captured(int nth);
  fn C_ZNK23QRegularExpressionMatch8capturedEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  int QRegularExpressionMatch::capturedStart(int nth);
  fn C_ZNK23QRegularExpressionMatch13capturedStartEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  bool QRegularExpressionMatch::hasPartialMatch();
  fn C_ZNK23QRegularExpressionMatch15hasPartialMatchEv(qthis: u64 /* *mut c_void*/) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QRegularExpressionMatchIterator)=1
#[derive(Default)]
pub struct QRegularExpressionMatchIterator {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QRegularExpression)=1
#[derive(Default)]
pub struct QRegularExpression {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QRegularExpressionMatch)=1
#[derive(Default)]
pub struct QRegularExpressionMatch {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QRegularExpressionMatchIterator {
    return QRegularExpressionMatchIterator{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  bool QRegularExpressionMatchIterator::hasNext();
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn hasNext<RetType, T: QRegularExpressionMatchIterator_hasNext<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasNext(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_hasNext<RetType> {
  fn hasNext(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}

  // proto:  bool QRegularExpressionMatchIterator::hasNext();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_hasNext<i8> for () {
  fn hasNext(self , rsthis: & QRegularExpressionMatchIterator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK31QRegularExpressionMatchIterator7hasNextEv()};
    let mut ret = unsafe {C_ZNK31QRegularExpressionMatchIterator7hasNextEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QRegularExpressionMatchIterator::isValid();
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn isValid<RetType, T: QRegularExpressionMatchIterator_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_isValid<RetType> {
  fn isValid(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}

  // proto:  bool QRegularExpressionMatchIterator::isValid();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_isValid<i8> for () {
  fn isValid(self , rsthis: & QRegularExpressionMatchIterator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK31QRegularExpressionMatchIterator7isValidEv()};
    let mut ret = unsafe {C_ZNK31QRegularExpressionMatchIterator7isValidEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QRegularExpressionMatch QRegularExpressionMatchIterator::peekNext();
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn peekNext<RetType, T: QRegularExpressionMatchIterator_peekNext<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.peekNext(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_peekNext<RetType> {
  fn peekNext(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}

  // proto:  QRegularExpressionMatch QRegularExpressionMatchIterator::peekNext();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_peekNext<QRegularExpressionMatch> for () {
  fn peekNext(self , rsthis: & QRegularExpressionMatchIterator) -> QRegularExpressionMatch {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK31QRegularExpressionMatchIterator8peekNextEv()};
    let mut ret = unsafe {C_ZNK31QRegularExpressionMatchIterator8peekNextEv(rsthis.qclsinst)};
    let mut ret1 = QRegularExpressionMatch::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegularExpressionMatchIterator::QRegularExpressionMatchIterator();
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn new<T: QRegularExpressionMatchIterator_new>(value: T) -> QRegularExpressionMatchIterator {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_new {
  fn new(self) -> QRegularExpressionMatchIterator;
}

  // proto:  void QRegularExpressionMatchIterator::QRegularExpressionMatchIterator();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_new for () {
  fn new(self) -> QRegularExpressionMatchIterator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QRegularExpressionMatchIteratorC2Ev()};
    let ctysz: c_int = unsafe{QRegularExpressionMatchIterator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN31QRegularExpressionMatchIteratorC2Ev()};
    let rsthis = QRegularExpressionMatchIterator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QRegularExpression QRegularExpressionMatchIterator::regularExpression();
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn regularExpression<RetType, T: QRegularExpressionMatchIterator_regularExpression<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.regularExpression(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_regularExpression<RetType> {
  fn regularExpression(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}

  // proto:  QRegularExpression QRegularExpressionMatchIterator::regularExpression();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_regularExpression<QRegularExpression> for () {
  fn regularExpression(self , rsthis: & QRegularExpressionMatchIterator) -> QRegularExpression {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK31QRegularExpressionMatchIterator17regularExpressionEv()};
    let mut ret = unsafe {C_ZNK31QRegularExpressionMatchIterator17regularExpressionEv(rsthis.qclsinst)};
    let mut ret1 = QRegularExpression::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegularExpressionMatchIterator::QRegularExpressionMatchIterator(const QRegularExpressionMatchIterator & iterator);
impl<'a> /*trait*/ QRegularExpressionMatchIterator_new for (&'a QRegularExpressionMatchIterator) {
  fn new(self) -> QRegularExpressionMatchIterator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QRegularExpressionMatchIteratorC2ERKS_()};
    let ctysz: c_int = unsafe{QRegularExpressionMatchIterator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN31QRegularExpressionMatchIteratorC2ERKS_(arg0)};
    let rsthis = QRegularExpressionMatchIterator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRegularExpressionMatchIterator::~QRegularExpressionMatchIterator();
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn free<RetType, T: QRegularExpressionMatchIterator_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_free<RetType> {
  fn free(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}

  // proto:  void QRegularExpressionMatchIterator::~QRegularExpressionMatchIterator();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_free<()> for () {
  fn free(self , rsthis: & QRegularExpressionMatchIterator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QRegularExpressionMatchIteratorD2Ev()};
     unsafe {C_ZN31QRegularExpressionMatchIteratorD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRegularExpressionMatch QRegularExpressionMatchIterator::next();
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn next<RetType, T: QRegularExpressionMatchIterator_next<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.next(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_next<RetType> {
  fn next(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}

  // proto:  QRegularExpressionMatch QRegularExpressionMatchIterator::next();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_next<QRegularExpressionMatch> for () {
  fn next(self , rsthis: & QRegularExpressionMatchIterator) -> QRegularExpressionMatch {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QRegularExpressionMatchIterator4nextEv()};
    let mut ret = unsafe {C_ZN31QRegularExpressionMatchIterator4nextEv(rsthis.qclsinst)};
    let mut ret1 = QRegularExpressionMatch::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegularExpressionMatchIterator::swap(QRegularExpressionMatchIterator & other);
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn swap<RetType, T: QRegularExpressionMatchIterator_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_swap<RetType> {
  fn swap(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}

  // proto:  void QRegularExpressionMatchIterator::swap(QRegularExpressionMatchIterator & other);
impl<'a> /*trait*/ QRegularExpressionMatchIterator_swap<()> for (&'a QRegularExpressionMatchIterator) {
  fn swap(self , rsthis: & QRegularExpressionMatchIterator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QRegularExpressionMatchIterator4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN31QRegularExpressionMatchIterator4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QRegularExpression {
    return QRegularExpression{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  int QRegularExpression::patternErrorOffset();
impl /*struct*/ QRegularExpression {
  pub fn patternErrorOffset<RetType, T: QRegularExpression_patternErrorOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.patternErrorOffset(self);
    // return 1;
  }
}

pub trait QRegularExpression_patternErrorOffset<RetType> {
  fn patternErrorOffset(self , rsthis: & QRegularExpression) -> RetType;
}

  // proto:  int QRegularExpression::patternErrorOffset();
impl<'a> /*trait*/ QRegularExpression_patternErrorOffset<i32> for () {
  fn patternErrorOffset(self , rsthis: & QRegularExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression18patternErrorOffsetEv()};
    let mut ret = unsafe {C_ZNK18QRegularExpression18patternErrorOffsetEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QString QRegularExpression::pattern();
impl /*struct*/ QRegularExpression {
  pub fn pattern<RetType, T: QRegularExpression_pattern<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pattern(self);
    // return 1;
  }
}

pub trait QRegularExpression_pattern<RetType> {
  fn pattern(self , rsthis: & QRegularExpression) -> RetType;
}

  // proto:  QString QRegularExpression::pattern();
impl<'a> /*trait*/ QRegularExpression_pattern<QString> for () {
  fn pattern(self , rsthis: & QRegularExpression) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression7patternEv()};
    let mut ret = unsafe {C_ZNK18QRegularExpression7patternEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegularExpression::~QRegularExpression();
impl /*struct*/ QRegularExpression {
  pub fn free<RetType, T: QRegularExpression_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QRegularExpression_free<RetType> {
  fn free(self , rsthis: & QRegularExpression) -> RetType;
}

  // proto:  void QRegularExpression::~QRegularExpression();
impl<'a> /*trait*/ QRegularExpression_free<()> for () {
  fn free(self , rsthis: & QRegularExpression) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpressionD2Ev()};
     unsafe {C_ZN18QRegularExpressionD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QRegularExpression::optimize();
impl /*struct*/ QRegularExpression {
  pub fn optimize<RetType, T: QRegularExpression_optimize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.optimize(self);
    // return 1;
  }
}

pub trait QRegularExpression_optimize<RetType> {
  fn optimize(self , rsthis: & QRegularExpression) -> RetType;
}

  // proto:  void QRegularExpression::optimize();
impl<'a> /*trait*/ QRegularExpression_optimize<()> for () {
  fn optimize(self , rsthis: & QRegularExpression) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression8optimizeEv()};
     unsafe {C_ZNK18QRegularExpression8optimizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QString QRegularExpression::escape(const QString & str);
impl /*struct*/ QRegularExpression {
  pub fn escape_s<RetType, T: QRegularExpression_escape_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.escape_s();
    // return 1;
  }
}

pub trait QRegularExpression_escape_s<RetType> {
  fn escape_s(self ) -> RetType;
}

  // proto: static QString QRegularExpression::escape(const QString & str);
impl<'a> /*trait*/ QRegularExpression_escape_s<QString> for (&'a QString) {
  fn escape_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpression6escapeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN18QRegularExpression6escapeERK7QString(arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegularExpression::QRegularExpression();
impl /*struct*/ QRegularExpression {
  pub fn new<T: QRegularExpression_new>(value: T) -> QRegularExpression {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QRegularExpression_new {
  fn new(self) -> QRegularExpression;
}

  // proto:  void QRegularExpression::QRegularExpression();
impl<'a> /*trait*/ QRegularExpression_new for () {
  fn new(self) -> QRegularExpression {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpressionC2Ev()};
    let ctysz: c_int = unsafe{QRegularExpression_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN18QRegularExpressionC2Ev()};
    let rsthis = QRegularExpression{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRegularExpression::swap(QRegularExpression & other);
impl /*struct*/ QRegularExpression {
  pub fn swap<RetType, T: QRegularExpression_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QRegularExpression_swap<RetType> {
  fn swap(self , rsthis: & QRegularExpression) -> RetType;
}

  // proto:  void QRegularExpression::swap(QRegularExpression & other);
impl<'a> /*trait*/ QRegularExpression_swap<()> for (&'a QRegularExpression) {
  fn swap(self , rsthis: & QRegularExpression) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpression4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN18QRegularExpression4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QRegularExpression::errorString();
impl /*struct*/ QRegularExpression {
  pub fn errorString<RetType, T: QRegularExpression_errorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QRegularExpression_errorString<RetType> {
  fn errorString(self , rsthis: & QRegularExpression) -> RetType;
}

  // proto:  QString QRegularExpression::errorString();
impl<'a> /*trait*/ QRegularExpression_errorString<QString> for () {
  fn errorString(self , rsthis: & QRegularExpression) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression11errorStringEv()};
    let mut ret = unsafe {C_ZNK18QRegularExpression11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QRegularExpression::isValid();
impl /*struct*/ QRegularExpression {
  pub fn isValid<RetType, T: QRegularExpression_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QRegularExpression_isValid<RetType> {
  fn isValid(self , rsthis: & QRegularExpression) -> RetType;
}

  // proto:  bool QRegularExpression::isValid();
impl<'a> /*trait*/ QRegularExpression_isValid<i8> for () {
  fn isValid(self , rsthis: & QRegularExpression) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression7isValidEv()};
    let mut ret = unsafe {C_ZNK18QRegularExpression7isValidEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QRegularExpression::QRegularExpression(const QRegularExpression & re);
impl<'a> /*trait*/ QRegularExpression_new for (&'a QRegularExpression) {
  fn new(self) -> QRegularExpression {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpressionC2ERKS_()};
    let ctysz: c_int = unsafe{QRegularExpression_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN18QRegularExpressionC2ERKS_(arg0)};
    let rsthis = QRegularExpression{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringList QRegularExpression::namedCaptureGroups();
impl /*struct*/ QRegularExpression {
  pub fn namedCaptureGroups<RetType, T: QRegularExpression_namedCaptureGroups<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.namedCaptureGroups(self);
    // return 1;
  }
}

pub trait QRegularExpression_namedCaptureGroups<RetType> {
  fn namedCaptureGroups(self , rsthis: & QRegularExpression) -> RetType;
}

  // proto:  QStringList QRegularExpression::namedCaptureGroups();
impl<'a> /*trait*/ QRegularExpression_namedCaptureGroups<QStringList> for () {
  fn namedCaptureGroups(self , rsthis: & QRegularExpression) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression18namedCaptureGroupsEv()};
    let mut ret = unsafe {C_ZNK18QRegularExpression18namedCaptureGroupsEv(rsthis.qclsinst)};
    let mut ret1 = QStringList::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QRegularExpression::captureCount();
impl /*struct*/ QRegularExpression {
  pub fn captureCount<RetType, T: QRegularExpression_captureCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.captureCount(self);
    // return 1;
  }
}

pub trait QRegularExpression_captureCount<RetType> {
  fn captureCount(self , rsthis: & QRegularExpression) -> RetType;
}

  // proto:  int QRegularExpression::captureCount();
impl<'a> /*trait*/ QRegularExpression_captureCount<i32> for () {
  fn captureCount(self , rsthis: & QRegularExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression12captureCountEv()};
    let mut ret = unsafe {C_ZNK18QRegularExpression12captureCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QRegularExpression::setPattern(const QString & pattern);
impl /*struct*/ QRegularExpression {
  pub fn setPattern<RetType, T: QRegularExpression_setPattern<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPattern(self);
    // return 1;
  }
}

pub trait QRegularExpression_setPattern<RetType> {
  fn setPattern(self , rsthis: & QRegularExpression) -> RetType;
}

  // proto:  void QRegularExpression::setPattern(const QString & pattern);
impl<'a> /*trait*/ QRegularExpression_setPattern<()> for (&'a QString) {
  fn setPattern(self , rsthis: & QRegularExpression) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpression10setPatternERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN18QRegularExpression10setPatternERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRegularExpressionMatch {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QRegularExpressionMatch {
    return QRegularExpressionMatch{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  int QRegularExpressionMatch::lastCapturedIndex();
impl /*struct*/ QRegularExpressionMatch {
  pub fn lastCapturedIndex<RetType, T: QRegularExpressionMatch_lastCapturedIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastCapturedIndex(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_lastCapturedIndex<RetType> {
  fn lastCapturedIndex(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  int QRegularExpressionMatch::lastCapturedIndex();
impl<'a> /*trait*/ QRegularExpressionMatch_lastCapturedIndex<i32> for () {
  fn lastCapturedIndex(self , rsthis: & QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch17lastCapturedIndexEv()};
    let mut ret = unsafe {C_ZNK23QRegularExpressionMatch17lastCapturedIndexEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QRegularExpressionMatch::QRegularExpressionMatch();
impl /*struct*/ QRegularExpressionMatch {
  pub fn new<T: QRegularExpressionMatch_new>(value: T) -> QRegularExpressionMatch {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QRegularExpressionMatch_new {
  fn new(self) -> QRegularExpressionMatch;
}

  // proto:  void QRegularExpressionMatch::QRegularExpressionMatch();
impl<'a> /*trait*/ QRegularExpressionMatch_new for () {
  fn new(self) -> QRegularExpressionMatch {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QRegularExpressionMatchC2Ev()};
    let ctysz: c_int = unsafe{QRegularExpressionMatch_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN23QRegularExpressionMatchC2Ev()};
    let rsthis = QRegularExpressionMatch{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QRegularExpressionMatch::isValid();
impl /*struct*/ QRegularExpressionMatch {
  pub fn isValid<RetType, T: QRegularExpressionMatch_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_isValid<RetType> {
  fn isValid(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  bool QRegularExpressionMatch::isValid();
impl<'a> /*trait*/ QRegularExpressionMatch_isValid<i8> for () {
  fn isValid(self , rsthis: & QRegularExpressionMatch) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch7isValidEv()};
    let mut ret = unsafe {C_ZNK23QRegularExpressionMatch7isValidEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QRegularExpressionMatch::capturedLength(int nth);
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedLength<RetType, T: QRegularExpressionMatch_capturedLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.capturedLength(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_capturedLength<RetType> {
  fn capturedLength(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  int QRegularExpressionMatch::capturedLength(int nth);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedLength<i32> for (Option<i32>) {
  fn capturedLength(self , rsthis: & QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch14capturedLengthEi()};
    let arg0 = (if self.is_none() {0} else {self.unwrap()})  as c_int;
    let mut ret = unsafe {C_ZNK23QRegularExpressionMatch14capturedLengthEi(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QRegularExpressionMatch::capturedLength(const QString & name);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedLength<i32> for (&'a QString) {
  fn capturedLength(self , rsthis: & QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch14capturedLengthERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK23QRegularExpressionMatch14capturedLengthERK7QString(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QStringRef QRegularExpressionMatch::capturedRef(int nth);
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedRef<RetType, T: QRegularExpressionMatch_capturedRef<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.capturedRef(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_capturedRef<RetType> {
  fn capturedRef(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  QStringRef QRegularExpressionMatch::capturedRef(int nth);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedRef<QStringRef> for (Option<i32>) {
  fn capturedRef(self , rsthis: & QRegularExpressionMatch) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch11capturedRefEi()};
    let arg0 = (if self.is_none() {0} else {self.unwrap()})  as c_int;
    let mut ret = unsafe {C_ZNK23QRegularExpressionMatch11capturedRefEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QRegularExpressionMatch::capturedEnd(const QString & name);
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedEnd<RetType, T: QRegularExpressionMatch_capturedEnd<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.capturedEnd(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_capturedEnd<RetType> {
  fn capturedEnd(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  int QRegularExpressionMatch::capturedEnd(const QString & name);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedEnd<i32> for (&'a QString) {
  fn capturedEnd(self , rsthis: & QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch11capturedEndERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK23QRegularExpressionMatch11capturedEndERK7QString(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QString QRegularExpressionMatch::captured(const QString & name);
impl /*struct*/ QRegularExpressionMatch {
  pub fn captured<RetType, T: QRegularExpressionMatch_captured<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.captured(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_captured<RetType> {
  fn captured(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  QString QRegularExpressionMatch::captured(const QString & name);
impl<'a> /*trait*/ QRegularExpressionMatch_captured<QString> for (&'a QString) {
  fn captured(self , rsthis: & QRegularExpressionMatch) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch8capturedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK23QRegularExpressionMatch8capturedERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringList QRegularExpressionMatch::capturedTexts();
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedTexts<RetType, T: QRegularExpressionMatch_capturedTexts<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.capturedTexts(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_capturedTexts<RetType> {
  fn capturedTexts(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  QStringList QRegularExpressionMatch::capturedTexts();
impl<'a> /*trait*/ QRegularExpressionMatch_capturedTexts<QStringList> for () {
  fn capturedTexts(self , rsthis: & QRegularExpressionMatch) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch13capturedTextsEv()};
    let mut ret = unsafe {C_ZNK23QRegularExpressionMatch13capturedTextsEv(rsthis.qclsinst)};
    let mut ret1 = QStringList::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegularExpressionMatch::QRegularExpressionMatch(const QRegularExpressionMatch & match);
impl<'a> /*trait*/ QRegularExpressionMatch_new for (&'a QRegularExpressionMatch) {
  fn new(self) -> QRegularExpressionMatch {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QRegularExpressionMatchC2ERKS_()};
    let ctysz: c_int = unsafe{QRegularExpressionMatch_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN23QRegularExpressionMatchC2ERKS_(arg0)};
    let rsthis = QRegularExpressionMatch{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRegularExpressionMatch::swap(QRegularExpressionMatch & other);
impl /*struct*/ QRegularExpressionMatch {
  pub fn swap<RetType, T: QRegularExpressionMatch_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_swap<RetType> {
  fn swap(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  void QRegularExpressionMatch::swap(QRegularExpressionMatch & other);
impl<'a> /*trait*/ QRegularExpressionMatch_swap<()> for (&'a QRegularExpressionMatch) {
  fn swap(self , rsthis: & QRegularExpressionMatch) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QRegularExpressionMatch4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN23QRegularExpressionMatch4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRegularExpressionMatch::~QRegularExpressionMatch();
impl /*struct*/ QRegularExpressionMatch {
  pub fn free<RetType, T: QRegularExpressionMatch_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_free<RetType> {
  fn free(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  void QRegularExpressionMatch::~QRegularExpressionMatch();
impl<'a> /*trait*/ QRegularExpressionMatch_free<()> for () {
  fn free(self , rsthis: & QRegularExpressionMatch) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QRegularExpressionMatchD2Ev()};
     unsafe {C_ZN23QRegularExpressionMatchD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QRegularExpressionMatch::capturedEnd(int nth);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedEnd<i32> for (Option<i32>) {
  fn capturedEnd(self , rsthis: & QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch11capturedEndEi()};
    let arg0 = (if self.is_none() {0} else {self.unwrap()})  as c_int;
    let mut ret = unsafe {C_ZNK23QRegularExpressionMatch11capturedEndEi(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QStringRef QRegularExpressionMatch::capturedRef(const QString & name);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedRef<QStringRef> for (&'a QString) {
  fn capturedRef(self , rsthis: & QRegularExpressionMatch) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch11capturedRefERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK23QRegularExpressionMatch11capturedRefERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QRegularExpressionMatch::hasMatch();
impl /*struct*/ QRegularExpressionMatch {
  pub fn hasMatch<RetType, T: QRegularExpressionMatch_hasMatch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasMatch(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_hasMatch<RetType> {
  fn hasMatch(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  bool QRegularExpressionMatch::hasMatch();
impl<'a> /*trait*/ QRegularExpressionMatch_hasMatch<i8> for () {
  fn hasMatch(self , rsthis: & QRegularExpressionMatch) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch8hasMatchEv()};
    let mut ret = unsafe {C_ZNK23QRegularExpressionMatch8hasMatchEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QRegularExpressionMatch::capturedStart(const QString & name);
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedStart<RetType, T: QRegularExpressionMatch_capturedStart<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.capturedStart(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_capturedStart<RetType> {
  fn capturedStart(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  int QRegularExpressionMatch::capturedStart(const QString & name);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedStart<i32> for (&'a QString) {
  fn capturedStart(self , rsthis: & QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch13capturedStartERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK23QRegularExpressionMatch13capturedStartERK7QString(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QRegularExpression QRegularExpressionMatch::regularExpression();
impl /*struct*/ QRegularExpressionMatch {
  pub fn regularExpression<RetType, T: QRegularExpressionMatch_regularExpression<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.regularExpression(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_regularExpression<RetType> {
  fn regularExpression(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  QRegularExpression QRegularExpressionMatch::regularExpression();
impl<'a> /*trait*/ QRegularExpressionMatch_regularExpression<QRegularExpression> for () {
  fn regularExpression(self , rsthis: & QRegularExpressionMatch) -> QRegularExpression {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch17regularExpressionEv()};
    let mut ret = unsafe {C_ZNK23QRegularExpressionMatch17regularExpressionEv(rsthis.qclsinst)};
    let mut ret1 = QRegularExpression::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QRegularExpressionMatch::captured(int nth);
impl<'a> /*trait*/ QRegularExpressionMatch_captured<QString> for (Option<i32>) {
  fn captured(self , rsthis: & QRegularExpressionMatch) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch8capturedEi()};
    let arg0 = (if self.is_none() {0} else {self.unwrap()})  as c_int;
    let mut ret = unsafe {C_ZNK23QRegularExpressionMatch8capturedEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QRegularExpressionMatch::capturedStart(int nth);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedStart<i32> for (Option<i32>) {
  fn capturedStart(self , rsthis: & QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch13capturedStartEi()};
    let arg0 = (if self.is_none() {0} else {self.unwrap()})  as c_int;
    let mut ret = unsafe {C_ZNK23QRegularExpressionMatch13capturedStartEi(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QRegularExpressionMatch::hasPartialMatch();
impl /*struct*/ QRegularExpressionMatch {
  pub fn hasPartialMatch<RetType, T: QRegularExpressionMatch_hasPartialMatch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasPartialMatch(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_hasPartialMatch<RetType> {
  fn hasPartialMatch(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  bool QRegularExpressionMatch::hasPartialMatch();
impl<'a> /*trait*/ QRegularExpressionMatch_hasPartialMatch<i8> for () {
  fn hasPartialMatch(self , rsthis: & QRegularExpressionMatch) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch15hasPartialMatchEv()};
    let mut ret = unsafe {C_ZNK23QRegularExpressionMatch15hasPartialMatchEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

// <= body block end

