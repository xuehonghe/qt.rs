// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtWidgets/qactiongroup.h
// dst-file: /src/widgets/qactiongroup.rs
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
// use super::qlist::*; // 775
use super::super::core::qobjectdefs::*; // 771
use super::qaction::*; // 773
use super::super::gui::qicon::*; // 771
use super::super::core::qstring::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QActionGroup_Class_Size() -> c_int;
  // proto:  void QActionGroup::QActionGroup(QObject * parent);
  fn C_ZN12QActionGroupC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  QList<QAction *> QActionGroup::actions();
  fn C_ZNK12QActionGroup7actionsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QActionGroup::setDisabled(bool b);
  fn C_ZN12QActionGroup11setDisabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QActionGroup::setEnabled(bool );
  fn C_ZN12QActionGroup10setEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  const QMetaObject * QActionGroup::metaObject();
  fn C_ZNK12QActionGroup10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QAction * QActionGroup::addAction(QAction * a);
  fn C_ZN12QActionGroup9addActionEP7QAction(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QActionGroup::~QActionGroup();
  fn C_ZN12QActionGroupD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QAction * QActionGroup::checkedAction();
  fn C_ZNK12QActionGroup13checkedActionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QAction * QActionGroup::addAction(const QIcon & icon, const QString & text);
  fn C_ZN12QActionGroup9addActionERK5QIconRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QActionGroup::setVisible(bool );
  fn C_ZN12QActionGroup10setVisibleEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QActionGroup::isVisible();
  fn C_ZNK12QActionGroup9isVisibleEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QActionGroup::setExclusive(bool );
  fn C_ZN12QActionGroup12setExclusiveEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QAction * QActionGroup::addAction(const QString & text);
  fn C_ZN12QActionGroup9addActionERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QActionGroup::isEnabled();
  fn C_ZNK12QActionGroup9isEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QActionGroup::isExclusive();
  fn C_ZNK12QActionGroup11isExclusiveEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QActionGroup::removeAction(QAction * a);
  fn C_ZN12QActionGroup12removeActionEP7QAction(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QActionGroup_SlotProxy_connect__ZN12QActionGroup7hoveredEP7QAction(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QActionGroup_SlotProxy_connect__ZN12QActionGroup9triggeredEP7QAction(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QActionGroup)=1
#[derive(Default)]
pub struct QActionGroup {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _triggered: QActionGroup_triggered_signal,
  pub _hovered: QActionGroup_hovered_signal,
}

impl /*struct*/ QActionGroup {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QActionGroup {
    return QActionGroup{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QActionGroup {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QActionGroup {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QActionGroup::QActionGroup(QObject * parent);
impl /*struct*/ QActionGroup {
  pub fn new<T: QActionGroup_new>(value: T) -> QActionGroup {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QActionGroup_new {
  fn new(self) -> QActionGroup;
}

  // proto:  void QActionGroup::QActionGroup(QObject * parent);
impl<'a> /*trait*/ QActionGroup_new for (&'a QObject) {
  fn new(self) -> QActionGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroupC2EP7QObject()};
    let ctysz: c_int = unsafe{QActionGroup_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN12QActionGroupC2EP7QObject(arg0)};
    let rsthis = QActionGroup{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QList<QAction *> QActionGroup::actions();
impl /*struct*/ QActionGroup {
  pub fn actions<RetType, T: QActionGroup_actions<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.actions(self);
    // return 1;
  }
}

pub trait QActionGroup_actions<RetType> {
  fn actions(self , rsthis: & QActionGroup) -> RetType;
}

  // proto:  QList<QAction *> QActionGroup::actions();
impl<'a> /*trait*/ QActionGroup_actions<u64> for () {
  fn actions(self , rsthis: & QActionGroup) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QActionGroup7actionsEv()};
    let mut ret = unsafe {C_ZNK12QActionGroup7actionsEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  void QActionGroup::setDisabled(bool b);
impl /*struct*/ QActionGroup {
  pub fn setDisabled<RetType, T: QActionGroup_setDisabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDisabled(self);
    // return 1;
  }
}

pub trait QActionGroup_setDisabled<RetType> {
  fn setDisabled(self , rsthis: & QActionGroup) -> RetType;
}

  // proto:  void QActionGroup::setDisabled(bool b);
impl<'a> /*trait*/ QActionGroup_setDisabled<()> for (i8) {
  fn setDisabled(self , rsthis: & QActionGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup11setDisabledEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN12QActionGroup11setDisabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QActionGroup::setEnabled(bool );
impl /*struct*/ QActionGroup {
  pub fn setEnabled<RetType, T: QActionGroup_setEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEnabled(self);
    // return 1;
  }
}

pub trait QActionGroup_setEnabled<RetType> {
  fn setEnabled(self , rsthis: & QActionGroup) -> RetType;
}

  // proto:  void QActionGroup::setEnabled(bool );
impl<'a> /*trait*/ QActionGroup_setEnabled<()> for (i8) {
  fn setEnabled(self , rsthis: & QActionGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup10setEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN12QActionGroup10setEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QActionGroup::metaObject();
impl /*struct*/ QActionGroup {
  pub fn metaObject<RetType, T: QActionGroup_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QActionGroup_metaObject<RetType> {
  fn metaObject(self , rsthis: & QActionGroup) -> RetType;
}

  // proto:  const QMetaObject * QActionGroup::metaObject();
impl<'a> /*trait*/ QActionGroup_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QActionGroup) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QActionGroup10metaObjectEv()};
    let mut ret = unsafe {C_ZNK12QActionGroup10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QAction * QActionGroup::addAction(QAction * a);
impl /*struct*/ QActionGroup {
  pub fn addAction<RetType, T: QActionGroup_addAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addAction(self);
    // return 1;
  }
}

pub trait QActionGroup_addAction<RetType> {
  fn addAction(self , rsthis: & QActionGroup) -> RetType;
}

  // proto:  QAction * QActionGroup::addAction(QAction * a);
impl<'a> /*trait*/ QActionGroup_addAction<QAction> for (&'a QAction) {
  fn addAction(self , rsthis: & QActionGroup) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup9addActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN12QActionGroup9addActionEP7QAction(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QActionGroup::~QActionGroup();
impl /*struct*/ QActionGroup {
  pub fn free<RetType, T: QActionGroup_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QActionGroup_free<RetType> {
  fn free(self , rsthis: & QActionGroup) -> RetType;
}

  // proto:  void QActionGroup::~QActionGroup();
impl<'a> /*trait*/ QActionGroup_free<()> for () {
  fn free(self , rsthis: & QActionGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroupD2Ev()};
     unsafe {C_ZN12QActionGroupD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAction * QActionGroup::checkedAction();
impl /*struct*/ QActionGroup {
  pub fn checkedAction<RetType, T: QActionGroup_checkedAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.checkedAction(self);
    // return 1;
  }
}

pub trait QActionGroup_checkedAction<RetType> {
  fn checkedAction(self , rsthis: & QActionGroup) -> RetType;
}

  // proto:  QAction * QActionGroup::checkedAction();
impl<'a> /*trait*/ QActionGroup_checkedAction<QAction> for () {
  fn checkedAction(self , rsthis: & QActionGroup) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QActionGroup13checkedActionEv()};
    let mut ret = unsafe {C_ZNK12QActionGroup13checkedActionEv(rsthis.qclsinst)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QAction * QActionGroup::addAction(const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QActionGroup_addAction<QAction> for (&'a QIcon, &'a QString) {
  fn addAction(self , rsthis: & QActionGroup) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup9addActionERK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN12QActionGroup9addActionERK5QIconRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QActionGroup::setVisible(bool );
impl /*struct*/ QActionGroup {
  pub fn setVisible<RetType, T: QActionGroup_setVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QActionGroup_setVisible<RetType> {
  fn setVisible(self , rsthis: & QActionGroup) -> RetType;
}

  // proto:  void QActionGroup::setVisible(bool );
impl<'a> /*trait*/ QActionGroup_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: & QActionGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN12QActionGroup10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QActionGroup::isVisible();
impl /*struct*/ QActionGroup {
  pub fn isVisible<RetType, T: QActionGroup_isVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isVisible(self);
    // return 1;
  }
}

pub trait QActionGroup_isVisible<RetType> {
  fn isVisible(self , rsthis: & QActionGroup) -> RetType;
}

  // proto:  bool QActionGroup::isVisible();
impl<'a> /*trait*/ QActionGroup_isVisible<i8> for () {
  fn isVisible(self , rsthis: & QActionGroup) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QActionGroup9isVisibleEv()};
    let mut ret = unsafe {C_ZNK12QActionGroup9isVisibleEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QActionGroup::setExclusive(bool );
impl /*struct*/ QActionGroup {
  pub fn setExclusive<RetType, T: QActionGroup_setExclusive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setExclusive(self);
    // return 1;
  }
}

pub trait QActionGroup_setExclusive<RetType> {
  fn setExclusive(self , rsthis: & QActionGroup) -> RetType;
}

  // proto:  void QActionGroup::setExclusive(bool );
impl<'a> /*trait*/ QActionGroup_setExclusive<()> for (i8) {
  fn setExclusive(self , rsthis: & QActionGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup12setExclusiveEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN12QActionGroup12setExclusiveEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAction * QActionGroup::addAction(const QString & text);
impl<'a> /*trait*/ QActionGroup_addAction<QAction> for (&'a QString) {
  fn addAction(self , rsthis: & QActionGroup) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup9addActionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN12QActionGroup9addActionERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QActionGroup::isEnabled();
impl /*struct*/ QActionGroup {
  pub fn isEnabled<RetType, T: QActionGroup_isEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEnabled(self);
    // return 1;
  }
}

pub trait QActionGroup_isEnabled<RetType> {
  fn isEnabled(self , rsthis: & QActionGroup) -> RetType;
}

  // proto:  bool QActionGroup::isEnabled();
impl<'a> /*trait*/ QActionGroup_isEnabled<i8> for () {
  fn isEnabled(self , rsthis: & QActionGroup) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QActionGroup9isEnabledEv()};
    let mut ret = unsafe {C_ZNK12QActionGroup9isEnabledEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QActionGroup::isExclusive();
impl /*struct*/ QActionGroup {
  pub fn isExclusive<RetType, T: QActionGroup_isExclusive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isExclusive(self);
    // return 1;
  }
}

pub trait QActionGroup_isExclusive<RetType> {
  fn isExclusive(self , rsthis: & QActionGroup) -> RetType;
}

  // proto:  bool QActionGroup::isExclusive();
impl<'a> /*trait*/ QActionGroup_isExclusive<i8> for () {
  fn isExclusive(self , rsthis: & QActionGroup) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QActionGroup11isExclusiveEv()};
    let mut ret = unsafe {C_ZNK12QActionGroup11isExclusiveEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QActionGroup::removeAction(QAction * a);
impl /*struct*/ QActionGroup {
  pub fn removeAction<RetType, T: QActionGroup_removeAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeAction(self);
    // return 1;
  }
}

pub trait QActionGroup_removeAction<RetType> {
  fn removeAction(self , rsthis: & QActionGroup) -> RetType;
}

  // proto:  void QActionGroup::removeAction(QAction * a);
impl<'a> /*trait*/ QActionGroup_removeAction<()> for (&'a QAction) {
  fn removeAction(self , rsthis: & QActionGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup12removeActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QActionGroup12removeActionEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QActionGroup_triggered
pub struct QActionGroup_triggered_signal{poi:u64}
impl /* struct */ QActionGroup {
  pub fn triggered(&self) -> QActionGroup_triggered_signal {
     return QActionGroup_triggered_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QActionGroup_triggered_signal {
  pub fn connect<T: QActionGroup_triggered_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QActionGroup_triggered_signal_connect {
  fn connect(self, sigthis: QActionGroup_triggered_signal);
}

#[derive(Default)] // for QActionGroup_hovered
pub struct QActionGroup_hovered_signal{poi:u64}
impl /* struct */ QActionGroup {
  pub fn hovered(&self) -> QActionGroup_hovered_signal {
     return QActionGroup_hovered_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QActionGroup_hovered_signal {
  pub fn connect<T: QActionGroup_hovered_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QActionGroup_hovered_signal_connect {
  fn connect(self, sigthis: QActionGroup_hovered_signal);
}

// hovered(class QAction *)
extern fn QActionGroup_hovered_signal_connect_cb_0(rsfptr:fn(QAction), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QAction::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QActionGroup_hovered_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QAction)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QAction::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QActionGroup_hovered_signal_connect for fn(QAction) {
  fn connect(self, sigthis: QActionGroup_hovered_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QActionGroup_hovered_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QActionGroup_SlotProxy_connect__ZN12QActionGroup7hoveredEP7QAction(arg0, arg1, arg2)};
  }
}
impl /* trait */ QActionGroup_hovered_signal_connect for Box<Fn(QAction)> {
  fn connect(self, sigthis: QActionGroup_hovered_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QActionGroup_hovered_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QActionGroup_SlotProxy_connect__ZN12QActionGroup7hoveredEP7QAction(arg0, arg1, arg2)};
  }
}
// triggered(class QAction *)
extern fn QActionGroup_triggered_signal_connect_cb_1(rsfptr:fn(QAction), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QAction::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QActionGroup_triggered_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(QAction)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QAction::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QActionGroup_triggered_signal_connect for fn(QAction) {
  fn connect(self, sigthis: QActionGroup_triggered_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QActionGroup_triggered_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QActionGroup_SlotProxy_connect__ZN12QActionGroup9triggeredEP7QAction(arg0, arg1, arg2)};
  }
}
impl /* trait */ QActionGroup_triggered_signal_connect for Box<Fn(QAction)> {
  fn connect(self, sigthis: QActionGroup_triggered_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QActionGroup_triggered_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QActionGroup_SlotProxy_connect__ZN12QActionGroup9triggeredEP7QAction(arg0, arg1, arg2)};
  }
}
// <= body block end

