// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtWidgets/qstyle.h
// dst-file: /src/widgets/qstyle.rs
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
use super::super::gui::qpixmap::*; // 771
use super::qstyleoption::*; // 773
use super::qwidget::*; // 773
use super::super::gui::qpainter::*; // 771
use super::super::core::qpoint::*; // 771
use super::super::core::qrect::*; // 771
use super::super::gui::qpalette::*; // 771
use super::super::gui::qfontmetrics::*; // 771
use super::super::core::qstring::*; // 771
use super::super::core::qsize::*; // 771
use super::super::core::qobjectdefs::*; // 771
use super::qapplication::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QStyle_Class_Size() -> c_int;
  // proto:  void QStyle::unpolish(QWidget * );
  fn C_ZN6QStyle8unpolishEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStyle::~QStyle();
  fn C_ZN6QStyleD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QStyle::polish(QPalette & );
  fn C_ZN6QStyle6polishER8QPalette(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStyle::QStyle();
  fn C_ZN6QStyleC2Ev() -> u64;
  // proto:  QRect QStyle::itemPixmapRect(const QRect & r, int flags, const QPixmap & pixmap);
  fn C_ZNK6QStyle14itemPixmapRectERK5QRectiRK7QPixmap(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  QRect QStyle::itemTextRect(const QFontMetrics & fm, const QRect & r, int flags, bool enabled, const QString & text);
  fn C_ZNK6QStyle12itemTextRectERK12QFontMetricsRK5QRectibRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: c_char, arg4: *mut c_void) -> *mut c_void;
  // proto:  const QStyle * QStyle::proxy();
  fn C_ZNK6QStyle5proxyEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPalette QStyle::standardPalette();
  fn C_ZNK6QStyle15standardPaletteEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QStyle::metaObject();
  fn C_ZNK6QStyle10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStyle::polish(QApplication * );
  fn C_ZN6QStyle6polishEP12QApplication(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStyle::drawItemPixmap(QPainter * painter, const QRect & rect, int alignment, const QPixmap & pixmap);
  fn C_ZNK6QStyle14drawItemPixmapEP8QPainterRK5QRectiRK7QPixmap(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: *mut c_void);
  // proto:  void QStyle::polish(QWidget * );
  fn C_ZN6QStyle6polishEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static int QStyle::sliderPositionFromValue(int min, int max, int val, int space, bool upsideDown);
  fn C_ZN6QStyle23sliderPositionFromValueEiiiib(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_char) -> c_int;
  // proto: static int QStyle::sliderValueFromPosition(int min, int max, int pos, int space, bool upsideDown);
  fn C_ZN6QStyle23sliderValueFromPositionEiiiib(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_char) -> c_int;
  // proto:  void QStyle::unpolish(QApplication * );
  fn C_ZN6QStyle8unpolishEP12QApplication(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QStyle)=1
#[derive(Default)]
pub struct QStyle {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QStyle {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyle {
    return QStyle{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QStyle {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QStyle {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QStyle::unpolish(QWidget * );
impl /*struct*/ QStyle {
  pub fn unpolish<RetType, T: QStyle_unpolish<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unpolish(self);
    // return 1;
  }
}

pub trait QStyle_unpolish<RetType> {
  fn unpolish(self , rsthis: & QStyle) -> RetType;
}

  // proto:  void QStyle::unpolish(QWidget * );
impl<'a> /*trait*/ QStyle_unpolish<()> for (&'a QWidget) {
  fn unpolish(self , rsthis: & QStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyle8unpolishEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN6QStyle8unpolishEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStyle::~QStyle();
impl /*struct*/ QStyle {
  pub fn free<RetType, T: QStyle_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QStyle_free<RetType> {
  fn free(self , rsthis: & QStyle) -> RetType;
}

  // proto:  void QStyle::~QStyle();
impl<'a> /*trait*/ QStyle_free<()> for () {
  fn free(self , rsthis: & QStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyleD2Ev()};
     unsafe {C_ZN6QStyleD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStyle::polish(QPalette & );
impl /*struct*/ QStyle {
  pub fn polish<RetType, T: QStyle_polish<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.polish(self);
    // return 1;
  }
}

pub trait QStyle_polish<RetType> {
  fn polish(self , rsthis: & QStyle) -> RetType;
}

  // proto:  void QStyle::polish(QPalette & );
impl<'a> /*trait*/ QStyle_polish<()> for (&'a QPalette) {
  fn polish(self , rsthis: & QStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyle6polishER8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN6QStyle6polishER8QPalette(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStyle::QStyle();
impl /*struct*/ QStyle {
  pub fn new<T: QStyle_new>(value: T) -> QStyle {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyle_new {
  fn new(self) -> QStyle;
}

  // proto:  void QStyle::QStyle();
impl<'a> /*trait*/ QStyle_new for () {
  fn new(self) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyleC2Ev()};
    let ctysz: c_int = unsafe{QStyle_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN6QStyleC2Ev()};
    let rsthis = QStyle{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QRect QStyle::itemPixmapRect(const QRect & r, int flags, const QPixmap & pixmap);
impl /*struct*/ QStyle {
  pub fn itemPixmapRect<RetType, T: QStyle_itemPixmapRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemPixmapRect(self);
    // return 1;
  }
}

pub trait QStyle_itemPixmapRect<RetType> {
  fn itemPixmapRect(self , rsthis: & QStyle) -> RetType;
}

  // proto:  QRect QStyle::itemPixmapRect(const QRect & r, int flags, const QPixmap & pixmap);
impl<'a> /*trait*/ QStyle_itemPixmapRect<QRect> for (&'a QRect, i32, &'a QPixmap) {
  fn itemPixmapRect(self , rsthis: & QStyle) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QStyle14itemPixmapRectERK5QRectiRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK6QStyle14itemPixmapRectERK5QRectiRK7QPixmap(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QStyle::itemTextRect(const QFontMetrics & fm, const QRect & r, int flags, bool enabled, const QString & text);
impl /*struct*/ QStyle {
  pub fn itemTextRect<RetType, T: QStyle_itemTextRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemTextRect(self);
    // return 1;
  }
}

pub trait QStyle_itemTextRect<RetType> {
  fn itemTextRect(self , rsthis: & QStyle) -> RetType;
}

  // proto:  QRect QStyle::itemTextRect(const QFontMetrics & fm, const QRect & r, int flags, bool enabled, const QString & text);
impl<'a> /*trait*/ QStyle_itemTextRect<QRect> for (&'a QFontMetrics, &'a QRect, i32, i8, &'a QString) {
  fn itemTextRect(self , rsthis: & QStyle) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QStyle12itemTextRectERK12QFontMetricsRK5QRectibRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_char;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK6QStyle12itemTextRectERK12QFontMetricsRK5QRectibRK7QString(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QStyle * QStyle::proxy();
impl /*struct*/ QStyle {
  pub fn proxy<RetType, T: QStyle_proxy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.proxy(self);
    // return 1;
  }
}

pub trait QStyle_proxy<RetType> {
  fn proxy(self , rsthis: & QStyle) -> RetType;
}

  // proto:  const QStyle * QStyle::proxy();
impl<'a> /*trait*/ QStyle_proxy<QStyle> for () {
  fn proxy(self , rsthis: & QStyle) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QStyle5proxyEv()};
    let mut ret = unsafe {C_ZNK6QStyle5proxyEv(rsthis.qclsinst)};
    let mut ret1 = QStyle::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPalette QStyle::standardPalette();
impl /*struct*/ QStyle {
  pub fn standardPalette<RetType, T: QStyle_standardPalette<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.standardPalette(self);
    // return 1;
  }
}

pub trait QStyle_standardPalette<RetType> {
  fn standardPalette(self , rsthis: & QStyle) -> RetType;
}

  // proto:  QPalette QStyle::standardPalette();
impl<'a> /*trait*/ QStyle_standardPalette<QPalette> for () {
  fn standardPalette(self , rsthis: & QStyle) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QStyle15standardPaletteEv()};
    let mut ret = unsafe {C_ZNK6QStyle15standardPaletteEv(rsthis.qclsinst)};
    let mut ret1 = QPalette::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QStyle::metaObject();
impl /*struct*/ QStyle {
  pub fn metaObject<RetType, T: QStyle_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QStyle_metaObject<RetType> {
  fn metaObject(self , rsthis: & QStyle) -> RetType;
}

  // proto:  const QMetaObject * QStyle::metaObject();
impl<'a> /*trait*/ QStyle_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QStyle) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QStyle10metaObjectEv()};
    let mut ret = unsafe {C_ZNK6QStyle10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStyle::polish(QApplication * );
impl<'a> /*trait*/ QStyle_polish<()> for (&'a QApplication) {
  fn polish(self , rsthis: & QStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyle6polishEP12QApplication()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN6QStyle6polishEP12QApplication(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStyle::drawItemPixmap(QPainter * painter, const QRect & rect, int alignment, const QPixmap & pixmap);
impl /*struct*/ QStyle {
  pub fn drawItemPixmap<RetType, T: QStyle_drawItemPixmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawItemPixmap(self);
    // return 1;
  }
}

pub trait QStyle_drawItemPixmap<RetType> {
  fn drawItemPixmap(self , rsthis: & QStyle) -> RetType;
}

  // proto:  void QStyle::drawItemPixmap(QPainter * painter, const QRect & rect, int alignment, const QPixmap & pixmap);
impl<'a> /*trait*/ QStyle_drawItemPixmap<()> for (&'a QPainter, &'a QRect, i32, &'a QPixmap) {
  fn drawItemPixmap(self , rsthis: & QStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QStyle14drawItemPixmapEP8QPainterRK5QRectiRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
     unsafe {C_ZNK6QStyle14drawItemPixmapEP8QPainterRK5QRectiRK7QPixmap(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QStyle::polish(QWidget * );
impl<'a> /*trait*/ QStyle_polish<()> for (&'a QWidget) {
  fn polish(self , rsthis: & QStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyle6polishEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN6QStyle6polishEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static int QStyle::sliderPositionFromValue(int min, int max, int val, int space, bool upsideDown);
impl /*struct*/ QStyle {
  pub fn sliderPositionFromValue_s<RetType, T: QStyle_sliderPositionFromValue_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.sliderPositionFromValue_s();
    // return 1;
  }
}

pub trait QStyle_sliderPositionFromValue_s<RetType> {
  fn sliderPositionFromValue_s(self ) -> RetType;
}

  // proto: static int QStyle::sliderPositionFromValue(int min, int max, int val, int space, bool upsideDown);
impl<'a> /*trait*/ QStyle_sliderPositionFromValue_s<i32> for (i32, i32, i32, i32, Option<i8>) {
  fn sliderPositionFromValue_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyle23sliderPositionFromValueEiiiib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = (if self.4.is_none() {false as i8} else {self.4.unwrap()})  as c_char;
    let mut ret = unsafe {C_ZN6QStyle23sliderPositionFromValueEiiiib(arg0, arg1, arg2, arg3, arg4)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static int QStyle::sliderValueFromPosition(int min, int max, int pos, int space, bool upsideDown);
impl /*struct*/ QStyle {
  pub fn sliderValueFromPosition_s<RetType, T: QStyle_sliderValueFromPosition_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.sliderValueFromPosition_s();
    // return 1;
  }
}

pub trait QStyle_sliderValueFromPosition_s<RetType> {
  fn sliderValueFromPosition_s(self ) -> RetType;
}

  // proto: static int QStyle::sliderValueFromPosition(int min, int max, int pos, int space, bool upsideDown);
impl<'a> /*trait*/ QStyle_sliderValueFromPosition_s<i32> for (i32, i32, i32, i32, Option<i8>) {
  fn sliderValueFromPosition_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyle23sliderValueFromPositionEiiiib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = (if self.4.is_none() {false as i8} else {self.4.unwrap()})  as c_char;
    let mut ret = unsafe {C_ZN6QStyle23sliderValueFromPositionEiiiib(arg0, arg1, arg2, arg3, arg4)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QStyle::unpolish(QApplication * );
impl<'a> /*trait*/ QStyle_unpolish<()> for (&'a QApplication) {
  fn unpolish(self , rsthis: & QStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyle8unpolishEP12QApplication()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN6QStyle8unpolishEP12QApplication(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

