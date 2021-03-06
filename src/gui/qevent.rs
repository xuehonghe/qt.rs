// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtGui/qevent.h
// dst-file: /src/gui/qevent.rs
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
use super::super::core::qcoreevent::*; // 771
use std::ops::Deref;
use super::super::core::qstring::*; // 771
use super::qregion::*; // 773
use super::super::core::qpoint::*; // 771
use super::super::widgets::qaction::*; // 771
// use super::qevent::QInputEvent; // 773
use super::super::core::qfile::*; // 771
use super::super::core::qurl::*; // 771
use super::qtouchdevice::*; // 773
use super::qwindow::*; // 773
use super::super::core::qobject::*; // 771
use super::qscreen::*; // 773
// use super::qevent::QDragMoveEvent; // 773
use super::super::core::qmimedata::*; // 771
// use super::qevent::QDropEvent; // 773
use super::super::core::qrect::*; // 771
use super::super::core::qsize::*; // 771
use super::qkeysequence::*; // 773
use super::super::core::qvariant::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QWhatsThisClickedEvent_Class_Size() -> c_int;
  // proto:  QString QWhatsThisClickedEvent::href();
  fn C_ZNK22QWhatsThisClickedEvent4hrefEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWhatsThisClickedEvent::~QWhatsThisClickedEvent();
  fn C_ZN22QWhatsThisClickedEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QWhatsThisClickedEvent::QWhatsThisClickedEvent(const QString & href);
  fn C_ZN22QWhatsThisClickedEventC2ERK7QString(arg0: *mut c_void) -> u64;
  fn QExposeEvent_Class_Size() -> c_int;
  // proto:  void QExposeEvent::QExposeEvent(const QRegion & rgn);
  fn C_ZN12QExposeEventC2ERK7QRegion(arg0: *mut c_void) -> u64;
  // proto:  const QRegion & QExposeEvent::region();
  fn C_ZNK12QExposeEvent6regionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QExposeEvent::~QExposeEvent();
  fn C_ZN12QExposeEventD2Ev(qthis: u64 /* *mut c_void*/);
  fn QInputMethodEvent_Class_Size() -> c_int;
  // proto:  const QString & QInputMethodEvent::preeditString();
  fn C_ZNK17QInputMethodEvent13preeditStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QInputMethodEvent::QInputMethodEvent();
  fn C_ZN17QInputMethodEventC2Ev() -> u64;
  // proto:  int QInputMethodEvent::replacementStart();
  fn C_ZNK17QInputMethodEvent16replacementStartEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QInputMethodEvent::QInputMethodEvent(const QInputMethodEvent & other);
  fn C_ZN17QInputMethodEventC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  const QString & QInputMethodEvent::commitString();
  fn C_ZNK17QInputMethodEvent12commitStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QInputMethodEvent::setCommitString(const QString & commitString, int replaceFrom, int replaceLength);
  fn C_ZN17QInputMethodEvent15setCommitStringERK7QStringii(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  int QInputMethodEvent::replacementLength();
  fn C_ZNK17QInputMethodEvent17replacementLengthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  fn QHelpEvent_Class_Size() -> c_int;
  // proto:  const QPoint & QHelpEvent::globalPos();
  fn C_ZNK10QHelpEvent9globalPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QHelpEvent::globalX();
  fn C_ZNK10QHelpEvent7globalXEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  const QPoint & QHelpEvent::pos();
  fn C_ZNK10QHelpEvent3posEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QHelpEvent::y();
  fn C_ZNK10QHelpEvent1yEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QHelpEvent::globalY();
  fn C_ZNK10QHelpEvent7globalYEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QHelpEvent::x();
  fn C_ZNK10QHelpEvent1xEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QHelpEvent::~QHelpEvent();
  fn C_ZN10QHelpEventD2Ev(qthis: u64 /* *mut c_void*/);
  fn QActionEvent_Class_Size() -> c_int;
  // proto:  void QActionEvent::QActionEvent(int type, QAction * action, QAction * before);
  fn C_ZN12QActionEventC2EiP7QActionS1_(arg0: c_int, arg1: *mut c_void, arg2: *mut c_void) -> u64;
  // proto:  QAction * QActionEvent::before();
  fn C_ZNK12QActionEvent6beforeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QAction * QActionEvent::action();
  fn C_ZNK12QActionEvent6actionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QActionEvent::~QActionEvent();
  fn C_ZN12QActionEventD2Ev(qthis: u64 /* *mut c_void*/);
  fn QMouseEvent_Class_Size() -> c_int;
  // proto:  QPoint QMouseEvent::globalPos();
  fn C_ZNK11QMouseEvent9globalPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QMouseEvent::y();
  fn C_ZNK11QMouseEvent1yEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  const QPointF & QMouseEvent::screenPos();
  fn C_ZNK11QMouseEvent9screenPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QMouseEvent::x();
  fn C_ZNK11QMouseEvent1xEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  const QPointF & QMouseEvent::localPos();
  fn C_ZNK11QMouseEvent8localPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QMouseEvent::globalX();
  fn C_ZNK11QMouseEvent7globalXEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  const QPointF & QMouseEvent::windowPos();
  fn C_ZNK11QMouseEvent9windowPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QMouseEvent::~QMouseEvent();
  fn C_ZN11QMouseEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QMouseEvent::globalY();
  fn C_ZNK11QMouseEvent7globalYEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QPoint QMouseEvent::pos();
  fn C_ZNK11QMouseEvent3posEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QFileOpenEvent_Class_Size() -> c_int;
  // proto:  void QFileOpenEvent::QFileOpenEvent(const QString & file);
  fn C_ZN14QFileOpenEventC2ERK7QString(arg0: *mut c_void) -> u64;
  // proto:  void QFileOpenEvent::~QFileOpenEvent();
  fn C_ZN14QFileOpenEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QFileOpenEvent::QFileOpenEvent(const QUrl & url);
  fn C_ZN14QFileOpenEventC2ERK4QUrl(arg0: *mut c_void) -> u64;
  // proto:  QString QFileOpenEvent::file();
  fn C_ZNK14QFileOpenEvent4fileEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QUrl QFileOpenEvent::url();
  fn C_ZNK14QFileOpenEvent3urlEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QToolBarChangeEvent_Class_Size() -> c_int;
  // proto:  void QToolBarChangeEvent::QToolBarChangeEvent(bool t);
  fn C_ZN19QToolBarChangeEventC2Eb(arg0: c_char) -> u64;
  // proto:  void QToolBarChangeEvent::~QToolBarChangeEvent();
  fn C_ZN19QToolBarChangeEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QToolBarChangeEvent::toggle();
  fn C_ZNK19QToolBarChangeEvent6toggleEv(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QTabletEvent_Class_Size() -> c_int;
  // proto:  int QTabletEvent::x();
  fn C_ZNK12QTabletEvent1xEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QTabletEvent::xTilt();
  fn C_ZNK12QTabletEvent5xTiltEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  qint64 QTabletEvent::uniqueId();
  fn C_ZNK12QTabletEvent8uniqueIdEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  const QPointF & QTabletEvent::globalPosF();
  fn C_ZNK12QTabletEvent10globalPosFEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QTabletEvent::z();
  fn C_ZNK12QTabletEvent1zEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QTabletEvent::y();
  fn C_ZNK12QTabletEvent1yEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QPoint QTabletEvent::pos();
  fn C_ZNK12QTabletEvent3posEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qreal QTabletEvent::rotation();
  fn C_ZNK12QTabletEvent8rotationEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QPoint QTabletEvent::globalPos();
  fn C_ZNK12QTabletEvent9globalPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTabletEvent::~QTabletEvent();
  fn C_ZN12QTabletEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  qreal QTabletEvent::tangentialPressure();
  fn C_ZNK12QTabletEvent18tangentialPressureEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QTabletEvent::hiResGlobalX();
  fn C_ZNK12QTabletEvent12hiResGlobalXEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  int QTabletEvent::globalY();
  fn C_ZNK12QTabletEvent7globalYEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  qreal QTabletEvent::hiResGlobalY();
  fn C_ZNK12QTabletEvent12hiResGlobalYEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  int QTabletEvent::globalX();
  fn C_ZNK12QTabletEvent7globalXEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  const QPointF & QTabletEvent::posF();
  fn C_ZNK12QTabletEvent4posFEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qreal QTabletEvent::pressure();
  fn C_ZNK12QTabletEvent8pressureEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  int QTabletEvent::yTilt();
  fn C_ZNK12QTabletEvent5yTiltEv(qthis: u64 /* *mut c_void*/) -> c_int;
  fn QTouchEvent_Class_Size() -> c_int;
  // proto:  void QTouchEvent::setDevice(QTouchDevice * adevice);
  fn C_ZN11QTouchEvent9setDeviceEP12QTouchDevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QWindow * QTouchEvent::window();
  fn C_ZNK11QTouchEvent6windowEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QTouchDevice * QTouchEvent::device();
  fn C_ZNK11QTouchEvent6deviceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QObject * QTouchEvent::target();
  fn C_ZNK11QTouchEvent6targetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTouchEvent::~QTouchEvent();
  fn C_ZN11QTouchEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QTouchEvent::setWindow(QWindow * awindow);
  fn C_ZN11QTouchEvent9setWindowEP7QWindow(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTouchEvent::setTarget(QObject * atarget);
  fn C_ZN11QTouchEvent9setTargetEP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QScreenOrientationChangeEvent_Class_Size() -> c_int;
  // proto:  QScreen * QScreenOrientationChangeEvent::screen();
  fn C_ZNK29QScreenOrientationChangeEvent6screenEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QScreenOrientationChangeEvent::~QScreenOrientationChangeEvent();
  fn C_ZN29QScreenOrientationChangeEventD2Ev(qthis: u64 /* *mut c_void*/);
  fn QIconDragEvent_Class_Size() -> c_int;
  // proto:  void QIconDragEvent::~QIconDragEvent();
  fn C_ZN14QIconDragEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QIconDragEvent::QIconDragEvent();
  fn C_ZN14QIconDragEventC2Ev() -> u64;
  fn QCloseEvent_Class_Size() -> c_int;
  // proto:  void QCloseEvent::~QCloseEvent();
  fn C_ZN11QCloseEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QCloseEvent::QCloseEvent();
  fn C_ZN11QCloseEventC2Ev() -> u64;
  fn QDragEnterEvent_Class_Size() -> c_int;
  // proto:  void QDragEnterEvent::~QDragEnterEvent();
  fn C_ZN15QDragEnterEventD2Ev(qthis: u64 /* *mut c_void*/);
  fn QWheelEvent_Class_Size() -> c_int;
  // proto:  int QWheelEvent::x();
  fn C_ZNK11QWheelEvent1xEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QPoint QWheelEvent::angleDelta();
  fn C_ZNK11QWheelEvent10angleDeltaEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPoint QWheelEvent::pos();
  fn C_ZNK11QWheelEvent3posEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QWheelEvent::globalY();
  fn C_ZNK11QWheelEvent7globalYEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  const QPointF & QWheelEvent::posF();
  fn C_ZNK11QWheelEvent4posFEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QWheelEvent::globalX();
  fn C_ZNK11QWheelEvent7globalXEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QWheelEvent::y();
  fn C_ZNK11QWheelEvent1yEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QWheelEvent::~QWheelEvent();
  fn C_ZN11QWheelEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QPoint QWheelEvent::pixelDelta();
  fn C_ZNK11QWheelEvent10pixelDeltaEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QWheelEvent::delta();
  fn C_ZNK11QWheelEvent5deltaEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QPoint QWheelEvent::globalPos();
  fn C_ZNK11QWheelEvent9globalPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QPointF & QWheelEvent::globalPosF();
  fn C_ZNK11QWheelEvent10globalPosFEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QScrollEvent_Class_Size() -> c_int;
  // proto:  QPointF QScrollEvent::contentPos();
  fn C_ZNK12QScrollEvent10contentPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPointF QScrollEvent::overshootDistance();
  fn C_ZNK12QScrollEvent17overshootDistanceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QScrollEvent::~QScrollEvent();
  fn C_ZN12QScrollEventD2Ev(qthis: u64 /* *mut c_void*/);
  fn QHoverEvent_Class_Size() -> c_int;
  // proto:  void QHoverEvent::~QHoverEvent();
  fn C_ZN11QHoverEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QPointF & QHoverEvent::posF();
  fn C_ZNK11QHoverEvent4posFEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPoint QHoverEvent::oldPos();
  fn C_ZNK11QHoverEvent6oldPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QPointF & QHoverEvent::oldPosF();
  fn C_ZNK11QHoverEvent7oldPosFEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPoint QHoverEvent::pos();
  fn C_ZNK11QHoverEvent3posEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QDragMoveEvent_Class_Size() -> c_int;
  // proto:  void QDragMoveEvent::accept(const QRect & r);
  fn C_ZN14QDragMoveEvent6acceptERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QRect QDragMoveEvent::answerRect();
  fn C_ZNK14QDragMoveEvent10answerRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDragMoveEvent::ignore(const QRect & r);
  fn C_ZN14QDragMoveEvent6ignoreERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDragMoveEvent::ignore();
  fn C_ZN14QDragMoveEvent6ignoreEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDragMoveEvent::~QDragMoveEvent();
  fn C_ZN14QDragMoveEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QDragMoveEvent::accept();
  fn C_ZN14QDragMoveEvent6acceptEv(qthis: u64 /* *mut c_void*/);
  fn QShowEvent_Class_Size() -> c_int;
  // proto:  void QShowEvent::~QShowEvent();
  fn C_ZN10QShowEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QShowEvent::QShowEvent();
  fn C_ZN10QShowEventC2Ev() -> u64;
  fn QPlatformSurfaceEvent_Class_Size() -> c_int;
  // proto:  void QPlatformSurfaceEvent::~QPlatformSurfaceEvent();
  fn C_ZN21QPlatformSurfaceEventD2Ev(qthis: u64 /* *mut c_void*/);
  fn QPaintEvent_Class_Size() -> c_int;
  // proto:  void QPaintEvent::~QPaintEvent();
  fn C_ZN11QPaintEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QRect & QPaintEvent::rect();
  fn C_ZNK11QPaintEvent4rectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPaintEvent::QPaintEvent(const QRect & paintRect);
  fn C_ZN11QPaintEventC2ERK5QRect(arg0: *mut c_void) -> u64;
  // proto:  const QRegion & QPaintEvent::region();
  fn C_ZNK11QPaintEvent6regionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPaintEvent::QPaintEvent(const QRegion & paintRegion);
  fn C_ZN11QPaintEventC2ERK7QRegion(arg0: *mut c_void) -> u64;
  fn QFocusEvent_Class_Size() -> c_int;
  // proto:  bool QFocusEvent::lostFocus();
  fn C_ZNK11QFocusEvent9lostFocusEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QFocusEvent::gotFocus();
  fn C_ZNK11QFocusEvent8gotFocusEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QFocusEvent::~QFocusEvent();
  fn C_ZN11QFocusEventD2Ev(qthis: u64 /* *mut c_void*/);
  fn QNativeGestureEvent_Class_Size() -> c_int;
  // proto:  const QPointF & QNativeGestureEvent::localPos();
  fn C_ZNK19QNativeGestureEvent8localPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QPointF & QNativeGestureEvent::screenPos();
  fn C_ZNK19QNativeGestureEvent9screenPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QPoint QNativeGestureEvent::pos();
  fn C_ZNK19QNativeGestureEvent3posEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QPoint QNativeGestureEvent::globalPos();
  fn C_ZNK19QNativeGestureEvent9globalPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qreal QNativeGestureEvent::value();
  fn C_ZNK19QNativeGestureEvent5valueEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  const QPointF & QNativeGestureEvent::windowPos();
  fn C_ZNK19QNativeGestureEvent9windowPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QResizeEvent_Class_Size() -> c_int;
  // proto:  const QSize & QResizeEvent::oldSize();
  fn C_ZNK12QResizeEvent7oldSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QSize & QResizeEvent::size();
  fn C_ZNK12QResizeEvent4sizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QResizeEvent::~QResizeEvent();
  fn C_ZN12QResizeEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QResizeEvent::QResizeEvent(const QSize & size, const QSize & oldSize);
  fn C_ZN12QResizeEventC2ERK5QSizeS2_(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  fn QStatusTipEvent_Class_Size() -> c_int;
  // proto:  void QStatusTipEvent::~QStatusTipEvent();
  fn C_ZN15QStatusTipEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QString QStatusTipEvent::tip();
  fn C_ZNK15QStatusTipEvent3tipEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStatusTipEvent::QStatusTipEvent(const QString & tip);
  fn C_ZN15QStatusTipEventC2ERK7QString(arg0: *mut c_void) -> u64;
  fn QEnterEvent_Class_Size() -> c_int;
  // proto:  int QEnterEvent::y();
  fn C_ZNK11QEnterEvent1yEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QPoint QEnterEvent::pos();
  fn C_ZNK11QEnterEvent3posEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QEnterEvent::~QEnterEvent();
  fn C_ZN11QEnterEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QPointF & QEnterEvent::screenPos();
  fn C_ZNK11QEnterEvent9screenPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QPointF & QEnterEvent::localPos();
  fn C_ZNK11QEnterEvent8localPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QPointF & QEnterEvent::windowPos();
  fn C_ZNK11QEnterEvent9windowPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QEnterEvent::globalX();
  fn C_ZNK11QEnterEvent7globalXEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QEnterEvent::x();
  fn C_ZNK11QEnterEvent1xEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QPoint QEnterEvent::globalPos();
  fn C_ZNK11QEnterEvent9globalPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QEnterEvent::globalY();
  fn C_ZNK11QEnterEvent7globalYEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QEnterEvent::QEnterEvent(const QPointF & localPos, const QPointF & windowPos, const QPointF & screenPos);
  fn C_ZN11QEnterEventC2ERK7QPointFS2_S2_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> u64;
  fn QMoveEvent_Class_Size() -> c_int;
  // proto:  void QMoveEvent::~QMoveEvent();
  fn C_ZN10QMoveEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QPoint & QMoveEvent::oldPos();
  fn C_ZNK10QMoveEvent6oldPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QMoveEvent::QMoveEvent(const QPoint & pos, const QPoint & oldPos);
  fn C_ZN10QMoveEventC2ERK6QPointS2_(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  const QPoint & QMoveEvent::pos();
  fn C_ZNK10QMoveEvent3posEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QHideEvent_Class_Size() -> c_int;
  // proto:  void QHideEvent::QHideEvent();
  fn C_ZN10QHideEventC2Ev() -> u64;
  // proto:  void QHideEvent::~QHideEvent();
  fn C_ZN10QHideEventD2Ev(qthis: u64 /* *mut c_void*/);
  fn QDragLeaveEvent_Class_Size() -> c_int;
  // proto:  void QDragLeaveEvent::~QDragLeaveEvent();
  fn C_ZN15QDragLeaveEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QDragLeaveEvent::QDragLeaveEvent();
  fn C_ZN15QDragLeaveEventC2Ev() -> u64;
  fn QDropEvent_Class_Size() -> c_int;
  // proto:  void QDropEvent::~QDropEvent();
  fn C_ZN10QDropEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QPoint QDropEvent::pos();
  fn C_ZNK10QDropEvent3posEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QObject * QDropEvent::source();
  fn C_ZNK10QDropEvent6sourceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMimeData * QDropEvent::mimeData();
  fn C_ZNK10QDropEvent8mimeDataEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDropEvent::acceptProposedAction();
  fn C_ZN10QDropEvent20acceptProposedActionEv(qthis: u64 /* *mut c_void*/);
  // proto:  const QPointF & QDropEvent::posF();
  fn C_ZNK10QDropEvent4posFEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QInputEvent_Class_Size() -> c_int;
  // proto:  void QInputEvent::setTimestamp(ulong atimestamp);
  fn C_ZN11QInputEvent12setTimestampEm(qthis: u64 /* *mut c_void*/, arg0: c_ulong);
  // proto:  ulong QInputEvent::timestamp();
  fn C_ZNK11QInputEvent9timestampEv(qthis: u64 /* *mut c_void*/) -> c_ulong;
  // proto:  void QInputEvent::~QInputEvent();
  fn C_ZN11QInputEventD2Ev(qthis: u64 /* *mut c_void*/);
  fn QApplicationStateChangeEvent_Class_Size() -> c_int;
  fn QKeyEvent_Class_Size() -> c_int;
  // proto:  int QKeyEvent::count();
  fn C_ZNK9QKeyEvent5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QKeyEvent::~QKeyEvent();
  fn C_ZN9QKeyEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QString QKeyEvent::text();
  fn C_ZNK9QKeyEvent4textEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  quint32 QKeyEvent::nativeVirtualKey();
  fn C_ZNK9QKeyEvent16nativeVirtualKeyEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  bool QKeyEvent::isAutoRepeat();
  fn C_ZNK9QKeyEvent12isAutoRepeatEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QKeyEvent::key();
  fn C_ZNK9QKeyEvent3keyEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  quint32 QKeyEvent::nativeModifiers();
  fn C_ZNK9QKeyEvent15nativeModifiersEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  quint32 QKeyEvent::nativeScanCode();
  fn C_ZNK9QKeyEvent14nativeScanCodeEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  fn QContextMenuEvent_Class_Size() -> c_int;
  // proto:  const QPoint & QContextMenuEvent::globalPos();
  fn C_ZNK17QContextMenuEvent9globalPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QContextMenuEvent::globalY();
  fn C_ZNK17QContextMenuEvent7globalYEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QContextMenuEvent::globalX();
  fn C_ZNK17QContextMenuEvent7globalXEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  const QPoint & QContextMenuEvent::pos();
  fn C_ZNK17QContextMenuEvent3posEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QContextMenuEvent::y();
  fn C_ZNK17QContextMenuEvent1yEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QContextMenuEvent::x();
  fn C_ZNK17QContextMenuEvent1xEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QContextMenuEvent::~QContextMenuEvent();
  fn C_ZN17QContextMenuEventD2Ev(qthis: u64 /* *mut c_void*/);
  fn QScrollPrepareEvent_Class_Size() -> c_int;
  // proto:  void QScrollPrepareEvent::setContentPosRange(const QRectF & rect);
  fn C_ZN19QScrollPrepareEvent18setContentPosRangeERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QScrollPrepareEvent::setContentPos(const QPointF & pos);
  fn C_ZN19QScrollPrepareEvent13setContentPosERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QRectF QScrollPrepareEvent::contentPosRange();
  fn C_ZNK19QScrollPrepareEvent15contentPosRangeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPointF QScrollPrepareEvent::contentPos();
  fn C_ZNK19QScrollPrepareEvent10contentPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QScrollPrepareEvent::setViewportSize(const QSizeF & size);
  fn C_ZN19QScrollPrepareEvent15setViewportSizeERK6QSizeF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QScrollPrepareEvent::QScrollPrepareEvent(const QPointF & startPos);
  fn C_ZN19QScrollPrepareEventC2ERK7QPointF(arg0: *mut c_void) -> u64;
  // proto:  QPointF QScrollPrepareEvent::startPos();
  fn C_ZNK19QScrollPrepareEvent8startPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSizeF QScrollPrepareEvent::viewportSize();
  fn C_ZNK19QScrollPrepareEvent12viewportSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QScrollPrepareEvent::~QScrollPrepareEvent();
  fn C_ZN19QScrollPrepareEventD2Ev(qthis: u64 /* *mut c_void*/);
  fn QShortcutEvent_Class_Size() -> c_int;
  // proto:  const QKeySequence & QShortcutEvent::key();
  fn C_ZNK14QShortcutEvent3keyEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QShortcutEvent::~QShortcutEvent();
  fn C_ZN14QShortcutEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QShortcutEvent::isAmbiguous();
  fn C_ZNK14QShortcutEvent11isAmbiguousEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QShortcutEvent::QShortcutEvent(const QKeySequence & key, int id, bool ambiguous);
  fn C_ZN14QShortcutEventC2ERK12QKeySequenceib(arg0: *mut c_void, arg1: c_int, arg2: c_char) -> u64;
  // proto:  int QShortcutEvent::shortcutId();
  fn C_ZNK14QShortcutEvent10shortcutIdEv(qthis: u64 /* *mut c_void*/) -> c_int;
  fn QWindowStateChangeEvent_Class_Size() -> c_int;
  // proto:  bool QWindowStateChangeEvent::isOverride();
  fn C_ZNK23QWindowStateChangeEvent10isOverrideEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QWindowStateChangeEvent::~QWindowStateChangeEvent();
  fn C_ZN23QWindowStateChangeEventD2Ev(qthis: u64 /* *mut c_void*/);
  fn QInputMethodQueryEvent_Class_Size() -> c_int;
  // proto:  void QInputMethodQueryEvent::~QInputMethodQueryEvent();
  fn C_ZN22QInputMethodQueryEventD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QWhatsThisClickedEvent)=32
#[derive(Default)]
pub struct QWhatsThisClickedEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QExposeEvent)=32
#[derive(Default)]
pub struct QExposeEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QInputMethodEvent)=1
#[derive(Default)]
pub struct QInputMethodEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QHelpEvent)=40
#[derive(Default)]
pub struct QHelpEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QActionEvent)=40
#[derive(Default)]
pub struct QActionEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QMouseEvent)=1
#[derive(Default)]
pub struct QMouseEvent {
  qbase: QInputEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QFileOpenEvent)=40
#[derive(Default)]
pub struct QFileOpenEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QToolBarChangeEvent)=24
#[derive(Default)]
pub struct QToolBarChangeEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QTabletEvent)=1
#[derive(Default)]
pub struct QTabletEvent {
  qbase: QInputEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QTouchEvent)=1
#[derive(Default)]
pub struct QTouchEvent {
  qbase: QInputEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QScreenOrientationChangeEvent)=40
#[derive(Default)]
pub struct QScreenOrientationChangeEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QIconDragEvent)=24
#[derive(Default)]
pub struct QIconDragEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QCloseEvent)=24
#[derive(Default)]
pub struct QCloseEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QDragEnterEvent)=1
#[derive(Default)]
pub struct QDragEnterEvent {
  qbase: QDragMoveEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QWheelEvent)=1
#[derive(Default)]
pub struct QWheelEvent {
  qbase: QInputEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QScrollEvent)=64
#[derive(Default)]
pub struct QScrollEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QHoverEvent)=1
#[derive(Default)]
pub struct QHoverEvent {
  qbase: QInputEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QDragMoveEvent)=1
#[derive(Default)]
pub struct QDragMoveEvent {
  qbase: QDropEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QShowEvent)=24
#[derive(Default)]
pub struct QShowEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QPlatformSurfaceEvent)=24
#[derive(Default)]
pub struct QPlatformSurfaceEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QPaintEvent)=56
#[derive(Default)]
pub struct QPaintEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QFocusEvent)=24
#[derive(Default)]
pub struct QFocusEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QNativeGestureEvent)=1
#[derive(Default)]
pub struct QNativeGestureEvent {
  qbase: QInputEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QResizeEvent)=40
#[derive(Default)]
pub struct QResizeEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStatusTipEvent)=32
#[derive(Default)]
pub struct QStatusTipEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QEnterEvent)=72
#[derive(Default)]
pub struct QEnterEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QMoveEvent)=40
#[derive(Default)]
pub struct QMoveEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QHideEvent)=24
#[derive(Default)]
pub struct QHideEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QDragLeaveEvent)=24
#[derive(Default)]
pub struct QDragLeaveEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QDropEvent)=1
#[derive(Default)]
pub struct QDropEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QInputEvent)=1
#[derive(Default)]
pub struct QInputEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QApplicationStateChangeEvent)=24
#[derive(Default)]
pub struct QApplicationStateChangeEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QKeyEvent)=1
#[derive(Default)]
pub struct QKeyEvent {
  qbase: QInputEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QContextMenuEvent)=1
#[derive(Default)]
pub struct QContextMenuEvent {
  qbase: QInputEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QScrollPrepareEvent)=112
#[derive(Default)]
pub struct QScrollPrepareEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QShortcutEvent)=40
#[derive(Default)]
pub struct QShortcutEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QWindowStateChangeEvent)=1
#[derive(Default)]
pub struct QWindowStateChangeEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QInputMethodQueryEvent)=1
#[derive(Default)]
pub struct QInputMethodQueryEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QWhatsThisClickedEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QWhatsThisClickedEvent {
    return QWhatsThisClickedEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QWhatsThisClickedEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QWhatsThisClickedEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  QString QWhatsThisClickedEvent::href();
impl /*struct*/ QWhatsThisClickedEvent {
  pub fn href<RetType, T: QWhatsThisClickedEvent_href<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.href(self);
    // return 1;
  }
}

pub trait QWhatsThisClickedEvent_href<RetType> {
  fn href(self , rsthis: & QWhatsThisClickedEvent) -> RetType;
}

  // proto:  QString QWhatsThisClickedEvent::href();
impl<'a> /*trait*/ QWhatsThisClickedEvent_href<QString> for () {
  fn href(self , rsthis: & QWhatsThisClickedEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QWhatsThisClickedEvent4hrefEv()};
    let mut ret = unsafe {C_ZNK22QWhatsThisClickedEvent4hrefEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWhatsThisClickedEvent::~QWhatsThisClickedEvent();
impl /*struct*/ QWhatsThisClickedEvent {
  pub fn free<RetType, T: QWhatsThisClickedEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QWhatsThisClickedEvent_free<RetType> {
  fn free(self , rsthis: & QWhatsThisClickedEvent) -> RetType;
}

  // proto:  void QWhatsThisClickedEvent::~QWhatsThisClickedEvent();
impl<'a> /*trait*/ QWhatsThisClickedEvent_free<()> for () {
  fn free(self , rsthis: & QWhatsThisClickedEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QWhatsThisClickedEventD2Ev()};
     unsafe {C_ZN22QWhatsThisClickedEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWhatsThisClickedEvent::QWhatsThisClickedEvent(const QString & href);
impl /*struct*/ QWhatsThisClickedEvent {
  pub fn new<T: QWhatsThisClickedEvent_new>(value: T) -> QWhatsThisClickedEvent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QWhatsThisClickedEvent_new {
  fn new(self) -> QWhatsThisClickedEvent;
}

  // proto:  void QWhatsThisClickedEvent::QWhatsThisClickedEvent(const QString & href);
impl<'a> /*trait*/ QWhatsThisClickedEvent_new for (&'a QString) {
  fn new(self) -> QWhatsThisClickedEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QWhatsThisClickedEventC2ERK7QString()};
    let ctysz: c_int = unsafe{QWhatsThisClickedEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN22QWhatsThisClickedEventC2ERK7QString(arg0)};
    let rsthis = QWhatsThisClickedEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QExposeEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QExposeEvent {
    return QExposeEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QExposeEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QExposeEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QExposeEvent::QExposeEvent(const QRegion & rgn);
impl /*struct*/ QExposeEvent {
  pub fn new<T: QExposeEvent_new>(value: T) -> QExposeEvent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QExposeEvent_new {
  fn new(self) -> QExposeEvent;
}

  // proto:  void QExposeEvent::QExposeEvent(const QRegion & rgn);
impl<'a> /*trait*/ QExposeEvent_new for (&'a QRegion) {
  fn new(self) -> QExposeEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QExposeEventC2ERK7QRegion()};
    let ctysz: c_int = unsafe{QExposeEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN12QExposeEventC2ERK7QRegion(arg0)};
    let rsthis = QExposeEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QRegion & QExposeEvent::region();
impl /*struct*/ QExposeEvent {
  pub fn region<RetType, T: QExposeEvent_region<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.region(self);
    // return 1;
  }
}

pub trait QExposeEvent_region<RetType> {
  fn region(self , rsthis: & QExposeEvent) -> RetType;
}

  // proto:  const QRegion & QExposeEvent::region();
impl<'a> /*trait*/ QExposeEvent_region<QRegion> for () {
  fn region(self , rsthis: & QExposeEvent) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QExposeEvent6regionEv()};
    let mut ret = unsafe {C_ZNK12QExposeEvent6regionEv(rsthis.qclsinst)};
    let mut ret1 = QRegion::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QExposeEvent::~QExposeEvent();
impl /*struct*/ QExposeEvent {
  pub fn free<RetType, T: QExposeEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QExposeEvent_free<RetType> {
  fn free(self , rsthis: & QExposeEvent) -> RetType;
}

  // proto:  void QExposeEvent::~QExposeEvent();
impl<'a> /*trait*/ QExposeEvent_free<()> for () {
  fn free(self , rsthis: & QExposeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QExposeEventD2Ev()};
     unsafe {C_ZN12QExposeEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QInputMethodEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QInputMethodEvent {
    return QInputMethodEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QInputMethodEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QInputMethodEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  const QString & QInputMethodEvent::preeditString();
impl /*struct*/ QInputMethodEvent {
  pub fn preeditString<RetType, T: QInputMethodEvent_preeditString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.preeditString(self);
    // return 1;
  }
}

pub trait QInputMethodEvent_preeditString<RetType> {
  fn preeditString(self , rsthis: & QInputMethodEvent) -> RetType;
}

  // proto:  const QString & QInputMethodEvent::preeditString();
impl<'a> /*trait*/ QInputMethodEvent_preeditString<QString> for () {
  fn preeditString(self , rsthis: & QInputMethodEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QInputMethodEvent13preeditStringEv()};
    let mut ret = unsafe {C_ZNK17QInputMethodEvent13preeditStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QInputMethodEvent::QInputMethodEvent();
impl /*struct*/ QInputMethodEvent {
  pub fn new<T: QInputMethodEvent_new>(value: T) -> QInputMethodEvent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QInputMethodEvent_new {
  fn new(self) -> QInputMethodEvent;
}

  // proto:  void QInputMethodEvent::QInputMethodEvent();
impl<'a> /*trait*/ QInputMethodEvent_new for () {
  fn new(self) -> QInputMethodEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QInputMethodEventC2Ev()};
    let ctysz: c_int = unsafe{QInputMethodEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN17QInputMethodEventC2Ev()};
    let rsthis = QInputMethodEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QInputMethodEvent::replacementStart();
impl /*struct*/ QInputMethodEvent {
  pub fn replacementStart<RetType, T: QInputMethodEvent_replacementStart<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.replacementStart(self);
    // return 1;
  }
}

pub trait QInputMethodEvent_replacementStart<RetType> {
  fn replacementStart(self , rsthis: & QInputMethodEvent) -> RetType;
}

  // proto:  int QInputMethodEvent::replacementStart();
impl<'a> /*trait*/ QInputMethodEvent_replacementStart<i32> for () {
  fn replacementStart(self , rsthis: & QInputMethodEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QInputMethodEvent16replacementStartEv()};
    let mut ret = unsafe {C_ZNK17QInputMethodEvent16replacementStartEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QInputMethodEvent::QInputMethodEvent(const QInputMethodEvent & other);
impl<'a> /*trait*/ QInputMethodEvent_new for (&'a QInputMethodEvent) {
  fn new(self) -> QInputMethodEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QInputMethodEventC2ERKS_()};
    let ctysz: c_int = unsafe{QInputMethodEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN17QInputMethodEventC2ERKS_(arg0)};
    let rsthis = QInputMethodEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QString & QInputMethodEvent::commitString();
impl /*struct*/ QInputMethodEvent {
  pub fn commitString<RetType, T: QInputMethodEvent_commitString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.commitString(self);
    // return 1;
  }
}

pub trait QInputMethodEvent_commitString<RetType> {
  fn commitString(self , rsthis: & QInputMethodEvent) -> RetType;
}

  // proto:  const QString & QInputMethodEvent::commitString();
impl<'a> /*trait*/ QInputMethodEvent_commitString<QString> for () {
  fn commitString(self , rsthis: & QInputMethodEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QInputMethodEvent12commitStringEv()};
    let mut ret = unsafe {C_ZNK17QInputMethodEvent12commitStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QInputMethodEvent::setCommitString(const QString & commitString, int replaceFrom, int replaceLength);
impl /*struct*/ QInputMethodEvent {
  pub fn setCommitString<RetType, T: QInputMethodEvent_setCommitString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCommitString(self);
    // return 1;
  }
}

pub trait QInputMethodEvent_setCommitString<RetType> {
  fn setCommitString(self , rsthis: & QInputMethodEvent) -> RetType;
}

  // proto:  void QInputMethodEvent::setCommitString(const QString & commitString, int replaceFrom, int replaceLength);
impl<'a> /*trait*/ QInputMethodEvent_setCommitString<()> for (&'a QString, Option<i32>, Option<i32>) {
  fn setCommitString(self , rsthis: & QInputMethodEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QInputMethodEvent15setCommitStringERK7QStringii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {0} else {self.1.unwrap()})  as c_int;
    let arg2 = (if self.2.is_none() {0} else {self.2.unwrap()})  as c_int;
     unsafe {C_ZN17QInputMethodEvent15setCommitStringERK7QStringii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  int QInputMethodEvent::replacementLength();
impl /*struct*/ QInputMethodEvent {
  pub fn replacementLength<RetType, T: QInputMethodEvent_replacementLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.replacementLength(self);
    // return 1;
  }
}

pub trait QInputMethodEvent_replacementLength<RetType> {
  fn replacementLength(self , rsthis: & QInputMethodEvent) -> RetType;
}

  // proto:  int QInputMethodEvent::replacementLength();
impl<'a> /*trait*/ QInputMethodEvent_replacementLength<i32> for () {
  fn replacementLength(self , rsthis: & QInputMethodEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QInputMethodEvent17replacementLengthEv()};
    let mut ret = unsafe {C_ZNK17QInputMethodEvent17replacementLengthEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

impl /*struct*/ QHelpEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QHelpEvent {
    return QHelpEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QHelpEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QHelpEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  const QPoint & QHelpEvent::globalPos();
impl /*struct*/ QHelpEvent {
  pub fn globalPos<RetType, T: QHelpEvent_globalPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalPos(self);
    // return 1;
  }
}

pub trait QHelpEvent_globalPos<RetType> {
  fn globalPos(self , rsthis: & QHelpEvent) -> RetType;
}

  // proto:  const QPoint & QHelpEvent::globalPos();
impl<'a> /*trait*/ QHelpEvent_globalPos<QPoint> for () {
  fn globalPos(self , rsthis: & QHelpEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent9globalPosEv()};
    let mut ret = unsafe {C_ZNK10QHelpEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QHelpEvent::globalX();
impl /*struct*/ QHelpEvent {
  pub fn globalX<RetType, T: QHelpEvent_globalX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalX(self);
    // return 1;
  }
}

pub trait QHelpEvent_globalX<RetType> {
  fn globalX(self , rsthis: & QHelpEvent) -> RetType;
}

  // proto:  int QHelpEvent::globalX();
impl<'a> /*trait*/ QHelpEvent_globalX<i32> for () {
  fn globalX(self , rsthis: & QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent7globalXEv()};
    let mut ret = unsafe {C_ZNK10QHelpEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  const QPoint & QHelpEvent::pos();
impl /*struct*/ QHelpEvent {
  pub fn pos<RetType, T: QHelpEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QHelpEvent_pos<RetType> {
  fn pos(self , rsthis: & QHelpEvent) -> RetType;
}

  // proto:  const QPoint & QHelpEvent::pos();
impl<'a> /*trait*/ QHelpEvent_pos<QPoint> for () {
  fn pos(self , rsthis: & QHelpEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent3posEv()};
    let mut ret = unsafe {C_ZNK10QHelpEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QHelpEvent::y();
impl /*struct*/ QHelpEvent {
  pub fn y<RetType, T: QHelpEvent_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QHelpEvent_y<RetType> {
  fn y(self , rsthis: & QHelpEvent) -> RetType;
}

  // proto:  int QHelpEvent::y();
impl<'a> /*trait*/ QHelpEvent_y<i32> for () {
  fn y(self , rsthis: & QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent1yEv()};
    let mut ret = unsafe {C_ZNK10QHelpEvent1yEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QHelpEvent::globalY();
impl /*struct*/ QHelpEvent {
  pub fn globalY<RetType, T: QHelpEvent_globalY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalY(self);
    // return 1;
  }
}

pub trait QHelpEvent_globalY<RetType> {
  fn globalY(self , rsthis: & QHelpEvent) -> RetType;
}

  // proto:  int QHelpEvent::globalY();
impl<'a> /*trait*/ QHelpEvent_globalY<i32> for () {
  fn globalY(self , rsthis: & QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent7globalYEv()};
    let mut ret = unsafe {C_ZNK10QHelpEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QHelpEvent::x();
impl /*struct*/ QHelpEvent {
  pub fn x<RetType, T: QHelpEvent_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QHelpEvent_x<RetType> {
  fn x(self , rsthis: & QHelpEvent) -> RetType;
}

  // proto:  int QHelpEvent::x();
impl<'a> /*trait*/ QHelpEvent_x<i32> for () {
  fn x(self , rsthis: & QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent1xEv()};
    let mut ret = unsafe {C_ZNK10QHelpEvent1xEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QHelpEvent::~QHelpEvent();
impl /*struct*/ QHelpEvent {
  pub fn free<RetType, T: QHelpEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QHelpEvent_free<RetType> {
  fn free(self , rsthis: & QHelpEvent) -> RetType;
}

  // proto:  void QHelpEvent::~QHelpEvent();
impl<'a> /*trait*/ QHelpEvent_free<()> for () {
  fn free(self , rsthis: & QHelpEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN10QHelpEventD2Ev()};
     unsafe {C_ZN10QHelpEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QActionEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QActionEvent {
    return QActionEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QActionEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QActionEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QActionEvent::QActionEvent(int type, QAction * action, QAction * before);
impl /*struct*/ QActionEvent {
  pub fn new<T: QActionEvent_new>(value: T) -> QActionEvent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QActionEvent_new {
  fn new(self) -> QActionEvent;
}

  // proto:  void QActionEvent::QActionEvent(int type, QAction * action, QAction * before);
impl<'a> /*trait*/ QActionEvent_new for (i32, &'a QAction, Option<&'a QAction>) {
  fn new(self) -> QActionEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN12QActionEventC2EiP7QActionS1_()};
    let ctysz: c_int = unsafe{QActionEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = (if self.2.is_none() {0} else {self.2.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN12QActionEventC2EiP7QActionS1_(arg0, arg1, arg2)};
    let rsthis = QActionEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QAction * QActionEvent::before();
impl /*struct*/ QActionEvent {
  pub fn before<RetType, T: QActionEvent_before<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.before(self);
    // return 1;
  }
}

pub trait QActionEvent_before<RetType> {
  fn before(self , rsthis: & QActionEvent) -> RetType;
}

  // proto:  QAction * QActionEvent::before();
impl<'a> /*trait*/ QActionEvent_before<QAction> for () {
  fn before(self , rsthis: & QActionEvent) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK12QActionEvent6beforeEv()};
    let mut ret = unsafe {C_ZNK12QActionEvent6beforeEv(rsthis.qclsinst)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QAction * QActionEvent::action();
impl /*struct*/ QActionEvent {
  pub fn action<RetType, T: QActionEvent_action<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.action(self);
    // return 1;
  }
}

pub trait QActionEvent_action<RetType> {
  fn action(self , rsthis: & QActionEvent) -> RetType;
}

  // proto:  QAction * QActionEvent::action();
impl<'a> /*trait*/ QActionEvent_action<QAction> for () {
  fn action(self , rsthis: & QActionEvent) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK12QActionEvent6actionEv()};
    let mut ret = unsafe {C_ZNK12QActionEvent6actionEv(rsthis.qclsinst)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QActionEvent::~QActionEvent();
impl /*struct*/ QActionEvent {
  pub fn free<RetType, T: QActionEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QActionEvent_free<RetType> {
  fn free(self , rsthis: & QActionEvent) -> RetType;
}

  // proto:  void QActionEvent::~QActionEvent();
impl<'a> /*trait*/ QActionEvent_free<()> for () {
  fn free(self , rsthis: & QActionEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN12QActionEventD2Ev()};
     unsafe {C_ZN12QActionEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMouseEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QMouseEvent {
    return QMouseEvent{qbase: QInputEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QMouseEvent {
  type Target = QInputEvent;

  fn deref(&self) -> &QInputEvent {
    return & self.qbase;
  }
}
impl AsRef<QInputEvent> for QMouseEvent {
  fn as_ref(& self) -> & QInputEvent {
    return & self.qbase;
  }
}
  // proto:  QPoint QMouseEvent::globalPos();
impl /*struct*/ QMouseEvent {
  pub fn globalPos<RetType, T: QMouseEvent_globalPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalPos(self);
    // return 1;
  }
}

pub trait QMouseEvent_globalPos<RetType> {
  fn globalPos(self , rsthis: & QMouseEvent) -> RetType;
}

  // proto:  QPoint QMouseEvent::globalPos();
impl<'a> /*trait*/ QMouseEvent_globalPos<QPoint> for () {
  fn globalPos(self , rsthis: & QMouseEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent9globalPosEv()};
    let mut ret = unsafe {C_ZNK11QMouseEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QMouseEvent::y();
impl /*struct*/ QMouseEvent {
  pub fn y<RetType, T: QMouseEvent_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QMouseEvent_y<RetType> {
  fn y(self , rsthis: & QMouseEvent) -> RetType;
}

  // proto:  int QMouseEvent::y();
impl<'a> /*trait*/ QMouseEvent_y<i32> for () {
  fn y(self , rsthis: & QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent1yEv()};
    let mut ret = unsafe {C_ZNK11QMouseEvent1yEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  const QPointF & QMouseEvent::screenPos();
impl /*struct*/ QMouseEvent {
  pub fn screenPos<RetType, T: QMouseEvent_screenPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screenPos(self);
    // return 1;
  }
}

pub trait QMouseEvent_screenPos<RetType> {
  fn screenPos(self , rsthis: & QMouseEvent) -> RetType;
}

  // proto:  const QPointF & QMouseEvent::screenPos();
impl<'a> /*trait*/ QMouseEvent_screenPos<QPointF> for () {
  fn screenPos(self , rsthis: & QMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent9screenPosEv()};
    let mut ret = unsafe {C_ZNK11QMouseEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QMouseEvent::x();
impl /*struct*/ QMouseEvent {
  pub fn x<RetType, T: QMouseEvent_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QMouseEvent_x<RetType> {
  fn x(self , rsthis: & QMouseEvent) -> RetType;
}

  // proto:  int QMouseEvent::x();
impl<'a> /*trait*/ QMouseEvent_x<i32> for () {
  fn x(self , rsthis: & QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent1xEv()};
    let mut ret = unsafe {C_ZNK11QMouseEvent1xEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  const QPointF & QMouseEvent::localPos();
impl /*struct*/ QMouseEvent {
  pub fn localPos<RetType, T: QMouseEvent_localPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.localPos(self);
    // return 1;
  }
}

pub trait QMouseEvent_localPos<RetType> {
  fn localPos(self , rsthis: & QMouseEvent) -> RetType;
}

  // proto:  const QPointF & QMouseEvent::localPos();
impl<'a> /*trait*/ QMouseEvent_localPos<QPointF> for () {
  fn localPos(self , rsthis: & QMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent8localPosEv()};
    let mut ret = unsafe {C_ZNK11QMouseEvent8localPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QMouseEvent::globalX();
impl /*struct*/ QMouseEvent {
  pub fn globalX<RetType, T: QMouseEvent_globalX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalX(self);
    // return 1;
  }
}

pub trait QMouseEvent_globalX<RetType> {
  fn globalX(self , rsthis: & QMouseEvent) -> RetType;
}

  // proto:  int QMouseEvent::globalX();
impl<'a> /*trait*/ QMouseEvent_globalX<i32> for () {
  fn globalX(self , rsthis: & QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent7globalXEv()};
    let mut ret = unsafe {C_ZNK11QMouseEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  const QPointF & QMouseEvent::windowPos();
impl /*struct*/ QMouseEvent {
  pub fn windowPos<RetType, T: QMouseEvent_windowPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.windowPos(self);
    // return 1;
  }
}

pub trait QMouseEvent_windowPos<RetType> {
  fn windowPos(self , rsthis: & QMouseEvent) -> RetType;
}

  // proto:  const QPointF & QMouseEvent::windowPos();
impl<'a> /*trait*/ QMouseEvent_windowPos<QPointF> for () {
  fn windowPos(self , rsthis: & QMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent9windowPosEv()};
    let mut ret = unsafe {C_ZNK11QMouseEvent9windowPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMouseEvent::~QMouseEvent();
impl /*struct*/ QMouseEvent {
  pub fn free<RetType, T: QMouseEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QMouseEvent_free<RetType> {
  fn free(self , rsthis: & QMouseEvent) -> RetType;
}

  // proto:  void QMouseEvent::~QMouseEvent();
impl<'a> /*trait*/ QMouseEvent_free<()> for () {
  fn free(self , rsthis: & QMouseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMouseEventD2Ev()};
     unsafe {C_ZN11QMouseEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QMouseEvent::globalY();
impl /*struct*/ QMouseEvent {
  pub fn globalY<RetType, T: QMouseEvent_globalY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalY(self);
    // return 1;
  }
}

pub trait QMouseEvent_globalY<RetType> {
  fn globalY(self , rsthis: & QMouseEvent) -> RetType;
}

  // proto:  int QMouseEvent::globalY();
impl<'a> /*trait*/ QMouseEvent_globalY<i32> for () {
  fn globalY(self , rsthis: & QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent7globalYEv()};
    let mut ret = unsafe {C_ZNK11QMouseEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QPoint QMouseEvent::pos();
impl /*struct*/ QMouseEvent {
  pub fn pos<RetType, T: QMouseEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QMouseEvent_pos<RetType> {
  fn pos(self , rsthis: & QMouseEvent) -> RetType;
}

  // proto:  QPoint QMouseEvent::pos();
impl<'a> /*trait*/ QMouseEvent_pos<QPoint> for () {
  fn pos(self , rsthis: & QMouseEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent3posEv()};
    let mut ret = unsafe {C_ZNK11QMouseEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileOpenEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QFileOpenEvent {
    return QFileOpenEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QFileOpenEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QFileOpenEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QFileOpenEvent::QFileOpenEvent(const QString & file);
impl /*struct*/ QFileOpenEvent {
  pub fn new<T: QFileOpenEvent_new>(value: T) -> QFileOpenEvent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QFileOpenEvent_new {
  fn new(self) -> QFileOpenEvent;
}

  // proto:  void QFileOpenEvent::QFileOpenEvent(const QString & file);
impl<'a> /*trait*/ QFileOpenEvent_new for (&'a QString) {
  fn new(self) -> QFileOpenEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN14QFileOpenEventC2ERK7QString()};
    let ctysz: c_int = unsafe{QFileOpenEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN14QFileOpenEventC2ERK7QString(arg0)};
    let rsthis = QFileOpenEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFileOpenEvent::~QFileOpenEvent();
impl /*struct*/ QFileOpenEvent {
  pub fn free<RetType, T: QFileOpenEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QFileOpenEvent_free<RetType> {
  fn free(self , rsthis: & QFileOpenEvent) -> RetType;
}

  // proto:  void QFileOpenEvent::~QFileOpenEvent();
impl<'a> /*trait*/ QFileOpenEvent_free<()> for () {
  fn free(self , rsthis: & QFileOpenEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN14QFileOpenEventD2Ev()};
     unsafe {C_ZN14QFileOpenEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFileOpenEvent::QFileOpenEvent(const QUrl & url);
impl<'a> /*trait*/ QFileOpenEvent_new for (&'a QUrl) {
  fn new(self) -> QFileOpenEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN14QFileOpenEventC2ERK4QUrl()};
    let ctysz: c_int = unsafe{QFileOpenEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN14QFileOpenEventC2ERK4QUrl(arg0)};
    let rsthis = QFileOpenEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QFileOpenEvent::file();
impl /*struct*/ QFileOpenEvent {
  pub fn file<RetType, T: QFileOpenEvent_file<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.file(self);
    // return 1;
  }
}

pub trait QFileOpenEvent_file<RetType> {
  fn file(self , rsthis: & QFileOpenEvent) -> RetType;
}

  // proto:  QString QFileOpenEvent::file();
impl<'a> /*trait*/ QFileOpenEvent_file<QString> for () {
  fn file(self , rsthis: & QFileOpenEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK14QFileOpenEvent4fileEv()};
    let mut ret = unsafe {C_ZNK14QFileOpenEvent4fileEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QUrl QFileOpenEvent::url();
impl /*struct*/ QFileOpenEvent {
  pub fn url<RetType, T: QFileOpenEvent_url<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.url(self);
    // return 1;
  }
}

pub trait QFileOpenEvent_url<RetType> {
  fn url(self , rsthis: & QFileOpenEvent) -> RetType;
}

  // proto:  QUrl QFileOpenEvent::url();
impl<'a> /*trait*/ QFileOpenEvent_url<QUrl> for () {
  fn url(self , rsthis: & QFileOpenEvent) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK14QFileOpenEvent3urlEv()};
    let mut ret = unsafe {C_ZNK14QFileOpenEvent3urlEv(rsthis.qclsinst)};
    let mut ret1 = QUrl::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QToolBarChangeEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QToolBarChangeEvent {
    return QToolBarChangeEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QToolBarChangeEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QToolBarChangeEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QToolBarChangeEvent::QToolBarChangeEvent(bool t);
impl /*struct*/ QToolBarChangeEvent {
  pub fn new<T: QToolBarChangeEvent_new>(value: T) -> QToolBarChangeEvent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QToolBarChangeEvent_new {
  fn new(self) -> QToolBarChangeEvent;
}

  // proto:  void QToolBarChangeEvent::QToolBarChangeEvent(bool t);
impl<'a> /*trait*/ QToolBarChangeEvent_new for (i8) {
  fn new(self) -> QToolBarChangeEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QToolBarChangeEventC2Eb()};
    let ctysz: c_int = unsafe{QToolBarChangeEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_char;
    let qthis: u64 = unsafe {C_ZN19QToolBarChangeEventC2Eb(arg0)};
    let rsthis = QToolBarChangeEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QToolBarChangeEvent::~QToolBarChangeEvent();
impl /*struct*/ QToolBarChangeEvent {
  pub fn free<RetType, T: QToolBarChangeEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QToolBarChangeEvent_free<RetType> {
  fn free(self , rsthis: & QToolBarChangeEvent) -> RetType;
}

  // proto:  void QToolBarChangeEvent::~QToolBarChangeEvent();
impl<'a> /*trait*/ QToolBarChangeEvent_free<()> for () {
  fn free(self , rsthis: & QToolBarChangeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QToolBarChangeEventD2Ev()};
     unsafe {C_ZN19QToolBarChangeEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QToolBarChangeEvent::toggle();
impl /*struct*/ QToolBarChangeEvent {
  pub fn toggle<RetType, T: QToolBarChangeEvent_toggle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toggle(self);
    // return 1;
  }
}

pub trait QToolBarChangeEvent_toggle<RetType> {
  fn toggle(self , rsthis: & QToolBarChangeEvent) -> RetType;
}

  // proto:  bool QToolBarChangeEvent::toggle();
impl<'a> /*trait*/ QToolBarChangeEvent_toggle<i8> for () {
  fn toggle(self , rsthis: & QToolBarChangeEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QToolBarChangeEvent6toggleEv()};
    let mut ret = unsafe {C_ZNK19QToolBarChangeEvent6toggleEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTabletEvent {
    return QTabletEvent{qbase: QInputEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QTabletEvent {
  type Target = QInputEvent;

  fn deref(&self) -> &QInputEvent {
    return & self.qbase;
  }
}
impl AsRef<QInputEvent> for QTabletEvent {
  fn as_ref(& self) -> & QInputEvent {
    return & self.qbase;
  }
}
  // proto:  int QTabletEvent::x();
impl /*struct*/ QTabletEvent {
  pub fn x<RetType, T: QTabletEvent_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QTabletEvent_x<RetType> {
  fn x(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  int QTabletEvent::x();
impl<'a> /*trait*/ QTabletEvent_x<i32> for () {
  fn x(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent1xEv()};
    let mut ret = unsafe {C_ZNK12QTabletEvent1xEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QTabletEvent::xTilt();
impl /*struct*/ QTabletEvent {
  pub fn xTilt<RetType, T: QTabletEvent_xTilt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.xTilt(self);
    // return 1;
  }
}

pub trait QTabletEvent_xTilt<RetType> {
  fn xTilt(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  int QTabletEvent::xTilt();
impl<'a> /*trait*/ QTabletEvent_xTilt<i32> for () {
  fn xTilt(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent5xTiltEv()};
    let mut ret = unsafe {C_ZNK12QTabletEvent5xTiltEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  qint64 QTabletEvent::uniqueId();
impl /*struct*/ QTabletEvent {
  pub fn uniqueId<RetType, T: QTabletEvent_uniqueId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.uniqueId(self);
    // return 1;
  }
}

pub trait QTabletEvent_uniqueId<RetType> {
  fn uniqueId(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  qint64 QTabletEvent::uniqueId();
impl<'a> /*trait*/ QTabletEvent_uniqueId<i64> for () {
  fn uniqueId(self , rsthis: & QTabletEvent) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent8uniqueIdEv()};
    let mut ret = unsafe {C_ZNK12QTabletEvent8uniqueIdEv(rsthis.qclsinst)};
    return ret as i64; // 1
    // return 1;
  }
}

  // proto:  const QPointF & QTabletEvent::globalPosF();
impl /*struct*/ QTabletEvent {
  pub fn globalPosF<RetType, T: QTabletEvent_globalPosF<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalPosF(self);
    // return 1;
  }
}

pub trait QTabletEvent_globalPosF<RetType> {
  fn globalPosF(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  const QPointF & QTabletEvent::globalPosF();
impl<'a> /*trait*/ QTabletEvent_globalPosF<QPointF> for () {
  fn globalPosF(self , rsthis: & QTabletEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent10globalPosFEv()};
    let mut ret = unsafe {C_ZNK12QTabletEvent10globalPosFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTabletEvent::z();
impl /*struct*/ QTabletEvent {
  pub fn z<RetType, T: QTabletEvent_z<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.z(self);
    // return 1;
  }
}

pub trait QTabletEvent_z<RetType> {
  fn z(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  int QTabletEvent::z();
impl<'a> /*trait*/ QTabletEvent_z<i32> for () {
  fn z(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent1zEv()};
    let mut ret = unsafe {C_ZNK12QTabletEvent1zEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QTabletEvent::y();
impl /*struct*/ QTabletEvent {
  pub fn y<RetType, T: QTabletEvent_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QTabletEvent_y<RetType> {
  fn y(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  int QTabletEvent::y();
impl<'a> /*trait*/ QTabletEvent_y<i32> for () {
  fn y(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent1yEv()};
    let mut ret = unsafe {C_ZNK12QTabletEvent1yEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QPoint QTabletEvent::pos();
impl /*struct*/ QTabletEvent {
  pub fn pos<RetType, T: QTabletEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QTabletEvent_pos<RetType> {
  fn pos(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  QPoint QTabletEvent::pos();
impl<'a> /*trait*/ QTabletEvent_pos<QPoint> for () {
  fn pos(self , rsthis: & QTabletEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent3posEv()};
    let mut ret = unsafe {C_ZNK12QTabletEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QTabletEvent::rotation();
impl /*struct*/ QTabletEvent {
  pub fn rotation<RetType, T: QTabletEvent_rotation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rotation(self);
    // return 1;
  }
}

pub trait QTabletEvent_rotation<RetType> {
  fn rotation(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  qreal QTabletEvent::rotation();
impl<'a> /*trait*/ QTabletEvent_rotation<f64> for () {
  fn rotation(self , rsthis: & QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent8rotationEv()};
    let mut ret = unsafe {C_ZNK12QTabletEvent8rotationEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  QPoint QTabletEvent::globalPos();
impl /*struct*/ QTabletEvent {
  pub fn globalPos<RetType, T: QTabletEvent_globalPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalPos(self);
    // return 1;
  }
}

pub trait QTabletEvent_globalPos<RetType> {
  fn globalPos(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  QPoint QTabletEvent::globalPos();
impl<'a> /*trait*/ QTabletEvent_globalPos<QPoint> for () {
  fn globalPos(self , rsthis: & QTabletEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent9globalPosEv()};
    let mut ret = unsafe {C_ZNK12QTabletEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTabletEvent::~QTabletEvent();
impl /*struct*/ QTabletEvent {
  pub fn free<RetType, T: QTabletEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTabletEvent_free<RetType> {
  fn free(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  void QTabletEvent::~QTabletEvent();
impl<'a> /*trait*/ QTabletEvent_free<()> for () {
  fn free(self , rsthis: & QTabletEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTabletEventD2Ev()};
     unsafe {C_ZN12QTabletEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QTabletEvent::tangentialPressure();
impl /*struct*/ QTabletEvent {
  pub fn tangentialPressure<RetType, T: QTabletEvent_tangentialPressure<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tangentialPressure(self);
    // return 1;
  }
}

pub trait QTabletEvent_tangentialPressure<RetType> {
  fn tangentialPressure(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  qreal QTabletEvent::tangentialPressure();
impl<'a> /*trait*/ QTabletEvent_tangentialPressure<f64> for () {
  fn tangentialPressure(self , rsthis: & QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent18tangentialPressureEv()};
    let mut ret = unsafe {C_ZNK12QTabletEvent18tangentialPressureEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  qreal QTabletEvent::hiResGlobalX();
impl /*struct*/ QTabletEvent {
  pub fn hiResGlobalX<RetType, T: QTabletEvent_hiResGlobalX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hiResGlobalX(self);
    // return 1;
  }
}

pub trait QTabletEvent_hiResGlobalX<RetType> {
  fn hiResGlobalX(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  qreal QTabletEvent::hiResGlobalX();
impl<'a> /*trait*/ QTabletEvent_hiResGlobalX<f64> for () {
  fn hiResGlobalX(self , rsthis: & QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent12hiResGlobalXEv()};
    let mut ret = unsafe {C_ZNK12QTabletEvent12hiResGlobalXEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  int QTabletEvent::globalY();
impl /*struct*/ QTabletEvent {
  pub fn globalY<RetType, T: QTabletEvent_globalY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalY(self);
    // return 1;
  }
}

pub trait QTabletEvent_globalY<RetType> {
  fn globalY(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  int QTabletEvent::globalY();
impl<'a> /*trait*/ QTabletEvent_globalY<i32> for () {
  fn globalY(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent7globalYEv()};
    let mut ret = unsafe {C_ZNK12QTabletEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  qreal QTabletEvent::hiResGlobalY();
impl /*struct*/ QTabletEvent {
  pub fn hiResGlobalY<RetType, T: QTabletEvent_hiResGlobalY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hiResGlobalY(self);
    // return 1;
  }
}

pub trait QTabletEvent_hiResGlobalY<RetType> {
  fn hiResGlobalY(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  qreal QTabletEvent::hiResGlobalY();
impl<'a> /*trait*/ QTabletEvent_hiResGlobalY<f64> for () {
  fn hiResGlobalY(self , rsthis: & QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent12hiResGlobalYEv()};
    let mut ret = unsafe {C_ZNK12QTabletEvent12hiResGlobalYEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  int QTabletEvent::globalX();
impl /*struct*/ QTabletEvent {
  pub fn globalX<RetType, T: QTabletEvent_globalX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalX(self);
    // return 1;
  }
}

pub trait QTabletEvent_globalX<RetType> {
  fn globalX(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  int QTabletEvent::globalX();
impl<'a> /*trait*/ QTabletEvent_globalX<i32> for () {
  fn globalX(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent7globalXEv()};
    let mut ret = unsafe {C_ZNK12QTabletEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  const QPointF & QTabletEvent::posF();
impl /*struct*/ QTabletEvent {
  pub fn posF<RetType, T: QTabletEvent_posF<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.posF(self);
    // return 1;
  }
}

pub trait QTabletEvent_posF<RetType> {
  fn posF(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  const QPointF & QTabletEvent::posF();
impl<'a> /*trait*/ QTabletEvent_posF<QPointF> for () {
  fn posF(self , rsthis: & QTabletEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent4posFEv()};
    let mut ret = unsafe {C_ZNK12QTabletEvent4posFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QTabletEvent::pressure();
impl /*struct*/ QTabletEvent {
  pub fn pressure<RetType, T: QTabletEvent_pressure<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pressure(self);
    // return 1;
  }
}

pub trait QTabletEvent_pressure<RetType> {
  fn pressure(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  qreal QTabletEvent::pressure();
impl<'a> /*trait*/ QTabletEvent_pressure<f64> for () {
  fn pressure(self , rsthis: & QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent8pressureEv()};
    let mut ret = unsafe {C_ZNK12QTabletEvent8pressureEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  int QTabletEvent::yTilt();
impl /*struct*/ QTabletEvent {
  pub fn yTilt<RetType, T: QTabletEvent_yTilt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.yTilt(self);
    // return 1;
  }
}

pub trait QTabletEvent_yTilt<RetType> {
  fn yTilt(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  int QTabletEvent::yTilt();
impl<'a> /*trait*/ QTabletEvent_yTilt<i32> for () {
  fn yTilt(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent5yTiltEv()};
    let mut ret = unsafe {C_ZNK12QTabletEvent5yTiltEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

impl /*struct*/ QTouchEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTouchEvent {
    return QTouchEvent{qbase: QInputEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QTouchEvent {
  type Target = QInputEvent;

  fn deref(&self) -> &QInputEvent {
    return & self.qbase;
  }
}
impl AsRef<QInputEvent> for QTouchEvent {
  fn as_ref(& self) -> & QInputEvent {
    return & self.qbase;
  }
}
  // proto:  void QTouchEvent::setDevice(QTouchDevice * adevice);
impl /*struct*/ QTouchEvent {
  pub fn setDevice<RetType, T: QTouchEvent_setDevice<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDevice(self);
    // return 1;
  }
}

pub trait QTouchEvent_setDevice<RetType> {
  fn setDevice(self , rsthis: & QTouchEvent) -> RetType;
}

  // proto:  void QTouchEvent::setDevice(QTouchDevice * adevice);
impl<'a> /*trait*/ QTouchEvent_setDevice<()> for (&'a QTouchDevice) {
  fn setDevice(self , rsthis: & QTouchEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTouchEvent9setDeviceEP12QTouchDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QTouchEvent9setDeviceEP12QTouchDevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWindow * QTouchEvent::window();
impl /*struct*/ QTouchEvent {
  pub fn window<RetType, T: QTouchEvent_window<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.window(self);
    // return 1;
  }
}

pub trait QTouchEvent_window<RetType> {
  fn window(self , rsthis: & QTouchEvent) -> RetType;
}

  // proto:  QWindow * QTouchEvent::window();
impl<'a> /*trait*/ QTouchEvent_window<QWindow> for () {
  fn window(self , rsthis: & QTouchEvent) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTouchEvent6windowEv()};
    let mut ret = unsafe {C_ZNK11QTouchEvent6windowEv(rsthis.qclsinst)};
    let mut ret1 = QWindow::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTouchDevice * QTouchEvent::device();
impl /*struct*/ QTouchEvent {
  pub fn device<RetType, T: QTouchEvent_device<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.device(self);
    // return 1;
  }
}

pub trait QTouchEvent_device<RetType> {
  fn device(self , rsthis: & QTouchEvent) -> RetType;
}

  // proto:  QTouchDevice * QTouchEvent::device();
impl<'a> /*trait*/ QTouchEvent_device<QTouchDevice> for () {
  fn device(self , rsthis: & QTouchEvent) -> QTouchDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTouchEvent6deviceEv()};
    let mut ret = unsafe {C_ZNK11QTouchEvent6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QTouchDevice::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QObject * QTouchEvent::target();
impl /*struct*/ QTouchEvent {
  pub fn target<RetType, T: QTouchEvent_target<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.target(self);
    // return 1;
  }
}

pub trait QTouchEvent_target<RetType> {
  fn target(self , rsthis: & QTouchEvent) -> RetType;
}

  // proto:  QObject * QTouchEvent::target();
impl<'a> /*trait*/ QTouchEvent_target<QObject> for () {
  fn target(self , rsthis: & QTouchEvent) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTouchEvent6targetEv()};
    let mut ret = unsafe {C_ZNK11QTouchEvent6targetEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTouchEvent::~QTouchEvent();
impl /*struct*/ QTouchEvent {
  pub fn free<RetType, T: QTouchEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTouchEvent_free<RetType> {
  fn free(self , rsthis: & QTouchEvent) -> RetType;
}

  // proto:  void QTouchEvent::~QTouchEvent();
impl<'a> /*trait*/ QTouchEvent_free<()> for () {
  fn free(self , rsthis: & QTouchEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTouchEventD2Ev()};
     unsafe {C_ZN11QTouchEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTouchEvent::setWindow(QWindow * awindow);
impl /*struct*/ QTouchEvent {
  pub fn setWindow<RetType, T: QTouchEvent_setWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWindow(self);
    // return 1;
  }
}

pub trait QTouchEvent_setWindow<RetType> {
  fn setWindow(self , rsthis: & QTouchEvent) -> RetType;
}

  // proto:  void QTouchEvent::setWindow(QWindow * awindow);
impl<'a> /*trait*/ QTouchEvent_setWindow<()> for (&'a QWindow) {
  fn setWindow(self , rsthis: & QTouchEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTouchEvent9setWindowEP7QWindow()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QTouchEvent9setWindowEP7QWindow(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTouchEvent::setTarget(QObject * atarget);
impl /*struct*/ QTouchEvent {
  pub fn setTarget<RetType, T: QTouchEvent_setTarget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTarget(self);
    // return 1;
  }
}

pub trait QTouchEvent_setTarget<RetType> {
  fn setTarget(self , rsthis: & QTouchEvent) -> RetType;
}

  // proto:  void QTouchEvent::setTarget(QObject * atarget);
impl<'a> /*trait*/ QTouchEvent_setTarget<()> for (&'a QObject) {
  fn setTarget(self , rsthis: & QTouchEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTouchEvent9setTargetEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QTouchEvent9setTargetEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QScreenOrientationChangeEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QScreenOrientationChangeEvent {
    return QScreenOrientationChangeEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QScreenOrientationChangeEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QScreenOrientationChangeEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  QScreen * QScreenOrientationChangeEvent::screen();
impl /*struct*/ QScreenOrientationChangeEvent {
  pub fn screen<RetType, T: QScreenOrientationChangeEvent_screen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screen(self);
    // return 1;
  }
}

pub trait QScreenOrientationChangeEvent_screen<RetType> {
  fn screen(self , rsthis: & QScreenOrientationChangeEvent) -> RetType;
}

  // proto:  QScreen * QScreenOrientationChangeEvent::screen();
impl<'a> /*trait*/ QScreenOrientationChangeEvent_screen<QScreen> for () {
  fn screen(self , rsthis: & QScreenOrientationChangeEvent) -> QScreen {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK29QScreenOrientationChangeEvent6screenEv()};
    let mut ret = unsafe {C_ZNK29QScreenOrientationChangeEvent6screenEv(rsthis.qclsinst)};
    let mut ret1 = QScreen::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QScreenOrientationChangeEvent::~QScreenOrientationChangeEvent();
impl /*struct*/ QScreenOrientationChangeEvent {
  pub fn free<RetType, T: QScreenOrientationChangeEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QScreenOrientationChangeEvent_free<RetType> {
  fn free(self , rsthis: & QScreenOrientationChangeEvent) -> RetType;
}

  // proto:  void QScreenOrientationChangeEvent::~QScreenOrientationChangeEvent();
impl<'a> /*trait*/ QScreenOrientationChangeEvent_free<()> for () {
  fn free(self , rsthis: & QScreenOrientationChangeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN29QScreenOrientationChangeEventD2Ev()};
     unsafe {C_ZN29QScreenOrientationChangeEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QIconDragEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QIconDragEvent {
    return QIconDragEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QIconDragEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QIconDragEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QIconDragEvent::~QIconDragEvent();
impl /*struct*/ QIconDragEvent {
  pub fn free<RetType, T: QIconDragEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QIconDragEvent_free<RetType> {
  fn free(self , rsthis: & QIconDragEvent) -> RetType;
}

  // proto:  void QIconDragEvent::~QIconDragEvent();
impl<'a> /*trait*/ QIconDragEvent_free<()> for () {
  fn free(self , rsthis: & QIconDragEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QIconDragEventD2Ev()};
     unsafe {C_ZN14QIconDragEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QIconDragEvent::QIconDragEvent();
impl /*struct*/ QIconDragEvent {
  pub fn new<T: QIconDragEvent_new>(value: T) -> QIconDragEvent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QIconDragEvent_new {
  fn new(self) -> QIconDragEvent;
}

  // proto:  void QIconDragEvent::QIconDragEvent();
impl<'a> /*trait*/ QIconDragEvent_new for () {
  fn new(self) -> QIconDragEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QIconDragEventC2Ev()};
    let ctysz: c_int = unsafe{QIconDragEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN14QIconDragEventC2Ev()};
    let rsthis = QIconDragEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCloseEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QCloseEvent {
    return QCloseEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QCloseEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QCloseEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QCloseEvent::~QCloseEvent();
impl /*struct*/ QCloseEvent {
  pub fn free<RetType, T: QCloseEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QCloseEvent_free<RetType> {
  fn free(self , rsthis: & QCloseEvent) -> RetType;
}

  // proto:  void QCloseEvent::~QCloseEvent();
impl<'a> /*trait*/ QCloseEvent_free<()> for () {
  fn free(self , rsthis: & QCloseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QCloseEventD2Ev()};
     unsafe {C_ZN11QCloseEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCloseEvent::QCloseEvent();
impl /*struct*/ QCloseEvent {
  pub fn new<T: QCloseEvent_new>(value: T) -> QCloseEvent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QCloseEvent_new {
  fn new(self) -> QCloseEvent;
}

  // proto:  void QCloseEvent::QCloseEvent();
impl<'a> /*trait*/ QCloseEvent_new for () {
  fn new(self) -> QCloseEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QCloseEventC2Ev()};
    let ctysz: c_int = unsafe{QCloseEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN11QCloseEventC2Ev()};
    let rsthis = QCloseEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDragEnterEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDragEnterEvent {
    return QDragEnterEvent{qbase: QDragMoveEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QDragEnterEvent {
  type Target = QDragMoveEvent;

  fn deref(&self) -> &QDragMoveEvent {
    return & self.qbase;
  }
}
impl AsRef<QDragMoveEvent> for QDragEnterEvent {
  fn as_ref(& self) -> & QDragMoveEvent {
    return & self.qbase;
  }
}
  // proto:  void QDragEnterEvent::~QDragEnterEvent();
impl /*struct*/ QDragEnterEvent {
  pub fn free<RetType, T: QDragEnterEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QDragEnterEvent_free<RetType> {
  fn free(self , rsthis: & QDragEnterEvent) -> RetType;
}

  // proto:  void QDragEnterEvent::~QDragEnterEvent();
impl<'a> /*trait*/ QDragEnterEvent_free<()> for () {
  fn free(self , rsthis: & QDragEnterEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QDragEnterEventD2Ev()};
     unsafe {C_ZN15QDragEnterEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QWheelEvent {
    return QWheelEvent{qbase: QInputEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QWheelEvent {
  type Target = QInputEvent;

  fn deref(&self) -> &QInputEvent {
    return & self.qbase;
  }
}
impl AsRef<QInputEvent> for QWheelEvent {
  fn as_ref(& self) -> & QInputEvent {
    return & self.qbase;
  }
}
  // proto:  int QWheelEvent::x();
impl /*struct*/ QWheelEvent {
  pub fn x<RetType, T: QWheelEvent_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QWheelEvent_x<RetType> {
  fn x(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  int QWheelEvent::x();
impl<'a> /*trait*/ QWheelEvent_x<i32> for () {
  fn x(self , rsthis: & QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent1xEv()};
    let mut ret = unsafe {C_ZNK11QWheelEvent1xEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QPoint QWheelEvent::angleDelta();
impl /*struct*/ QWheelEvent {
  pub fn angleDelta<RetType, T: QWheelEvent_angleDelta<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.angleDelta(self);
    // return 1;
  }
}

pub trait QWheelEvent_angleDelta<RetType> {
  fn angleDelta(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  QPoint QWheelEvent::angleDelta();
impl<'a> /*trait*/ QWheelEvent_angleDelta<QPoint> for () {
  fn angleDelta(self , rsthis: & QWheelEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent10angleDeltaEv()};
    let mut ret = unsafe {C_ZNK11QWheelEvent10angleDeltaEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QWheelEvent::pos();
impl /*struct*/ QWheelEvent {
  pub fn pos<RetType, T: QWheelEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QWheelEvent_pos<RetType> {
  fn pos(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  QPoint QWheelEvent::pos();
impl<'a> /*trait*/ QWheelEvent_pos<QPoint> for () {
  fn pos(self , rsthis: & QWheelEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent3posEv()};
    let mut ret = unsafe {C_ZNK11QWheelEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QWheelEvent::globalY();
impl /*struct*/ QWheelEvent {
  pub fn globalY<RetType, T: QWheelEvent_globalY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalY(self);
    // return 1;
  }
}

pub trait QWheelEvent_globalY<RetType> {
  fn globalY(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  int QWheelEvent::globalY();
impl<'a> /*trait*/ QWheelEvent_globalY<i32> for () {
  fn globalY(self , rsthis: & QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent7globalYEv()};
    let mut ret = unsafe {C_ZNK11QWheelEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  const QPointF & QWheelEvent::posF();
impl /*struct*/ QWheelEvent {
  pub fn posF<RetType, T: QWheelEvent_posF<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.posF(self);
    // return 1;
  }
}

pub trait QWheelEvent_posF<RetType> {
  fn posF(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  const QPointF & QWheelEvent::posF();
impl<'a> /*trait*/ QWheelEvent_posF<QPointF> for () {
  fn posF(self , rsthis: & QWheelEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent4posFEv()};
    let mut ret = unsafe {C_ZNK11QWheelEvent4posFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QWheelEvent::globalX();
impl /*struct*/ QWheelEvent {
  pub fn globalX<RetType, T: QWheelEvent_globalX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalX(self);
    // return 1;
  }
}

pub trait QWheelEvent_globalX<RetType> {
  fn globalX(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  int QWheelEvent::globalX();
impl<'a> /*trait*/ QWheelEvent_globalX<i32> for () {
  fn globalX(self , rsthis: & QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent7globalXEv()};
    let mut ret = unsafe {C_ZNK11QWheelEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QWheelEvent::y();
impl /*struct*/ QWheelEvent {
  pub fn y<RetType, T: QWheelEvent_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QWheelEvent_y<RetType> {
  fn y(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  int QWheelEvent::y();
impl<'a> /*trait*/ QWheelEvent_y<i32> for () {
  fn y(self , rsthis: & QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent1yEv()};
    let mut ret = unsafe {C_ZNK11QWheelEvent1yEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QWheelEvent::~QWheelEvent();
impl /*struct*/ QWheelEvent {
  pub fn free<RetType, T: QWheelEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QWheelEvent_free<RetType> {
  fn free(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  void QWheelEvent::~QWheelEvent();
impl<'a> /*trait*/ QWheelEvent_free<()> for () {
  fn free(self , rsthis: & QWheelEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWheelEventD2Ev()};
     unsafe {C_ZN11QWheelEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPoint QWheelEvent::pixelDelta();
impl /*struct*/ QWheelEvent {
  pub fn pixelDelta<RetType, T: QWheelEvent_pixelDelta<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pixelDelta(self);
    // return 1;
  }
}

pub trait QWheelEvent_pixelDelta<RetType> {
  fn pixelDelta(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  QPoint QWheelEvent::pixelDelta();
impl<'a> /*trait*/ QWheelEvent_pixelDelta<QPoint> for () {
  fn pixelDelta(self , rsthis: & QWheelEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent10pixelDeltaEv()};
    let mut ret = unsafe {C_ZNK11QWheelEvent10pixelDeltaEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QWheelEvent::delta();
impl /*struct*/ QWheelEvent {
  pub fn delta<RetType, T: QWheelEvent_delta<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.delta(self);
    // return 1;
  }
}

pub trait QWheelEvent_delta<RetType> {
  fn delta(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  int QWheelEvent::delta();
impl<'a> /*trait*/ QWheelEvent_delta<i32> for () {
  fn delta(self , rsthis: & QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent5deltaEv()};
    let mut ret = unsafe {C_ZNK11QWheelEvent5deltaEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QPoint QWheelEvent::globalPos();
impl /*struct*/ QWheelEvent {
  pub fn globalPos<RetType, T: QWheelEvent_globalPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalPos(self);
    // return 1;
  }
}

pub trait QWheelEvent_globalPos<RetType> {
  fn globalPos(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  QPoint QWheelEvent::globalPos();
impl<'a> /*trait*/ QWheelEvent_globalPos<QPoint> for () {
  fn globalPos(self , rsthis: & QWheelEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent9globalPosEv()};
    let mut ret = unsafe {C_ZNK11QWheelEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QPointF & QWheelEvent::globalPosF();
impl /*struct*/ QWheelEvent {
  pub fn globalPosF<RetType, T: QWheelEvent_globalPosF<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalPosF(self);
    // return 1;
  }
}

pub trait QWheelEvent_globalPosF<RetType> {
  fn globalPosF(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  const QPointF & QWheelEvent::globalPosF();
impl<'a> /*trait*/ QWheelEvent_globalPosF<QPointF> for () {
  fn globalPosF(self , rsthis: & QWheelEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent10globalPosFEv()};
    let mut ret = unsafe {C_ZNK11QWheelEvent10globalPosFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScrollEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QScrollEvent {
    return QScrollEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QScrollEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QScrollEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  QPointF QScrollEvent::contentPos();
impl /*struct*/ QScrollEvent {
  pub fn contentPos<RetType, T: QScrollEvent_contentPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contentPos(self);
    // return 1;
  }
}

pub trait QScrollEvent_contentPos<RetType> {
  fn contentPos(self , rsthis: & QScrollEvent) -> RetType;
}

  // proto:  QPointF QScrollEvent::contentPos();
impl<'a> /*trait*/ QScrollEvent_contentPos<QPointF> for () {
  fn contentPos(self , rsthis: & QScrollEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 64)};
    // unsafe{_ZNK12QScrollEvent10contentPosEv()};
    let mut ret = unsafe {C_ZNK12QScrollEvent10contentPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QScrollEvent::overshootDistance();
impl /*struct*/ QScrollEvent {
  pub fn overshootDistance<RetType, T: QScrollEvent_overshootDistance<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.overshootDistance(self);
    // return 1;
  }
}

pub trait QScrollEvent_overshootDistance<RetType> {
  fn overshootDistance(self , rsthis: & QScrollEvent) -> RetType;
}

  // proto:  QPointF QScrollEvent::overshootDistance();
impl<'a> /*trait*/ QScrollEvent_overshootDistance<QPointF> for () {
  fn overshootDistance(self , rsthis: & QScrollEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 64)};
    // unsafe{_ZNK12QScrollEvent17overshootDistanceEv()};
    let mut ret = unsafe {C_ZNK12QScrollEvent17overshootDistanceEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QScrollEvent::~QScrollEvent();
impl /*struct*/ QScrollEvent {
  pub fn free<RetType, T: QScrollEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QScrollEvent_free<RetType> {
  fn free(self , rsthis: & QScrollEvent) -> RetType;
}

  // proto:  void QScrollEvent::~QScrollEvent();
impl<'a> /*trait*/ QScrollEvent_free<()> for () {
  fn free(self , rsthis: & QScrollEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 64)};
    // unsafe{_ZN12QScrollEventD2Ev()};
     unsafe {C_ZN12QScrollEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QHoverEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QHoverEvent {
    return QHoverEvent{qbase: QInputEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QHoverEvent {
  type Target = QInputEvent;

  fn deref(&self) -> &QInputEvent {
    return & self.qbase;
  }
}
impl AsRef<QInputEvent> for QHoverEvent {
  fn as_ref(& self) -> & QInputEvent {
    return & self.qbase;
  }
}
  // proto:  void QHoverEvent::~QHoverEvent();
impl /*struct*/ QHoverEvent {
  pub fn free<RetType, T: QHoverEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QHoverEvent_free<RetType> {
  fn free(self , rsthis: & QHoverEvent) -> RetType;
}

  // proto:  void QHoverEvent::~QHoverEvent();
impl<'a> /*trait*/ QHoverEvent_free<()> for () {
  fn free(self , rsthis: & QHoverEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHoverEventD2Ev()};
     unsafe {C_ZN11QHoverEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QPointF & QHoverEvent::posF();
impl /*struct*/ QHoverEvent {
  pub fn posF<RetType, T: QHoverEvent_posF<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.posF(self);
    // return 1;
  }
}

pub trait QHoverEvent_posF<RetType> {
  fn posF(self , rsthis: & QHoverEvent) -> RetType;
}

  // proto:  const QPointF & QHoverEvent::posF();
impl<'a> /*trait*/ QHoverEvent_posF<QPointF> for () {
  fn posF(self , rsthis: & QHoverEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHoverEvent4posFEv()};
    let mut ret = unsafe {C_ZNK11QHoverEvent4posFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QHoverEvent::oldPos();
impl /*struct*/ QHoverEvent {
  pub fn oldPos<RetType, T: QHoverEvent_oldPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.oldPos(self);
    // return 1;
  }
}

pub trait QHoverEvent_oldPos<RetType> {
  fn oldPos(self , rsthis: & QHoverEvent) -> RetType;
}

  // proto:  QPoint QHoverEvent::oldPos();
impl<'a> /*trait*/ QHoverEvent_oldPos<QPoint> for () {
  fn oldPos(self , rsthis: & QHoverEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHoverEvent6oldPosEv()};
    let mut ret = unsafe {C_ZNK11QHoverEvent6oldPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QPointF & QHoverEvent::oldPosF();
impl /*struct*/ QHoverEvent {
  pub fn oldPosF<RetType, T: QHoverEvent_oldPosF<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.oldPosF(self);
    // return 1;
  }
}

pub trait QHoverEvent_oldPosF<RetType> {
  fn oldPosF(self , rsthis: & QHoverEvent) -> RetType;
}

  // proto:  const QPointF & QHoverEvent::oldPosF();
impl<'a> /*trait*/ QHoverEvent_oldPosF<QPointF> for () {
  fn oldPosF(self , rsthis: & QHoverEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHoverEvent7oldPosFEv()};
    let mut ret = unsafe {C_ZNK11QHoverEvent7oldPosFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QHoverEvent::pos();
impl /*struct*/ QHoverEvent {
  pub fn pos<RetType, T: QHoverEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QHoverEvent_pos<RetType> {
  fn pos(self , rsthis: & QHoverEvent) -> RetType;
}

  // proto:  QPoint QHoverEvent::pos();
impl<'a> /*trait*/ QHoverEvent_pos<QPoint> for () {
  fn pos(self , rsthis: & QHoverEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHoverEvent3posEv()};
    let mut ret = unsafe {C_ZNK11QHoverEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDragMoveEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDragMoveEvent {
    return QDragMoveEvent{qbase: QDropEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QDragMoveEvent {
  type Target = QDropEvent;

  fn deref(&self) -> &QDropEvent {
    return & self.qbase;
  }
}
impl AsRef<QDropEvent> for QDragMoveEvent {
  fn as_ref(& self) -> & QDropEvent {
    return & self.qbase;
  }
}
  // proto:  void QDragMoveEvent::accept(const QRect & r);
impl /*struct*/ QDragMoveEvent {
  pub fn accept<RetType, T: QDragMoveEvent_accept<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.accept(self);
    // return 1;
  }
}

pub trait QDragMoveEvent_accept<RetType> {
  fn accept(self , rsthis: & QDragMoveEvent) -> RetType;
}

  // proto:  void QDragMoveEvent::accept(const QRect & r);
impl<'a> /*trait*/ QDragMoveEvent_accept<()> for (&'a QRect) {
  fn accept(self , rsthis: & QDragMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDragMoveEvent6acceptERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN14QDragMoveEvent6acceptERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QDragMoveEvent::answerRect();
impl /*struct*/ QDragMoveEvent {
  pub fn answerRect<RetType, T: QDragMoveEvent_answerRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.answerRect(self);
    // return 1;
  }
}

pub trait QDragMoveEvent_answerRect<RetType> {
  fn answerRect(self , rsthis: & QDragMoveEvent) -> RetType;
}

  // proto:  QRect QDragMoveEvent::answerRect();
impl<'a> /*trait*/ QDragMoveEvent_answerRect<QRect> for () {
  fn answerRect(self , rsthis: & QDragMoveEvent) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDragMoveEvent10answerRectEv()};
    let mut ret = unsafe {C_ZNK14QDragMoveEvent10answerRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDragMoveEvent::ignore(const QRect & r);
impl /*struct*/ QDragMoveEvent {
  pub fn ignore<RetType, T: QDragMoveEvent_ignore<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ignore(self);
    // return 1;
  }
}

pub trait QDragMoveEvent_ignore<RetType> {
  fn ignore(self , rsthis: & QDragMoveEvent) -> RetType;
}

  // proto:  void QDragMoveEvent::ignore(const QRect & r);
impl<'a> /*trait*/ QDragMoveEvent_ignore<()> for (&'a QRect) {
  fn ignore(self , rsthis: & QDragMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDragMoveEvent6ignoreERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN14QDragMoveEvent6ignoreERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDragMoveEvent::ignore();
impl<'a> /*trait*/ QDragMoveEvent_ignore<()> for () {
  fn ignore(self , rsthis: & QDragMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDragMoveEvent6ignoreEv()};
     unsafe {C_ZN14QDragMoveEvent6ignoreEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDragMoveEvent::~QDragMoveEvent();
impl /*struct*/ QDragMoveEvent {
  pub fn free<RetType, T: QDragMoveEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QDragMoveEvent_free<RetType> {
  fn free(self , rsthis: & QDragMoveEvent) -> RetType;
}

  // proto:  void QDragMoveEvent::~QDragMoveEvent();
impl<'a> /*trait*/ QDragMoveEvent_free<()> for () {
  fn free(self , rsthis: & QDragMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDragMoveEventD2Ev()};
     unsafe {C_ZN14QDragMoveEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDragMoveEvent::accept();
impl<'a> /*trait*/ QDragMoveEvent_accept<()> for () {
  fn accept(self , rsthis: & QDragMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDragMoveEvent6acceptEv()};
     unsafe {C_ZN14QDragMoveEvent6acceptEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QShowEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QShowEvent {
    return QShowEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QShowEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QShowEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QShowEvent::~QShowEvent();
impl /*struct*/ QShowEvent {
  pub fn free<RetType, T: QShowEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QShowEvent_free<RetType> {
  fn free(self , rsthis: & QShowEvent) -> RetType;
}

  // proto:  void QShowEvent::~QShowEvent();
impl<'a> /*trait*/ QShowEvent_free<()> for () {
  fn free(self , rsthis: & QShowEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QShowEventD2Ev()};
     unsafe {C_ZN10QShowEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QShowEvent::QShowEvent();
impl /*struct*/ QShowEvent {
  pub fn new<T: QShowEvent_new>(value: T) -> QShowEvent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QShowEvent_new {
  fn new(self) -> QShowEvent;
}

  // proto:  void QShowEvent::QShowEvent();
impl<'a> /*trait*/ QShowEvent_new for () {
  fn new(self) -> QShowEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QShowEventC2Ev()};
    let ctysz: c_int = unsafe{QShowEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN10QShowEventC2Ev()};
    let rsthis = QShowEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPlatformSurfaceEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPlatformSurfaceEvent {
    return QPlatformSurfaceEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QPlatformSurfaceEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QPlatformSurfaceEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QPlatformSurfaceEvent::~QPlatformSurfaceEvent();
impl /*struct*/ QPlatformSurfaceEvent {
  pub fn free<RetType, T: QPlatformSurfaceEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QPlatformSurfaceEvent_free<RetType> {
  fn free(self , rsthis: & QPlatformSurfaceEvent) -> RetType;
}

  // proto:  void QPlatformSurfaceEvent::~QPlatformSurfaceEvent();
impl<'a> /*trait*/ QPlatformSurfaceEvent_free<()> for () {
  fn free(self , rsthis: & QPlatformSurfaceEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPlatformSurfaceEventD2Ev()};
     unsafe {C_ZN21QPlatformSurfaceEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPaintEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPaintEvent {
    return QPaintEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QPaintEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QPaintEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QPaintEvent::~QPaintEvent();
impl /*struct*/ QPaintEvent {
  pub fn free<RetType, T: QPaintEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QPaintEvent_free<RetType> {
  fn free(self , rsthis: & QPaintEvent) -> RetType;
}

  // proto:  void QPaintEvent::~QPaintEvent();
impl<'a> /*trait*/ QPaintEvent_free<()> for () {
  fn free(self , rsthis: & QPaintEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN11QPaintEventD2Ev()};
     unsafe {C_ZN11QPaintEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QRect & QPaintEvent::rect();
impl /*struct*/ QPaintEvent {
  pub fn rect<RetType, T: QPaintEvent_rect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rect(self);
    // return 1;
  }
}

pub trait QPaintEvent_rect<RetType> {
  fn rect(self , rsthis: & QPaintEvent) -> RetType;
}

  // proto:  const QRect & QPaintEvent::rect();
impl<'a> /*trait*/ QPaintEvent_rect<QRect> for () {
  fn rect(self , rsthis: & QPaintEvent) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK11QPaintEvent4rectEv()};
    let mut ret = unsafe {C_ZNK11QPaintEvent4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPaintEvent::QPaintEvent(const QRect & paintRect);
impl /*struct*/ QPaintEvent {
  pub fn new<T: QPaintEvent_new>(value: T) -> QPaintEvent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QPaintEvent_new {
  fn new(self) -> QPaintEvent;
}

  // proto:  void QPaintEvent::QPaintEvent(const QRect & paintRect);
impl<'a> /*trait*/ QPaintEvent_new for (&'a QRect) {
  fn new(self) -> QPaintEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN11QPaintEventC2ERK5QRect()};
    let ctysz: c_int = unsafe{QPaintEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QPaintEventC2ERK5QRect(arg0)};
    let rsthis = QPaintEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QRegion & QPaintEvent::region();
impl /*struct*/ QPaintEvent {
  pub fn region<RetType, T: QPaintEvent_region<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.region(self);
    // return 1;
  }
}

pub trait QPaintEvent_region<RetType> {
  fn region(self , rsthis: & QPaintEvent) -> RetType;
}

  // proto:  const QRegion & QPaintEvent::region();
impl<'a> /*trait*/ QPaintEvent_region<QRegion> for () {
  fn region(self , rsthis: & QPaintEvent) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK11QPaintEvent6regionEv()};
    let mut ret = unsafe {C_ZNK11QPaintEvent6regionEv(rsthis.qclsinst)};
    let mut ret1 = QRegion::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPaintEvent::QPaintEvent(const QRegion & paintRegion);
impl<'a> /*trait*/ QPaintEvent_new for (&'a QRegion) {
  fn new(self) -> QPaintEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN11QPaintEventC2ERK7QRegion()};
    let ctysz: c_int = unsafe{QPaintEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QPaintEventC2ERK7QRegion(arg0)};
    let rsthis = QPaintEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFocusEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QFocusEvent {
    return QFocusEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QFocusEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QFocusEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  bool QFocusEvent::lostFocus();
impl /*struct*/ QFocusEvent {
  pub fn lostFocus<RetType, T: QFocusEvent_lostFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lostFocus(self);
    // return 1;
  }
}

pub trait QFocusEvent_lostFocus<RetType> {
  fn lostFocus(self , rsthis: & QFocusEvent) -> RetType;
}

  // proto:  bool QFocusEvent::lostFocus();
impl<'a> /*trait*/ QFocusEvent_lostFocus<i8> for () {
  fn lostFocus(self , rsthis: & QFocusEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFocusEvent9lostFocusEv()};
    let mut ret = unsafe {C_ZNK11QFocusEvent9lostFocusEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QFocusEvent::gotFocus();
impl /*struct*/ QFocusEvent {
  pub fn gotFocus<RetType, T: QFocusEvent_gotFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.gotFocus(self);
    // return 1;
  }
}

pub trait QFocusEvent_gotFocus<RetType> {
  fn gotFocus(self , rsthis: & QFocusEvent) -> RetType;
}

  // proto:  bool QFocusEvent::gotFocus();
impl<'a> /*trait*/ QFocusEvent_gotFocus<i8> for () {
  fn gotFocus(self , rsthis: & QFocusEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFocusEvent8gotFocusEv()};
    let mut ret = unsafe {C_ZNK11QFocusEvent8gotFocusEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QFocusEvent::~QFocusEvent();
impl /*struct*/ QFocusEvent {
  pub fn free<RetType, T: QFocusEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QFocusEvent_free<RetType> {
  fn free(self , rsthis: & QFocusEvent) -> RetType;
}

  // proto:  void QFocusEvent::~QFocusEvent();
impl<'a> /*trait*/ QFocusEvent_free<()> for () {
  fn free(self , rsthis: & QFocusEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFocusEventD2Ev()};
     unsafe {C_ZN11QFocusEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QNativeGestureEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QNativeGestureEvent {
    return QNativeGestureEvent{qbase: QInputEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QNativeGestureEvent {
  type Target = QInputEvent;

  fn deref(&self) -> &QInputEvent {
    return & self.qbase;
  }
}
impl AsRef<QInputEvent> for QNativeGestureEvent {
  fn as_ref(& self) -> & QInputEvent {
    return & self.qbase;
  }
}
  // proto:  const QPointF & QNativeGestureEvent::localPos();
impl /*struct*/ QNativeGestureEvent {
  pub fn localPos<RetType, T: QNativeGestureEvent_localPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.localPos(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_localPos<RetType> {
  fn localPos(self , rsthis: & QNativeGestureEvent) -> RetType;
}

  // proto:  const QPointF & QNativeGestureEvent::localPos();
impl<'a> /*trait*/ QNativeGestureEvent_localPos<QPointF> for () {
  fn localPos(self , rsthis: & QNativeGestureEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent8localPosEv()};
    let mut ret = unsafe {C_ZNK19QNativeGestureEvent8localPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QPointF & QNativeGestureEvent::screenPos();
impl /*struct*/ QNativeGestureEvent {
  pub fn screenPos<RetType, T: QNativeGestureEvent_screenPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screenPos(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_screenPos<RetType> {
  fn screenPos(self , rsthis: & QNativeGestureEvent) -> RetType;
}

  // proto:  const QPointF & QNativeGestureEvent::screenPos();
impl<'a> /*trait*/ QNativeGestureEvent_screenPos<QPointF> for () {
  fn screenPos(self , rsthis: & QNativeGestureEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent9screenPosEv()};
    let mut ret = unsafe {C_ZNK19QNativeGestureEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QPoint QNativeGestureEvent::pos();
impl /*struct*/ QNativeGestureEvent {
  pub fn pos<RetType, T: QNativeGestureEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_pos<RetType> {
  fn pos(self , rsthis: & QNativeGestureEvent) -> RetType;
}

  // proto:  const QPoint QNativeGestureEvent::pos();
impl<'a> /*trait*/ QNativeGestureEvent_pos<QPoint> for () {
  fn pos(self , rsthis: & QNativeGestureEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent3posEv()};
    let mut ret = unsafe {C_ZNK19QNativeGestureEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QPoint QNativeGestureEvent::globalPos();
impl /*struct*/ QNativeGestureEvent {
  pub fn globalPos<RetType, T: QNativeGestureEvent_globalPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalPos(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_globalPos<RetType> {
  fn globalPos(self , rsthis: & QNativeGestureEvent) -> RetType;
}

  // proto:  const QPoint QNativeGestureEvent::globalPos();
impl<'a> /*trait*/ QNativeGestureEvent_globalPos<QPoint> for () {
  fn globalPos(self , rsthis: & QNativeGestureEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent9globalPosEv()};
    let mut ret = unsafe {C_ZNK19QNativeGestureEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QNativeGestureEvent::value();
impl /*struct*/ QNativeGestureEvent {
  pub fn value<RetType, T: QNativeGestureEvent_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_value<RetType> {
  fn value(self , rsthis: & QNativeGestureEvent) -> RetType;
}

  // proto:  qreal QNativeGestureEvent::value();
impl<'a> /*trait*/ QNativeGestureEvent_value<f64> for () {
  fn value(self , rsthis: & QNativeGestureEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent5valueEv()};
    let mut ret = unsafe {C_ZNK19QNativeGestureEvent5valueEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  const QPointF & QNativeGestureEvent::windowPos();
impl /*struct*/ QNativeGestureEvent {
  pub fn windowPos<RetType, T: QNativeGestureEvent_windowPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.windowPos(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_windowPos<RetType> {
  fn windowPos(self , rsthis: & QNativeGestureEvent) -> RetType;
}

  // proto:  const QPointF & QNativeGestureEvent::windowPos();
impl<'a> /*trait*/ QNativeGestureEvent_windowPos<QPointF> for () {
  fn windowPos(self , rsthis: & QNativeGestureEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent9windowPosEv()};
    let mut ret = unsafe {C_ZNK19QNativeGestureEvent9windowPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QResizeEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QResizeEvent {
    return QResizeEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QResizeEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QResizeEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  const QSize & QResizeEvent::oldSize();
impl /*struct*/ QResizeEvent {
  pub fn oldSize<RetType, T: QResizeEvent_oldSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.oldSize(self);
    // return 1;
  }
}

pub trait QResizeEvent_oldSize<RetType> {
  fn oldSize(self , rsthis: & QResizeEvent) -> RetType;
}

  // proto:  const QSize & QResizeEvent::oldSize();
impl<'a> /*trait*/ QResizeEvent_oldSize<QSize> for () {
  fn oldSize(self , rsthis: & QResizeEvent) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK12QResizeEvent7oldSizeEv()};
    let mut ret = unsafe {C_ZNK12QResizeEvent7oldSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QSize & QResizeEvent::size();
impl /*struct*/ QResizeEvent {
  pub fn size<RetType, T: QResizeEvent_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QResizeEvent_size<RetType> {
  fn size(self , rsthis: & QResizeEvent) -> RetType;
}

  // proto:  const QSize & QResizeEvent::size();
impl<'a> /*trait*/ QResizeEvent_size<QSize> for () {
  fn size(self , rsthis: & QResizeEvent) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK12QResizeEvent4sizeEv()};
    let mut ret = unsafe {C_ZNK12QResizeEvent4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QResizeEvent::~QResizeEvent();
impl /*struct*/ QResizeEvent {
  pub fn free<RetType, T: QResizeEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QResizeEvent_free<RetType> {
  fn free(self , rsthis: & QResizeEvent) -> RetType;
}

  // proto:  void QResizeEvent::~QResizeEvent();
impl<'a> /*trait*/ QResizeEvent_free<()> for () {
  fn free(self , rsthis: & QResizeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN12QResizeEventD2Ev()};
     unsafe {C_ZN12QResizeEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QResizeEvent::QResizeEvent(const QSize & size, const QSize & oldSize);
impl /*struct*/ QResizeEvent {
  pub fn new<T: QResizeEvent_new>(value: T) -> QResizeEvent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QResizeEvent_new {
  fn new(self) -> QResizeEvent;
}

  // proto:  void QResizeEvent::QResizeEvent(const QSize & size, const QSize & oldSize);
impl<'a> /*trait*/ QResizeEvent_new for (&'a QSize, &'a QSize) {
  fn new(self) -> QResizeEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN12QResizeEventC2ERK5QSizeS2_()};
    let ctysz: c_int = unsafe{QResizeEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN12QResizeEventC2ERK5QSizeS2_(arg0, arg1)};
    let rsthis = QResizeEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStatusTipEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStatusTipEvent {
    return QStatusTipEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QStatusTipEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QStatusTipEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QStatusTipEvent::~QStatusTipEvent();
impl /*struct*/ QStatusTipEvent {
  pub fn free<RetType, T: QStatusTipEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QStatusTipEvent_free<RetType> {
  fn free(self , rsthis: & QStatusTipEvent) -> RetType;
}

  // proto:  void QStatusTipEvent::~QStatusTipEvent();
impl<'a> /*trait*/ QStatusTipEvent_free<()> for () {
  fn free(self , rsthis: & QStatusTipEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QStatusTipEventD2Ev()};
     unsafe {C_ZN15QStatusTipEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QStatusTipEvent::tip();
impl /*struct*/ QStatusTipEvent {
  pub fn tip<RetType, T: QStatusTipEvent_tip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tip(self);
    // return 1;
  }
}

pub trait QStatusTipEvent_tip<RetType> {
  fn tip(self , rsthis: & QStatusTipEvent) -> RetType;
}

  // proto:  QString QStatusTipEvent::tip();
impl<'a> /*trait*/ QStatusTipEvent_tip<QString> for () {
  fn tip(self , rsthis: & QStatusTipEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QStatusTipEvent3tipEv()};
    let mut ret = unsafe {C_ZNK15QStatusTipEvent3tipEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStatusTipEvent::QStatusTipEvent(const QString & tip);
impl /*struct*/ QStatusTipEvent {
  pub fn new<T: QStatusTipEvent_new>(value: T) -> QStatusTipEvent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStatusTipEvent_new {
  fn new(self) -> QStatusTipEvent;
}

  // proto:  void QStatusTipEvent::QStatusTipEvent(const QString & tip);
impl<'a> /*trait*/ QStatusTipEvent_new for (&'a QString) {
  fn new(self) -> QStatusTipEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QStatusTipEventC2ERK7QString()};
    let ctysz: c_int = unsafe{QStatusTipEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN15QStatusTipEventC2ERK7QString(arg0)};
    let rsthis = QStatusTipEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QEnterEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QEnterEvent {
    return QEnterEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QEnterEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QEnterEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  int QEnterEvent::y();
impl /*struct*/ QEnterEvent {
  pub fn y<RetType, T: QEnterEvent_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QEnterEvent_y<RetType> {
  fn y(self , rsthis: & QEnterEvent) -> RetType;
}

  // proto:  int QEnterEvent::y();
impl<'a> /*trait*/ QEnterEvent_y<i32> for () {
  fn y(self , rsthis: & QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent1yEv()};
    let mut ret = unsafe {C_ZNK11QEnterEvent1yEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QPoint QEnterEvent::pos();
impl /*struct*/ QEnterEvent {
  pub fn pos<RetType, T: QEnterEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QEnterEvent_pos<RetType> {
  fn pos(self , rsthis: & QEnterEvent) -> RetType;
}

  // proto:  QPoint QEnterEvent::pos();
impl<'a> /*trait*/ QEnterEvent_pos<QPoint> for () {
  fn pos(self , rsthis: & QEnterEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent3posEv()};
    let mut ret = unsafe {C_ZNK11QEnterEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QEnterEvent::~QEnterEvent();
impl /*struct*/ QEnterEvent {
  pub fn free<RetType, T: QEnterEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QEnterEvent_free<RetType> {
  fn free(self , rsthis: & QEnterEvent) -> RetType;
}

  // proto:  void QEnterEvent::~QEnterEvent();
impl<'a> /*trait*/ QEnterEvent_free<()> for () {
  fn free(self , rsthis: & QEnterEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZN11QEnterEventD2Ev()};
     unsafe {C_ZN11QEnterEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QPointF & QEnterEvent::screenPos();
impl /*struct*/ QEnterEvent {
  pub fn screenPos<RetType, T: QEnterEvent_screenPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screenPos(self);
    // return 1;
  }
}

pub trait QEnterEvent_screenPos<RetType> {
  fn screenPos(self , rsthis: & QEnterEvent) -> RetType;
}

  // proto:  const QPointF & QEnterEvent::screenPos();
impl<'a> /*trait*/ QEnterEvent_screenPos<QPointF> for () {
  fn screenPos(self , rsthis: & QEnterEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent9screenPosEv()};
    let mut ret = unsafe {C_ZNK11QEnterEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QPointF & QEnterEvent::localPos();
impl /*struct*/ QEnterEvent {
  pub fn localPos<RetType, T: QEnterEvent_localPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.localPos(self);
    // return 1;
  }
}

pub trait QEnterEvent_localPos<RetType> {
  fn localPos(self , rsthis: & QEnterEvent) -> RetType;
}

  // proto:  const QPointF & QEnterEvent::localPos();
impl<'a> /*trait*/ QEnterEvent_localPos<QPointF> for () {
  fn localPos(self , rsthis: & QEnterEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent8localPosEv()};
    let mut ret = unsafe {C_ZNK11QEnterEvent8localPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QPointF & QEnterEvent::windowPos();
impl /*struct*/ QEnterEvent {
  pub fn windowPos<RetType, T: QEnterEvent_windowPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.windowPos(self);
    // return 1;
  }
}

pub trait QEnterEvent_windowPos<RetType> {
  fn windowPos(self , rsthis: & QEnterEvent) -> RetType;
}

  // proto:  const QPointF & QEnterEvent::windowPos();
impl<'a> /*trait*/ QEnterEvent_windowPos<QPointF> for () {
  fn windowPos(self , rsthis: & QEnterEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent9windowPosEv()};
    let mut ret = unsafe {C_ZNK11QEnterEvent9windowPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QEnterEvent::globalX();
impl /*struct*/ QEnterEvent {
  pub fn globalX<RetType, T: QEnterEvent_globalX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalX(self);
    // return 1;
  }
}

pub trait QEnterEvent_globalX<RetType> {
  fn globalX(self , rsthis: & QEnterEvent) -> RetType;
}

  // proto:  int QEnterEvent::globalX();
impl<'a> /*trait*/ QEnterEvent_globalX<i32> for () {
  fn globalX(self , rsthis: & QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent7globalXEv()};
    let mut ret = unsafe {C_ZNK11QEnterEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QEnterEvent::x();
impl /*struct*/ QEnterEvent {
  pub fn x<RetType, T: QEnterEvent_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QEnterEvent_x<RetType> {
  fn x(self , rsthis: & QEnterEvent) -> RetType;
}

  // proto:  int QEnterEvent::x();
impl<'a> /*trait*/ QEnterEvent_x<i32> for () {
  fn x(self , rsthis: & QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent1xEv()};
    let mut ret = unsafe {C_ZNK11QEnterEvent1xEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QPoint QEnterEvent::globalPos();
impl /*struct*/ QEnterEvent {
  pub fn globalPos<RetType, T: QEnterEvent_globalPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalPos(self);
    // return 1;
  }
}

pub trait QEnterEvent_globalPos<RetType> {
  fn globalPos(self , rsthis: & QEnterEvent) -> RetType;
}

  // proto:  QPoint QEnterEvent::globalPos();
impl<'a> /*trait*/ QEnterEvent_globalPos<QPoint> for () {
  fn globalPos(self , rsthis: & QEnterEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent9globalPosEv()};
    let mut ret = unsafe {C_ZNK11QEnterEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QEnterEvent::globalY();
impl /*struct*/ QEnterEvent {
  pub fn globalY<RetType, T: QEnterEvent_globalY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalY(self);
    // return 1;
  }
}

pub trait QEnterEvent_globalY<RetType> {
  fn globalY(self , rsthis: & QEnterEvent) -> RetType;
}

  // proto:  int QEnterEvent::globalY();
impl<'a> /*trait*/ QEnterEvent_globalY<i32> for () {
  fn globalY(self , rsthis: & QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent7globalYEv()};
    let mut ret = unsafe {C_ZNK11QEnterEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QEnterEvent::QEnterEvent(const QPointF & localPos, const QPointF & windowPos, const QPointF & screenPos);
impl /*struct*/ QEnterEvent {
  pub fn new<T: QEnterEvent_new>(value: T) -> QEnterEvent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QEnterEvent_new {
  fn new(self) -> QEnterEvent;
}

  // proto:  void QEnterEvent::QEnterEvent(const QPointF & localPos, const QPointF & windowPos, const QPointF & screenPos);
impl<'a> /*trait*/ QEnterEvent_new for (&'a QPointF, &'a QPointF, &'a QPointF) {
  fn new(self) -> QEnterEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZN11QEnterEventC2ERK7QPointFS2_S2_()};
    let ctysz: c_int = unsafe{QEnterEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QEnterEventC2ERK7QPointFS2_S2_(arg0, arg1, arg2)};
    let rsthis = QEnterEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMoveEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QMoveEvent {
    return QMoveEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QMoveEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QMoveEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QMoveEvent::~QMoveEvent();
impl /*struct*/ QMoveEvent {
  pub fn free<RetType, T: QMoveEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QMoveEvent_free<RetType> {
  fn free(self , rsthis: & QMoveEvent) -> RetType;
}

  // proto:  void QMoveEvent::~QMoveEvent();
impl<'a> /*trait*/ QMoveEvent_free<()> for () {
  fn free(self , rsthis: & QMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN10QMoveEventD2Ev()};
     unsafe {C_ZN10QMoveEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QPoint & QMoveEvent::oldPos();
impl /*struct*/ QMoveEvent {
  pub fn oldPos<RetType, T: QMoveEvent_oldPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.oldPos(self);
    // return 1;
  }
}

pub trait QMoveEvent_oldPos<RetType> {
  fn oldPos(self , rsthis: & QMoveEvent) -> RetType;
}

  // proto:  const QPoint & QMoveEvent::oldPos();
impl<'a> /*trait*/ QMoveEvent_oldPos<QPoint> for () {
  fn oldPos(self , rsthis: & QMoveEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QMoveEvent6oldPosEv()};
    let mut ret = unsafe {C_ZNK10QMoveEvent6oldPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMoveEvent::QMoveEvent(const QPoint & pos, const QPoint & oldPos);
impl /*struct*/ QMoveEvent {
  pub fn new<T: QMoveEvent_new>(value: T) -> QMoveEvent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QMoveEvent_new {
  fn new(self) -> QMoveEvent;
}

  // proto:  void QMoveEvent::QMoveEvent(const QPoint & pos, const QPoint & oldPos);
impl<'a> /*trait*/ QMoveEvent_new for (&'a QPoint, &'a QPoint) {
  fn new(self) -> QMoveEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN10QMoveEventC2ERK6QPointS2_()};
    let ctysz: c_int = unsafe{QMoveEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN10QMoveEventC2ERK6QPointS2_(arg0, arg1)};
    let rsthis = QMoveEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QPoint & QMoveEvent::pos();
impl /*struct*/ QMoveEvent {
  pub fn pos<RetType, T: QMoveEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QMoveEvent_pos<RetType> {
  fn pos(self , rsthis: & QMoveEvent) -> RetType;
}

  // proto:  const QPoint & QMoveEvent::pos();
impl<'a> /*trait*/ QMoveEvent_pos<QPoint> for () {
  fn pos(self , rsthis: & QMoveEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QMoveEvent3posEv()};
    let mut ret = unsafe {C_ZNK10QMoveEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QHideEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QHideEvent {
    return QHideEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QHideEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QHideEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QHideEvent::QHideEvent();
impl /*struct*/ QHideEvent {
  pub fn new<T: QHideEvent_new>(value: T) -> QHideEvent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QHideEvent_new {
  fn new(self) -> QHideEvent;
}

  // proto:  void QHideEvent::QHideEvent();
impl<'a> /*trait*/ QHideEvent_new for () {
  fn new(self) -> QHideEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QHideEventC2Ev()};
    let ctysz: c_int = unsafe{QHideEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN10QHideEventC2Ev()};
    let rsthis = QHideEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QHideEvent::~QHideEvent();
impl /*struct*/ QHideEvent {
  pub fn free<RetType, T: QHideEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QHideEvent_free<RetType> {
  fn free(self , rsthis: & QHideEvent) -> RetType;
}

  // proto:  void QHideEvent::~QHideEvent();
impl<'a> /*trait*/ QHideEvent_free<()> for () {
  fn free(self , rsthis: & QHideEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QHideEventD2Ev()};
     unsafe {C_ZN10QHideEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDragLeaveEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDragLeaveEvent {
    return QDragLeaveEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QDragLeaveEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QDragLeaveEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QDragLeaveEvent::~QDragLeaveEvent();
impl /*struct*/ QDragLeaveEvent {
  pub fn free<RetType, T: QDragLeaveEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QDragLeaveEvent_free<RetType> {
  fn free(self , rsthis: & QDragLeaveEvent) -> RetType;
}

  // proto:  void QDragLeaveEvent::~QDragLeaveEvent();
impl<'a> /*trait*/ QDragLeaveEvent_free<()> for () {
  fn free(self , rsthis: & QDragLeaveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QDragLeaveEventD2Ev()};
     unsafe {C_ZN15QDragLeaveEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDragLeaveEvent::QDragLeaveEvent();
impl /*struct*/ QDragLeaveEvent {
  pub fn new<T: QDragLeaveEvent_new>(value: T) -> QDragLeaveEvent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QDragLeaveEvent_new {
  fn new(self) -> QDragLeaveEvent;
}

  // proto:  void QDragLeaveEvent::QDragLeaveEvent();
impl<'a> /*trait*/ QDragLeaveEvent_new for () {
  fn new(self) -> QDragLeaveEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QDragLeaveEventC2Ev()};
    let ctysz: c_int = unsafe{QDragLeaveEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN15QDragLeaveEventC2Ev()};
    let rsthis = QDragLeaveEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDropEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDropEvent {
    return QDropEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QDropEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QDropEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QDropEvent::~QDropEvent();
impl /*struct*/ QDropEvent {
  pub fn free<RetType, T: QDropEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QDropEvent_free<RetType> {
  fn free(self , rsthis: & QDropEvent) -> RetType;
}

  // proto:  void QDropEvent::~QDropEvent();
impl<'a> /*trait*/ QDropEvent_free<()> for () {
  fn free(self , rsthis: & QDropEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QDropEventD2Ev()};
     unsafe {C_ZN10QDropEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPoint QDropEvent::pos();
impl /*struct*/ QDropEvent {
  pub fn pos<RetType, T: QDropEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QDropEvent_pos<RetType> {
  fn pos(self , rsthis: & QDropEvent) -> RetType;
}

  // proto:  QPoint QDropEvent::pos();
impl<'a> /*trait*/ QDropEvent_pos<QPoint> for () {
  fn pos(self , rsthis: & QDropEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDropEvent3posEv()};
    let mut ret = unsafe {C_ZNK10QDropEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QObject * QDropEvent::source();
impl /*struct*/ QDropEvent {
  pub fn source<RetType, T: QDropEvent_source<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.source(self);
    // return 1;
  }
}

pub trait QDropEvent_source<RetType> {
  fn source(self , rsthis: & QDropEvent) -> RetType;
}

  // proto:  QObject * QDropEvent::source();
impl<'a> /*trait*/ QDropEvent_source<QObject> for () {
  fn source(self , rsthis: & QDropEvent) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDropEvent6sourceEv()};
    let mut ret = unsafe {C_ZNK10QDropEvent6sourceEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMimeData * QDropEvent::mimeData();
impl /*struct*/ QDropEvent {
  pub fn mimeData<RetType, T: QDropEvent_mimeData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mimeData(self);
    // return 1;
  }
}

pub trait QDropEvent_mimeData<RetType> {
  fn mimeData(self , rsthis: & QDropEvent) -> RetType;
}

  // proto:  const QMimeData * QDropEvent::mimeData();
impl<'a> /*trait*/ QDropEvent_mimeData<QMimeData> for () {
  fn mimeData(self , rsthis: & QDropEvent) -> QMimeData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDropEvent8mimeDataEv()};
    let mut ret = unsafe {C_ZNK10QDropEvent8mimeDataEv(rsthis.qclsinst)};
    let mut ret1 = QMimeData::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDropEvent::acceptProposedAction();
impl /*struct*/ QDropEvent {
  pub fn acceptProposedAction<RetType, T: QDropEvent_acceptProposedAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.acceptProposedAction(self);
    // return 1;
  }
}

pub trait QDropEvent_acceptProposedAction<RetType> {
  fn acceptProposedAction(self , rsthis: & QDropEvent) -> RetType;
}

  // proto:  void QDropEvent::acceptProposedAction();
impl<'a> /*trait*/ QDropEvent_acceptProposedAction<()> for () {
  fn acceptProposedAction(self , rsthis: & QDropEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QDropEvent20acceptProposedActionEv()};
     unsafe {C_ZN10QDropEvent20acceptProposedActionEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QPointF & QDropEvent::posF();
impl /*struct*/ QDropEvent {
  pub fn posF<RetType, T: QDropEvent_posF<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.posF(self);
    // return 1;
  }
}

pub trait QDropEvent_posF<RetType> {
  fn posF(self , rsthis: & QDropEvent) -> RetType;
}

  // proto:  const QPointF & QDropEvent::posF();
impl<'a> /*trait*/ QDropEvent_posF<QPointF> for () {
  fn posF(self , rsthis: & QDropEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDropEvent4posFEv()};
    let mut ret = unsafe {C_ZNK10QDropEvent4posFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QInputEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QInputEvent {
    return QInputEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QInputEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QInputEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QInputEvent::setTimestamp(ulong atimestamp);
impl /*struct*/ QInputEvent {
  pub fn setTimestamp<RetType, T: QInputEvent_setTimestamp<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTimestamp(self);
    // return 1;
  }
}

pub trait QInputEvent_setTimestamp<RetType> {
  fn setTimestamp(self , rsthis: & QInputEvent) -> RetType;
}

  // proto:  void QInputEvent::setTimestamp(ulong atimestamp);
impl<'a> /*trait*/ QInputEvent_setTimestamp<()> for (u64) {
  fn setTimestamp(self , rsthis: & QInputEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QInputEvent12setTimestampEm()};
    let arg0 = self  as c_ulong;
     unsafe {C_ZN11QInputEvent12setTimestampEm(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  ulong QInputEvent::timestamp();
impl /*struct*/ QInputEvent {
  pub fn timestamp<RetType, T: QInputEvent_timestamp<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.timestamp(self);
    // return 1;
  }
}

pub trait QInputEvent_timestamp<RetType> {
  fn timestamp(self , rsthis: & QInputEvent) -> RetType;
}

  // proto:  ulong QInputEvent::timestamp();
impl<'a> /*trait*/ QInputEvent_timestamp<u64> for () {
  fn timestamp(self , rsthis: & QInputEvent) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QInputEvent9timestampEv()};
    let mut ret = unsafe {C_ZNK11QInputEvent9timestampEv(rsthis.qclsinst)};
    return ret as u64; // 1
    // return 1;
  }
}

  // proto:  void QInputEvent::~QInputEvent();
impl /*struct*/ QInputEvent {
  pub fn free<RetType, T: QInputEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QInputEvent_free<RetType> {
  fn free(self , rsthis: & QInputEvent) -> RetType;
}

  // proto:  void QInputEvent::~QInputEvent();
impl<'a> /*trait*/ QInputEvent_free<()> for () {
  fn free(self , rsthis: & QInputEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QInputEventD2Ev()};
     unsafe {C_ZN11QInputEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QApplicationStateChangeEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QApplicationStateChangeEvent {
    return QApplicationStateChangeEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QApplicationStateChangeEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QApplicationStateChangeEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
impl /*struct*/ QKeyEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QKeyEvent {
    return QKeyEvent{qbase: QInputEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QKeyEvent {
  type Target = QInputEvent;

  fn deref(&self) -> &QInputEvent {
    return & self.qbase;
  }
}
impl AsRef<QInputEvent> for QKeyEvent {
  fn as_ref(& self) -> & QInputEvent {
    return & self.qbase;
  }
}
  // proto:  int QKeyEvent::count();
impl /*struct*/ QKeyEvent {
  pub fn count<RetType, T: QKeyEvent_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QKeyEvent_count<RetType> {
  fn count(self , rsthis: & QKeyEvent) -> RetType;
}

  // proto:  int QKeyEvent::count();
impl<'a> /*trait*/ QKeyEvent_count<i32> for () {
  fn count(self , rsthis: & QKeyEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent5countEv()};
    let mut ret = unsafe {C_ZNK9QKeyEvent5countEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QKeyEvent::~QKeyEvent();
impl /*struct*/ QKeyEvent {
  pub fn free<RetType, T: QKeyEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QKeyEvent_free<RetType> {
  fn free(self , rsthis: & QKeyEvent) -> RetType;
}

  // proto:  void QKeyEvent::~QKeyEvent();
impl<'a> /*trait*/ QKeyEvent_free<()> for () {
  fn free(self , rsthis: & QKeyEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QKeyEventD2Ev()};
     unsafe {C_ZN9QKeyEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QKeyEvent::text();
impl /*struct*/ QKeyEvent {
  pub fn text<RetType, T: QKeyEvent_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QKeyEvent_text<RetType> {
  fn text(self , rsthis: & QKeyEvent) -> RetType;
}

  // proto:  QString QKeyEvent::text();
impl<'a> /*trait*/ QKeyEvent_text<QString> for () {
  fn text(self , rsthis: & QKeyEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent4textEv()};
    let mut ret = unsafe {C_ZNK9QKeyEvent4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  quint32 QKeyEvent::nativeVirtualKey();
impl /*struct*/ QKeyEvent {
  pub fn nativeVirtualKey<RetType, T: QKeyEvent_nativeVirtualKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nativeVirtualKey(self);
    // return 1;
  }
}

pub trait QKeyEvent_nativeVirtualKey<RetType> {
  fn nativeVirtualKey(self , rsthis: & QKeyEvent) -> RetType;
}

  // proto:  quint32 QKeyEvent::nativeVirtualKey();
impl<'a> /*trait*/ QKeyEvent_nativeVirtualKey<u32> for () {
  fn nativeVirtualKey(self , rsthis: & QKeyEvent) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent16nativeVirtualKeyEv()};
    let mut ret = unsafe {C_ZNK9QKeyEvent16nativeVirtualKeyEv(rsthis.qclsinst)};
    return ret as u32; // 1
    // return 1;
  }
}

  // proto:  bool QKeyEvent::isAutoRepeat();
impl /*struct*/ QKeyEvent {
  pub fn isAutoRepeat<RetType, T: QKeyEvent_isAutoRepeat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isAutoRepeat(self);
    // return 1;
  }
}

pub trait QKeyEvent_isAutoRepeat<RetType> {
  fn isAutoRepeat(self , rsthis: & QKeyEvent) -> RetType;
}

  // proto:  bool QKeyEvent::isAutoRepeat();
impl<'a> /*trait*/ QKeyEvent_isAutoRepeat<i8> for () {
  fn isAutoRepeat(self , rsthis: & QKeyEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent12isAutoRepeatEv()};
    let mut ret = unsafe {C_ZNK9QKeyEvent12isAutoRepeatEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QKeyEvent::key();
impl /*struct*/ QKeyEvent {
  pub fn key<RetType, T: QKeyEvent_key<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.key(self);
    // return 1;
  }
}

pub trait QKeyEvent_key<RetType> {
  fn key(self , rsthis: & QKeyEvent) -> RetType;
}

  // proto:  int QKeyEvent::key();
impl<'a> /*trait*/ QKeyEvent_key<i32> for () {
  fn key(self , rsthis: & QKeyEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent3keyEv()};
    let mut ret = unsafe {C_ZNK9QKeyEvent3keyEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  quint32 QKeyEvent::nativeModifiers();
impl /*struct*/ QKeyEvent {
  pub fn nativeModifiers<RetType, T: QKeyEvent_nativeModifiers<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nativeModifiers(self);
    // return 1;
  }
}

pub trait QKeyEvent_nativeModifiers<RetType> {
  fn nativeModifiers(self , rsthis: & QKeyEvent) -> RetType;
}

  // proto:  quint32 QKeyEvent::nativeModifiers();
impl<'a> /*trait*/ QKeyEvent_nativeModifiers<u32> for () {
  fn nativeModifiers(self , rsthis: & QKeyEvent) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent15nativeModifiersEv()};
    let mut ret = unsafe {C_ZNK9QKeyEvent15nativeModifiersEv(rsthis.qclsinst)};
    return ret as u32; // 1
    // return 1;
  }
}

  // proto:  quint32 QKeyEvent::nativeScanCode();
impl /*struct*/ QKeyEvent {
  pub fn nativeScanCode<RetType, T: QKeyEvent_nativeScanCode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nativeScanCode(self);
    // return 1;
  }
}

pub trait QKeyEvent_nativeScanCode<RetType> {
  fn nativeScanCode(self , rsthis: & QKeyEvent) -> RetType;
}

  // proto:  quint32 QKeyEvent::nativeScanCode();
impl<'a> /*trait*/ QKeyEvent_nativeScanCode<u32> for () {
  fn nativeScanCode(self , rsthis: & QKeyEvent) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent14nativeScanCodeEv()};
    let mut ret = unsafe {C_ZNK9QKeyEvent14nativeScanCodeEv(rsthis.qclsinst)};
    return ret as u32; // 1
    // return 1;
  }
}

impl /*struct*/ QContextMenuEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QContextMenuEvent {
    return QContextMenuEvent{qbase: QInputEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QContextMenuEvent {
  type Target = QInputEvent;

  fn deref(&self) -> &QInputEvent {
    return & self.qbase;
  }
}
impl AsRef<QInputEvent> for QContextMenuEvent {
  fn as_ref(& self) -> & QInputEvent {
    return & self.qbase;
  }
}
  // proto:  const QPoint & QContextMenuEvent::globalPos();
impl /*struct*/ QContextMenuEvent {
  pub fn globalPos<RetType, T: QContextMenuEvent_globalPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalPos(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_globalPos<RetType> {
  fn globalPos(self , rsthis: & QContextMenuEvent) -> RetType;
}

  // proto:  const QPoint & QContextMenuEvent::globalPos();
impl<'a> /*trait*/ QContextMenuEvent_globalPos<QPoint> for () {
  fn globalPos(self , rsthis: & QContextMenuEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent9globalPosEv()};
    let mut ret = unsafe {C_ZNK17QContextMenuEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QContextMenuEvent::globalY();
impl /*struct*/ QContextMenuEvent {
  pub fn globalY<RetType, T: QContextMenuEvent_globalY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalY(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_globalY<RetType> {
  fn globalY(self , rsthis: & QContextMenuEvent) -> RetType;
}

  // proto:  int QContextMenuEvent::globalY();
impl<'a> /*trait*/ QContextMenuEvent_globalY<i32> for () {
  fn globalY(self , rsthis: & QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent7globalYEv()};
    let mut ret = unsafe {C_ZNK17QContextMenuEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QContextMenuEvent::globalX();
impl /*struct*/ QContextMenuEvent {
  pub fn globalX<RetType, T: QContextMenuEvent_globalX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalX(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_globalX<RetType> {
  fn globalX(self , rsthis: & QContextMenuEvent) -> RetType;
}

  // proto:  int QContextMenuEvent::globalX();
impl<'a> /*trait*/ QContextMenuEvent_globalX<i32> for () {
  fn globalX(self , rsthis: & QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent7globalXEv()};
    let mut ret = unsafe {C_ZNK17QContextMenuEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  const QPoint & QContextMenuEvent::pos();
impl /*struct*/ QContextMenuEvent {
  pub fn pos<RetType, T: QContextMenuEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_pos<RetType> {
  fn pos(self , rsthis: & QContextMenuEvent) -> RetType;
}

  // proto:  const QPoint & QContextMenuEvent::pos();
impl<'a> /*trait*/ QContextMenuEvent_pos<QPoint> for () {
  fn pos(self , rsthis: & QContextMenuEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent3posEv()};
    let mut ret = unsafe {C_ZNK17QContextMenuEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QContextMenuEvent::y();
impl /*struct*/ QContextMenuEvent {
  pub fn y<RetType, T: QContextMenuEvent_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_y<RetType> {
  fn y(self , rsthis: & QContextMenuEvent) -> RetType;
}

  // proto:  int QContextMenuEvent::y();
impl<'a> /*trait*/ QContextMenuEvent_y<i32> for () {
  fn y(self , rsthis: & QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent1yEv()};
    let mut ret = unsafe {C_ZNK17QContextMenuEvent1yEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QContextMenuEvent::x();
impl /*struct*/ QContextMenuEvent {
  pub fn x<RetType, T: QContextMenuEvent_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_x<RetType> {
  fn x(self , rsthis: & QContextMenuEvent) -> RetType;
}

  // proto:  int QContextMenuEvent::x();
impl<'a> /*trait*/ QContextMenuEvent_x<i32> for () {
  fn x(self , rsthis: & QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent1xEv()};
    let mut ret = unsafe {C_ZNK17QContextMenuEvent1xEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QContextMenuEvent::~QContextMenuEvent();
impl /*struct*/ QContextMenuEvent {
  pub fn free<RetType, T: QContextMenuEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_free<RetType> {
  fn free(self , rsthis: & QContextMenuEvent) -> RetType;
}

  // proto:  void QContextMenuEvent::~QContextMenuEvent();
impl<'a> /*trait*/ QContextMenuEvent_free<()> for () {
  fn free(self , rsthis: & QContextMenuEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QContextMenuEventD2Ev()};
     unsafe {C_ZN17QContextMenuEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QScrollPrepareEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QScrollPrepareEvent {
    return QScrollPrepareEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QScrollPrepareEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QScrollPrepareEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QScrollPrepareEvent::setContentPosRange(const QRectF & rect);
impl /*struct*/ QScrollPrepareEvent {
  pub fn setContentPosRange<RetType, T: QScrollPrepareEvent_setContentPosRange<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setContentPosRange(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_setContentPosRange<RetType> {
  fn setContentPosRange(self , rsthis: & QScrollPrepareEvent) -> RetType;
}

  // proto:  void QScrollPrepareEvent::setContentPosRange(const QRectF & rect);
impl<'a> /*trait*/ QScrollPrepareEvent_setContentPosRange<()> for (&'a QRectF) {
  fn setContentPosRange(self , rsthis: & QScrollPrepareEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZN19QScrollPrepareEvent18setContentPosRangeERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN19QScrollPrepareEvent18setContentPosRangeERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QScrollPrepareEvent::setContentPos(const QPointF & pos);
impl /*struct*/ QScrollPrepareEvent {
  pub fn setContentPos<RetType, T: QScrollPrepareEvent_setContentPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setContentPos(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_setContentPos<RetType> {
  fn setContentPos(self , rsthis: & QScrollPrepareEvent) -> RetType;
}

  // proto:  void QScrollPrepareEvent::setContentPos(const QPointF & pos);
impl<'a> /*trait*/ QScrollPrepareEvent_setContentPos<()> for (&'a QPointF) {
  fn setContentPos(self , rsthis: & QScrollPrepareEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZN19QScrollPrepareEvent13setContentPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN19QScrollPrepareEvent13setContentPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QScrollPrepareEvent::contentPosRange();
impl /*struct*/ QScrollPrepareEvent {
  pub fn contentPosRange<RetType, T: QScrollPrepareEvent_contentPosRange<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contentPosRange(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_contentPosRange<RetType> {
  fn contentPosRange(self , rsthis: & QScrollPrepareEvent) -> RetType;
}

  // proto:  QRectF QScrollPrepareEvent::contentPosRange();
impl<'a> /*trait*/ QScrollPrepareEvent_contentPosRange<QRectF> for () {
  fn contentPosRange(self , rsthis: & QScrollPrepareEvent) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZNK19QScrollPrepareEvent15contentPosRangeEv()};
    let mut ret = unsafe {C_ZNK19QScrollPrepareEvent15contentPosRangeEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QScrollPrepareEvent::contentPos();
impl /*struct*/ QScrollPrepareEvent {
  pub fn contentPos<RetType, T: QScrollPrepareEvent_contentPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contentPos(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_contentPos<RetType> {
  fn contentPos(self , rsthis: & QScrollPrepareEvent) -> RetType;
}

  // proto:  QPointF QScrollPrepareEvent::contentPos();
impl<'a> /*trait*/ QScrollPrepareEvent_contentPos<QPointF> for () {
  fn contentPos(self , rsthis: & QScrollPrepareEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZNK19QScrollPrepareEvent10contentPosEv()};
    let mut ret = unsafe {C_ZNK19QScrollPrepareEvent10contentPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QScrollPrepareEvent::setViewportSize(const QSizeF & size);
impl /*struct*/ QScrollPrepareEvent {
  pub fn setViewportSize<RetType, T: QScrollPrepareEvent_setViewportSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setViewportSize(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_setViewportSize<RetType> {
  fn setViewportSize(self , rsthis: & QScrollPrepareEvent) -> RetType;
}

  // proto:  void QScrollPrepareEvent::setViewportSize(const QSizeF & size);
impl<'a> /*trait*/ QScrollPrepareEvent_setViewportSize<()> for (&'a QSizeF) {
  fn setViewportSize(self , rsthis: & QScrollPrepareEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZN19QScrollPrepareEvent15setViewportSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN19QScrollPrepareEvent15setViewportSizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QScrollPrepareEvent::QScrollPrepareEvent(const QPointF & startPos);
impl /*struct*/ QScrollPrepareEvent {
  pub fn new<T: QScrollPrepareEvent_new>(value: T) -> QScrollPrepareEvent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QScrollPrepareEvent_new {
  fn new(self) -> QScrollPrepareEvent;
}

  // proto:  void QScrollPrepareEvent::QScrollPrepareEvent(const QPointF & startPos);
impl<'a> /*trait*/ QScrollPrepareEvent_new for (&'a QPointF) {
  fn new(self) -> QScrollPrepareEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZN19QScrollPrepareEventC2ERK7QPointF()};
    let ctysz: c_int = unsafe{QScrollPrepareEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN19QScrollPrepareEventC2ERK7QPointF(arg0)};
    let rsthis = QScrollPrepareEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPointF QScrollPrepareEvent::startPos();
impl /*struct*/ QScrollPrepareEvent {
  pub fn startPos<RetType, T: QScrollPrepareEvent_startPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.startPos(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_startPos<RetType> {
  fn startPos(self , rsthis: & QScrollPrepareEvent) -> RetType;
}

  // proto:  QPointF QScrollPrepareEvent::startPos();
impl<'a> /*trait*/ QScrollPrepareEvent_startPos<QPointF> for () {
  fn startPos(self , rsthis: & QScrollPrepareEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZNK19QScrollPrepareEvent8startPosEv()};
    let mut ret = unsafe {C_ZNK19QScrollPrepareEvent8startPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSizeF QScrollPrepareEvent::viewportSize();
impl /*struct*/ QScrollPrepareEvent {
  pub fn viewportSize<RetType, T: QScrollPrepareEvent_viewportSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.viewportSize(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_viewportSize<RetType> {
  fn viewportSize(self , rsthis: & QScrollPrepareEvent) -> RetType;
}

  // proto:  QSizeF QScrollPrepareEvent::viewportSize();
impl<'a> /*trait*/ QScrollPrepareEvent_viewportSize<QSizeF> for () {
  fn viewportSize(self , rsthis: & QScrollPrepareEvent) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZNK19QScrollPrepareEvent12viewportSizeEv()};
    let mut ret = unsafe {C_ZNK19QScrollPrepareEvent12viewportSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QScrollPrepareEvent::~QScrollPrepareEvent();
impl /*struct*/ QScrollPrepareEvent {
  pub fn free<RetType, T: QScrollPrepareEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_free<RetType> {
  fn free(self , rsthis: & QScrollPrepareEvent) -> RetType;
}

  // proto:  void QScrollPrepareEvent::~QScrollPrepareEvent();
impl<'a> /*trait*/ QScrollPrepareEvent_free<()> for () {
  fn free(self , rsthis: & QScrollPrepareEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZN19QScrollPrepareEventD2Ev()};
     unsafe {C_ZN19QScrollPrepareEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QShortcutEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QShortcutEvent {
    return QShortcutEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QShortcutEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QShortcutEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  const QKeySequence & QShortcutEvent::key();
impl /*struct*/ QShortcutEvent {
  pub fn key<RetType, T: QShortcutEvent_key<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.key(self);
    // return 1;
  }
}

pub trait QShortcutEvent_key<RetType> {
  fn key(self , rsthis: & QShortcutEvent) -> RetType;
}

  // proto:  const QKeySequence & QShortcutEvent::key();
impl<'a> /*trait*/ QShortcutEvent_key<QKeySequence> for () {
  fn key(self , rsthis: & QShortcutEvent) -> QKeySequence {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK14QShortcutEvent3keyEv()};
    let mut ret = unsafe {C_ZNK14QShortcutEvent3keyEv(rsthis.qclsinst)};
    let mut ret1 = QKeySequence::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QShortcutEvent::~QShortcutEvent();
impl /*struct*/ QShortcutEvent {
  pub fn free<RetType, T: QShortcutEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QShortcutEvent_free<RetType> {
  fn free(self , rsthis: & QShortcutEvent) -> RetType;
}

  // proto:  void QShortcutEvent::~QShortcutEvent();
impl<'a> /*trait*/ QShortcutEvent_free<()> for () {
  fn free(self , rsthis: & QShortcutEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN14QShortcutEventD2Ev()};
     unsafe {C_ZN14QShortcutEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QShortcutEvent::isAmbiguous();
impl /*struct*/ QShortcutEvent {
  pub fn isAmbiguous<RetType, T: QShortcutEvent_isAmbiguous<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isAmbiguous(self);
    // return 1;
  }
}

pub trait QShortcutEvent_isAmbiguous<RetType> {
  fn isAmbiguous(self , rsthis: & QShortcutEvent) -> RetType;
}

  // proto:  bool QShortcutEvent::isAmbiguous();
impl<'a> /*trait*/ QShortcutEvent_isAmbiguous<i8> for () {
  fn isAmbiguous(self , rsthis: & QShortcutEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK14QShortcutEvent11isAmbiguousEv()};
    let mut ret = unsafe {C_ZNK14QShortcutEvent11isAmbiguousEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QShortcutEvent::QShortcutEvent(const QKeySequence & key, int id, bool ambiguous);
impl /*struct*/ QShortcutEvent {
  pub fn new<T: QShortcutEvent_new>(value: T) -> QShortcutEvent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QShortcutEvent_new {
  fn new(self) -> QShortcutEvent;
}

  // proto:  void QShortcutEvent::QShortcutEvent(const QKeySequence & key, int id, bool ambiguous);
impl<'a> /*trait*/ QShortcutEvent_new for (&'a QKeySequence, i32, Option<i8>) {
  fn new(self) -> QShortcutEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN14QShortcutEventC2ERK12QKeySequenceib()};
    let ctysz: c_int = unsafe{QShortcutEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = (if self.2.is_none() {false as i8} else {self.2.unwrap()})  as c_char;
    let qthis: u64 = unsafe {C_ZN14QShortcutEventC2ERK12QKeySequenceib(arg0, arg1, arg2)};
    let rsthis = QShortcutEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QShortcutEvent::shortcutId();
impl /*struct*/ QShortcutEvent {
  pub fn shortcutId<RetType, T: QShortcutEvent_shortcutId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shortcutId(self);
    // return 1;
  }
}

pub trait QShortcutEvent_shortcutId<RetType> {
  fn shortcutId(self , rsthis: & QShortcutEvent) -> RetType;
}

  // proto:  int QShortcutEvent::shortcutId();
impl<'a> /*trait*/ QShortcutEvent_shortcutId<i32> for () {
  fn shortcutId(self , rsthis: & QShortcutEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK14QShortcutEvent10shortcutIdEv()};
    let mut ret = unsafe {C_ZNK14QShortcutEvent10shortcutIdEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

impl /*struct*/ QWindowStateChangeEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QWindowStateChangeEvent {
    return QWindowStateChangeEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QWindowStateChangeEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QWindowStateChangeEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  bool QWindowStateChangeEvent::isOverride();
impl /*struct*/ QWindowStateChangeEvent {
  pub fn isOverride<RetType, T: QWindowStateChangeEvent_isOverride<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isOverride(self);
    // return 1;
  }
}

pub trait QWindowStateChangeEvent_isOverride<RetType> {
  fn isOverride(self , rsthis: & QWindowStateChangeEvent) -> RetType;
}

  // proto:  bool QWindowStateChangeEvent::isOverride();
impl<'a> /*trait*/ QWindowStateChangeEvent_isOverride<i8> for () {
  fn isOverride(self , rsthis: & QWindowStateChangeEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QWindowStateChangeEvent10isOverrideEv()};
    let mut ret = unsafe {C_ZNK23QWindowStateChangeEvent10isOverrideEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QWindowStateChangeEvent::~QWindowStateChangeEvent();
impl /*struct*/ QWindowStateChangeEvent {
  pub fn free<RetType, T: QWindowStateChangeEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QWindowStateChangeEvent_free<RetType> {
  fn free(self , rsthis: & QWindowStateChangeEvent) -> RetType;
}

  // proto:  void QWindowStateChangeEvent::~QWindowStateChangeEvent();
impl<'a> /*trait*/ QWindowStateChangeEvent_free<()> for () {
  fn free(self , rsthis: & QWindowStateChangeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QWindowStateChangeEventD2Ev()};
     unsafe {C_ZN23QWindowStateChangeEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QInputMethodQueryEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QInputMethodQueryEvent {
    return QInputMethodQueryEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QInputMethodQueryEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QInputMethodQueryEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QInputMethodQueryEvent::~QInputMethodQueryEvent();
impl /*struct*/ QInputMethodQueryEvent {
  pub fn free<RetType, T: QInputMethodQueryEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QInputMethodQueryEvent_free<RetType> {
  fn free(self , rsthis: & QInputMethodQueryEvent) -> RetType;
}

  // proto:  void QInputMethodQueryEvent::~QInputMethodQueryEvent();
impl<'a> /*trait*/ QInputMethodQueryEvent_free<()> for () {
  fn free(self , rsthis: & QInputMethodQueryEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QInputMethodQueryEventD2Ev()};
     unsafe {C_ZN22QInputMethodQueryEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

