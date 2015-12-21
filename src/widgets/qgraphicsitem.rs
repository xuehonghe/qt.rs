// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpainterpath::QPainterPath;
use super::qpointf::QPointF;
use super::qpolygonf::QPolygonF;
use super::qtransform::QTransform;
use super::qregion::QRegion;
use super::qpainter::QPainter;
use super::qstyleoptiongraphicsitem::QStyleOptionGraphicsItem;
use super::qwidget::QWidget;
use super::qrectf::QRectF;
use super::qgraphicseffect::QGraphicsEffect;
use super::qstring::QString;
use super::qmatrix::QMatrix;
use super::qsize::QSize;
use super::qvariant::QVariant;
use super::qgraphicsitemgroup::QGraphicsItemGroup;
use super::qcursor::QCursor;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QGraphicsItem::QGraphicsItem(const QGraphicsItem & );
  fn _ZN13QGraphicsItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPainterPath QGraphicsItem::mapFromParent(const QPainterPath & path);
  fn _ZNK13QGraphicsItem13mapFromParentERK12QPainterPath(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPointF QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QPointF & point);
  fn _ZNK13QGraphicsItem11mapFromItemEPKS_RK7QPointF(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QGraphicsItem * QGraphicsItem::focusItem();
  fn _ZNK13QGraphicsItem9focusItemEv(qthis: *mut c_void);
  // proto:  QGraphicsObject * QGraphicsItem::parentObject();
  fn _ZNK13QGraphicsItem12parentObjectEv(qthis: *mut c_void);
  // proto:  void QGraphicsItem::setTransformOriginPoint(const QPointF & origin);
  fn _ZN13QGraphicsItem23setTransformOriginPointERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsItem::ungrabMouse();
  fn _ZN13QGraphicsItem11ungrabMouseEv(qthis: *mut c_void);
  // proto:  int QGraphicsItem::type();
  fn _ZNK13QGraphicsItem4typeEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QGraphicsItem::isSelected();
  fn _ZNK13QGraphicsItem10isSelectedEv(qthis: *mut c_void) -> c_char;
  // proto:  QPolygonF QGraphicsItem::mapFromItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem11mapFromItemEPKS_dddd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double) -> *mut c_void;
  // proto:  QGraphicsWidget * QGraphicsItem::parentWidget();
  fn _ZNK13QGraphicsItem12parentWidgetEv(qthis: *mut c_void);
  // proto:  void QGraphicsItem::resetTransform();
  fn _ZN13QGraphicsItem14resetTransformEv(qthis: *mut c_void);
  // proto:  QRegion QGraphicsItem::boundingRegion(const QTransform & itemToDeviceTransform);
  fn _ZNK13QGraphicsItem14boundingRegionERK10QTransform(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN13QGraphicsItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  bool QGraphicsItem::isActive();
  fn _ZNK13QGraphicsItem8isActiveEv(qthis: *mut c_void) -> c_char;
  // proto:  void QGraphicsItem::QGraphicsItem(QGraphicsItem * parent);
  fn _ZN13QGraphicsItemC1EPS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPolygonF QGraphicsItem::mapToParent(const QPolygonF & polygon);
  fn _ZNK13QGraphicsItem11mapToParentERK9QPolygonF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsItem::isWidget();
  fn _ZNK13QGraphicsItem8isWidgetEv(qthis: *mut c_void) -> c_char;
  // proto:  void QGraphicsItem::setParentItem(QGraphicsItem * parent);
  fn _ZN13QGraphicsItem13setParentItemEPS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem * item, const QRectF & rect);
  fn _ZNK13QGraphicsItem9mapToItemEPKS_RK6QRectF(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QGraphicsWidget * QGraphicsItem::window();
  fn _ZNK13QGraphicsItem6windowEv(qthis: *mut c_void);
  // proto:  QPointF QGraphicsItem::scenePos();
  fn _ZNK13QGraphicsItem8scenePosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsItem::handlesChildEvents();
  fn _ZNK13QGraphicsItem18handlesChildEventsEv(qthis: *mut c_void) -> c_char;
  // proto:  void QGraphicsItem::setOpacity(qreal opacity);
  fn _ZN13QGraphicsItem10setOpacityEd(qthis: *mut c_void, arg0: c_double);
  // proto:  QTransform QGraphicsItem::sceneTransform();
  fn _ZNK13QGraphicsItem14sceneTransformEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::setZValue(qreal z);
  fn _ZN13QGraphicsItem9setZValueEd(qthis: *mut c_void, arg0: c_double);
  // proto:  QPolygonF QGraphicsItem::mapFromParent(const QRectF & rect);
  fn _ZNK13QGraphicsItem13mapFromParentERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPolygonF QGraphicsItem::mapFromParent(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem13mapFromParentEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  bool QGraphicsItem::isObscured(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem10isObscuredEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> c_char;
  // proto:  void QGraphicsItem::installSceneEventFilter(QGraphicsItem * filterItem);
  fn _ZN13QGraphicsItem23installSceneEventFilterEPS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsItem::setY(qreal y);
  fn _ZN13QGraphicsItem4setYEd(qthis: *mut c_void, arg0: c_double);
  // proto:  QRectF QGraphicsItem::mapRectToItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem13mapRectToItemEPKS_dddd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double) -> *mut c_void;
  // proto:  QGraphicsItem * QGraphicsItem::parentItem();
  fn _ZNK13QGraphicsItem10parentItemEv(qthis: *mut c_void);
  // proto:  void QGraphicsItem::clearFocus();
  fn _ZN13QGraphicsItem10clearFocusEv(qthis: *mut c_void);
  // proto:  bool QGraphicsItem::isWindow();
  fn _ZNK13QGraphicsItem8isWindowEv(qthis: *mut c_void) -> c_char;
  // proto:  QPointF QGraphicsItem::transformOriginPoint();
  fn _ZNK13QGraphicsItem20transformOriginPointEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::boundingRect();
  fn _ZNK13QGraphicsItem12boundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::childrenBoundingRect();
  fn _ZNK13QGraphicsItem20childrenBoundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsItem::isObscured(const QRectF & rect);
  fn _ZNK13QGraphicsItem10isObscuredERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QPolygonF QGraphicsItem::mapFromScene(const QRectF & rect);
  fn _ZNK13QGraphicsItem12mapFromSceneERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsItem::hasCursor();
  fn _ZNK13QGraphicsItem9hasCursorEv(qthis: *mut c_void) -> c_char;
  // proto:  void QGraphicsItem::setGraphicsEffect(QGraphicsEffect * effect);
  fn _ZN13QGraphicsItem17setGraphicsEffectEP15QGraphicsEffect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPainterPath QGraphicsItem::mapToParent(const QPainterPath & path);
  fn _ZNK13QGraphicsItem11mapToParentERK12QPainterPath(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::ensureVisible(qreal x, qreal y, qreal w, qreal h, int xmargin, int ymargin);
  fn _ZN13QGraphicsItem13ensureVisibleEddddii(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_int, arg5: c_int);
  // proto:  QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem9mapToItemEPKS_dddd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double) -> *mut c_void;
  // proto:  QPointF QGraphicsItem::mapToItem(const QGraphicsItem * item, qreal x, qreal y);
  fn _ZNK13QGraphicsItem9mapToItemEPKS_dd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: c_double) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::mapRectToParent(const QRectF & rect);
  fn _ZNK13QGraphicsItem15mapRectToParentERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::setToolTip(const QString & toolTip);
  fn _ZN13QGraphicsItem10setToolTipERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QGraphicsItem::rotation();
  fn _ZNK13QGraphicsItem8rotationEv(qthis: *mut c_void) -> c_double;
  // proto:  QGraphicsScene * QGraphicsItem::scene();
  fn _ZNK13QGraphicsItem5sceneEv(qthis: *mut c_void);
  // proto:  QPainterPath QGraphicsItem::mapToItem(const QGraphicsItem * item, const QPainterPath & path);
  fn _ZNK13QGraphicsItem9mapToItemEPKS_RK12QPainterPath(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::mapRectToParent(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem15mapRectToParentEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  QPolygonF QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QRectF & rect);
  fn _ZNK13QGraphicsItem11mapFromItemEPKS_RK6QRectF(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::mapRectFromParent(const QRectF & rect);
  fn _ZNK13QGraphicsItem17mapRectFromParentERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::setFocusProxy(QGraphicsItem * item);
  fn _ZN13QGraphicsItem13setFocusProxyEPS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QGraphicsItem::acceptDrops();
  fn _ZNK13QGraphicsItem11acceptDropsEv(qthis: *mut c_void) -> c_char;
  // proto:  QPointF QGraphicsItem::mapToParent(const QPointF & point);
  fn _ZNK13QGraphicsItem11mapToParentERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::mapRectFromScene(const QRectF & rect);
  fn _ZNK13QGraphicsItem16mapRectFromSceneERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QGraphicsItem * QGraphicsItem::focusScopeItem();
  fn _ZNK13QGraphicsItem14focusScopeItemEv(qthis: *mut c_void);
  // proto:  void QGraphicsItem::removeSceneEventFilter(QGraphicsItem * filterItem);
  fn _ZN13QGraphicsItem22removeSceneEventFilterEPS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QGraphicsItem * QGraphicsItem::focusProxy();
  fn _ZNK13QGraphicsItem10focusProxyEv(qthis: *mut c_void);
  // proto:  QPointF QGraphicsItem::mapToItem(const QGraphicsItem * item, const QPointF & point);
  fn _ZNK13QGraphicsItem9mapToItemEPKS_RK7QPointF(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::sceneBoundingRect();
  fn _ZNK13QGraphicsItem17sceneBoundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::~QGraphicsItem();
  fn _ZN13QGraphicsItemD0Ev(qthis: *mut c_void);
  // proto:  void QGraphicsItem::setX(qreal x);
  fn _ZN13QGraphicsItem4setXEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsItem::update(qreal x, qreal y, qreal width, qreal height);
  fn _ZN13QGraphicsItem6updateEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  void QGraphicsItem::setSelected(bool selected);
  fn _ZN13QGraphicsItem11setSelectedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QRectF QGraphicsItem::mapRectToItem(const QGraphicsItem * item, const QRectF & rect);
  fn _ZNK13QGraphicsItem13mapRectToItemEPKS_RK6QRectF(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::stackBefore(const QGraphicsItem * sibling);
  fn _ZN13QGraphicsItem11stackBeforeEPKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPointF QGraphicsItem::mapFromItem(const QGraphicsItem * item, qreal x, qreal y);
  fn _ZNK13QGraphicsItem11mapFromItemEPKS_dd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: c_double) -> *mut c_void;
  // proto:  void QGraphicsItem::resetMatrix();
  fn _ZN13QGraphicsItem11resetMatrixEv(qthis: *mut c_void);
  // proto:  QPainterPath QGraphicsItem::opaqueArea();
  fn _ZNK13QGraphicsItem10opaqueAreaEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::unsetCursor();
  fn _ZN13QGraphicsItem11unsetCursorEv(qthis: *mut c_void);
  // proto:  QPointF QGraphicsItem::mapFromParent(qreal x, qreal y);
  fn _ZNK13QGraphicsItem13mapFromParentEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::mapRectToScene(const QRectF & rect);
  fn _ZNK13QGraphicsItem14mapRectToSceneERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::mapRectFromItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem15mapRectFromItemEPKS_dddd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double) -> *mut c_void;
  // proto:  qreal QGraphicsItem::scale();
  fn _ZNK13QGraphicsItem5scaleEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsItem::setBoundingRegionGranularity(qreal granularity);
  fn _ZN13QGraphicsItem28setBoundingRegionGranularityEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsItem::setAcceptDrops(bool on);
  fn _ZN13QGraphicsItem14setAcceptDropsEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QPolygonF QGraphicsItem::mapFromScene(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem12mapFromSceneEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  void QGraphicsItem::ungrabKeyboard();
  fn _ZN13QGraphicsItem14ungrabKeyboardEv(qthis: *mut c_void);
  // proto:  void QGraphicsItem::setEnabled(bool enabled);
  fn _ZN13QGraphicsItem10setEnabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QGraphicsEffect * QGraphicsItem::graphicsEffect();
  fn _ZNK13QGraphicsItem14graphicsEffectEv(qthis: *mut c_void);
  // proto:  bool QGraphicsItem::acceptHoverEvents();
  fn _ZNK13QGraphicsItem17acceptHoverEventsEv(qthis: *mut c_void) -> c_char;
  // proto:  QGraphicsWidget * QGraphicsItem::topLevelWidget();
  fn _ZNK13QGraphicsItem14topLevelWidgetEv(qthis: *mut c_void);
  // proto:  QList<QGraphicsTransform *> QGraphicsItem::transformations();
  fn _ZNK13QGraphicsItem15transformationsEv(qthis: *mut c_void);
  // proto:  QPolygonF QGraphicsItem::mapToScene(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem10mapToSceneEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  QPointF QGraphicsItem::mapToScene(qreal x, qreal y);
  fn _ZNK13QGraphicsItem10mapToSceneEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::mapRectFromScene(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem16mapRectFromSceneEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  void QGraphicsItem::advance(int phase);
  fn _ZN13QGraphicsItem7advanceEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QMatrix QGraphicsItem::sceneMatrix();
  fn _ZNK13QGraphicsItem11sceneMatrixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::setFiltersChildEvents(bool enabled);
  fn _ZN13QGraphicsItem21setFiltersChildEventsEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QPolygonF QGraphicsItem::mapToScene(const QPolygonF & polygon);
  fn _ZNK13QGraphicsItem10mapToSceneERK9QPolygonF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QTransform QGraphicsItem::itemTransform(const QGraphicsItem * other, bool * ok);
  fn _ZNK13QGraphicsItem13itemTransformEPKS_Pb(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char) -> *mut c_void;
  // proto:  void QGraphicsItem::setTransformOriginPoint(qreal ax, qreal ay);
  fn _ZN13QGraphicsItem23setTransformOriginPointEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QGraphicsItem::moveBy(qreal dx, qreal dy);
  fn _ZN13QGraphicsItem6moveByEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  QPolygonF QGraphicsItem::mapFromScene(const QPolygonF & polygon);
  fn _ZNK13QGraphicsItem12mapFromSceneERK9QPolygonF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QGraphicsItemGroup * QGraphicsItem::group();
  fn _ZNK13QGraphicsItem5groupEv(qthis: *mut c_void);
  // proto:  QPainterPath QGraphicsItem::shape();
  fn _ZNK13QGraphicsItem5shapeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPointF QGraphicsItem::mapFromScene(qreal x, qreal y);
  fn _ZNK13QGraphicsItem12mapFromSceneEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  void QGraphicsItem::scroll(qreal dx, qreal dy, const QRectF & rect);
  fn _ZN13QGraphicsItem6scrollEddRK6QRectF(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: *mut c_void);
  // proto:  bool QGraphicsItem::isObscuredBy(const QGraphicsItem * item);
  fn _ZNK13QGraphicsItem12isObscuredByEPKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QPointF QGraphicsItem::mapFromParent(const QPointF & point);
  fn _ZNK13QGraphicsItem13mapFromParentERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::setData(int key, const QVariant & value);
  fn _ZN13QGraphicsItem7setDataEiRK8QVariant(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  QGraphicsItem * QGraphicsItem::commonAncestorItem(const QGraphicsItem * other);
  fn _ZNK13QGraphicsItem18commonAncestorItemEPKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPainterPath QGraphicsItem::mapFromScene(const QPainterPath & path);
  fn _ZNK13QGraphicsItem12mapFromSceneERK12QPainterPath(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPainterPath QGraphicsItem::mapToScene(const QPainterPath & path);
  fn _ZNK13QGraphicsItem10mapToSceneERK12QPainterPath(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPolygonF QGraphicsItem::mapToParent(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem11mapToParentEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  void QGraphicsItem::setGroup(QGraphicsItemGroup * group);
  fn _ZN13QGraphicsItem8setGroupEP18QGraphicsItemGroup(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRectF QGraphicsItem::mapRectFromParent(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem17mapRectFromParentEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  void QGraphicsItem::show();
  fn _ZN13QGraphicsItem4showEv(qthis: *mut c_void);
  // proto:  QRectF QGraphicsItem::mapRectFromItem(const QGraphicsItem * item, const QRectF & rect);
  fn _ZNK13QGraphicsItem15mapRectFromItemEPKS_RK6QRectF(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  qreal QGraphicsItem::y();
  fn _ZNK13QGraphicsItem1yEv(qthis: *mut c_void);
  // proto:  QPointF QGraphicsItem::mapFromScene(const QPointF & point);
  fn _ZNK13QGraphicsItem12mapFromSceneERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsItem::hasFocus();
  fn _ZNK13QGraphicsItem8hasFocusEv(qthis: *mut c_void) -> c_char;
  // proto:  QPainterPath QGraphicsItem::clipPath();
  fn _ZNK13QGraphicsItem8clipPathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::setPos(qreal x, qreal y);
  fn _ZN13QGraphicsItem6setPosEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  bool QGraphicsItem::isEnabled();
  fn _ZNK13QGraphicsItem9isEnabledEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QGraphicsItem::contains(const QPointF & point);
  fn _ZNK13QGraphicsItem8containsERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  bool QGraphicsItem::isPanel();
  fn _ZNK13QGraphicsItem7isPanelEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QGraphicsItem::filtersChildEvents();
  fn _ZNK13QGraphicsItem18filtersChildEventsEv(qthis: *mut c_void) -> c_char;
  // proto:  void QGraphicsItem::grabKeyboard();
  fn _ZN13QGraphicsItem12grabKeyboardEv(qthis: *mut c_void);
  // proto:  QPainterPath QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QPainterPath & path);
  fn _ZNK13QGraphicsItem11mapFromItemEPKS_RK12QPainterPath(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::setActive(bool active);
  fn _ZN13QGraphicsItem9setActiveEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QGraphicsObject * QGraphicsItem::toGraphicsObject();
  fn _ZN13QGraphicsItem16toGraphicsObjectEv(qthis: *mut c_void);
  // proto:  QPolygonF QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QPolygonF & polygon);
  fn _ZNK13QGraphicsItem11mapFromItemEPKS_RK9QPolygonF(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::setHandlesChildEvents(bool enabled);
  fn _ZN13QGraphicsItem21setHandlesChildEventsEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QPolygonF QGraphicsItem::mapFromParent(const QPolygonF & polygon);
  fn _ZNK13QGraphicsItem13mapFromParentERK9QPolygonF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPointF QGraphicsItem::mapToParent(qreal x, qreal y);
  fn _ZNK13QGraphicsItem11mapToParentEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  void QGraphicsItem::setMatrix(const QMatrix & matrix, bool combine);
  fn _ZN13QGraphicsItem9setMatrixERK7QMatrixb(qthis: *mut c_void, arg0: *mut c_void, arg1: c_char);
  // proto:  void QGraphicsItem::update(const QRectF & rect);
  fn _ZN13QGraphicsItem6updateERK6QRectF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem * item, const QPolygonF & polygon);
  fn _ZNK13QGraphicsItem9mapToItemEPKS_RK9QPolygonF(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QTransform QGraphicsItem::transform();
  fn _ZNK13QGraphicsItem9transformEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QVariant QGraphicsItem::data(int key);
  fn _ZNK13QGraphicsItem4dataEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QGraphicsItem::hide();
  fn _ZN13QGraphicsItem4hideEv(qthis: *mut c_void);
  // proto:  bool QGraphicsItem::isUnderMouse();
  fn _ZNK13QGraphicsItem12isUnderMouseEv(qthis: *mut c_void) -> c_char;
  // proto:  void QGraphicsItem::setAcceptTouchEvents(bool enabled);
  fn _ZN13QGraphicsItem20setAcceptTouchEventsEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QGraphicsItem::setAcceptHoverEvents(bool enabled);
  fn _ZN13QGraphicsItem20setAcceptHoverEventsEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QList<QGraphicsItem *> QGraphicsItem::childItems();
  fn _ZNK13QGraphicsItem10childItemsEv(qthis: *mut c_void);
  // proto:  bool QGraphicsItem::isAncestorOf(const QGraphicsItem * child);
  fn _ZNK13QGraphicsItem12isAncestorOfEPKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  qreal QGraphicsItem::opacity();
  fn _ZNK13QGraphicsItem7opacityEv(qthis: *mut c_void) -> c_double;
  // proto:  bool QGraphicsItem::isVisibleTo(const QGraphicsItem * parent);
  fn _ZNK13QGraphicsItem11isVisibleToEPKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QString QGraphicsItem::toolTip();
  fn _ZNK13QGraphicsItem7toolTipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QCursor QGraphicsItem::cursor();
  fn _ZNK13QGraphicsItem6cursorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPointF QGraphicsItem::mapToScene(const QPointF & point);
  fn _ZNK13QGraphicsItem10mapToSceneERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  qreal QGraphicsItem::zValue();
  fn _ZNK13QGraphicsItem6zValueEv(qthis: *mut c_void) -> c_double;
  // proto:  QMatrix QGraphicsItem::matrix();
  fn _ZNK13QGraphicsItem6matrixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::mapRectToScene(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem14mapRectToSceneEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  void QGraphicsItem::setPos(const QPointF & pos);
  fn _ZN13QGraphicsItem6setPosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QGraphicsItem * QGraphicsItem::panel();
  fn _ZNK13QGraphicsItem5panelEv(qthis: *mut c_void);
  // proto:  bool QGraphicsItem::isClipped();
  fn _ZNK13QGraphicsItem9isClippedEv(qthis: *mut c_void) -> c_char;
  // proto:  QGraphicsItem * QGraphicsItem::topLevelItem();
  fn _ZNK13QGraphicsItem12topLevelItemEv(qthis: *mut c_void);
  // proto:  QPolygonF QGraphicsItem::mapToScene(const QRectF & rect);
  fn _ZNK13QGraphicsItem10mapToSceneERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::setScale(qreal scale);
  fn _ZN13QGraphicsItem8setScaleEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsItem::setCursor(const QCursor & cursor);
  fn _ZN13QGraphicsItem9setCursorERK7QCursor(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QGraphicsItem::isVisible();
  fn _ZNK13QGraphicsItem9isVisibleEv(qthis: *mut c_void) -> c_char;
  // proto:  QPointF QGraphicsItem::pos();
  fn _ZNK13QGraphicsItem3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsItem::isBlockedByModalPanel(QGraphicsItem ** blockingPanel);
  fn _ZNK13QGraphicsItem21isBlockedByModalPanelEPPS_(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  qreal QGraphicsItem::effectiveOpacity();
  fn _ZNK13QGraphicsItem16effectiveOpacityEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsItem::ensureVisible(const QRectF & rect, int xmargin, int ymargin);
  fn _ZN13QGraphicsItem13ensureVisibleERK6QRectFii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  qreal QGraphicsItem::boundingRegionGranularity();
  fn _ZNK13QGraphicsItem25boundingRegionGranularityEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QGraphicsItem::x();
  fn _ZNK13QGraphicsItem1xEv(qthis: *mut c_void);
  // proto:  void QGraphicsItem::grabMouse();
  fn _ZN13QGraphicsItem9grabMouseEv(qthis: *mut c_void);
  // proto:  void QGraphicsItem::setVisible(bool visible);
  fn _ZN13QGraphicsItem10setVisibleEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QGraphicsItem::setRotation(qreal angle);
  fn _ZN13QGraphicsItem11setRotationEd(qthis: *mut c_void, arg0: c_double);
  // proto:  QTransform QGraphicsItem::deviceTransform(const QTransform & viewportTransform);
  fn _ZNK13QGraphicsItem15deviceTransformERK10QTransform(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsItem::acceptTouchEvents();
  fn _ZNK13QGraphicsItem17acceptTouchEventsEv(qthis: *mut c_void) -> c_char;
  // proto:  void QGraphicsItem::setTransform(const QTransform & matrix, bool combine);
  fn _ZN13QGraphicsItem12setTransformERK10QTransformb(qthis: *mut c_void, arg0: *mut c_void, arg1: c_char);
  // proto:  QPolygonF QGraphicsItem::mapToParent(const QRectF & rect);
  fn _ZNK13QGraphicsItem11mapToParentERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QGraphicsItem)=1
pub struct QGraphicsItem {
  pub qclsinst: *mut c_void,
}

  // proto:  void QGraphicsItem::QGraphicsItem(const QGraphicsItem & );
impl /*struct*/ QGraphicsItem {
  pub fn NewQGraphicsItem<T: QGraphicsItem_NewQGraphicsItem>(value: T) -> QGraphicsItem {
    let rsthis = value.NewQGraphicsItem();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsItem_NewQGraphicsItem {
  fn NewQGraphicsItem(self) -> QGraphicsItem;
}

  // proto:  void QGraphicsItem::QGraphicsItem(const QGraphicsItem & );
impl<'a> /*trait*/ QGraphicsItem_NewQGraphicsItem for (QGraphicsItem) {
  fn NewQGraphicsItem(self) -> QGraphicsItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItemC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QGraphicsItemC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsItem::mapFromParent(const QPainterPath & path);
impl /*struct*/ QGraphicsItem {
  pub fn mapFromParent<RetType, T: QGraphicsItem_mapFromParent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapFromParent(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapFromParent<RetType> {
  fn mapFromParent(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsItem::mapFromParent(const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsItem_mapFromParent<QPainterPath> for (QPainterPath) {
  fn mapFromParent(self , rsthis: &mut QGraphicsItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapFromParentERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem13mapFromParentERK12QPainterPath(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QPointF & point);
impl /*struct*/ QGraphicsItem {
  pub fn mapFromItem<RetType, T: QGraphicsItem_mapFromItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapFromItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapFromItem<RetType> {
  fn mapFromItem(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QPointF QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QPointF & point);
impl<'a> /*trait*/ QGraphicsItem_mapFromItem<QPointF> for (QGraphicsItem, QPointF) {
  fn mapFromItem(self , rsthis: &mut QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapFromItemEPKS_RK7QPointF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem11mapFromItemEPKS_RK7QPointF(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsItem::focusItem();
impl /*struct*/ QGraphicsItem {
  pub fn focusItem<RetType, T: QGraphicsItem_focusItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.focusItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_focusItem<RetType> {
  fn focusItem(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsItem * QGraphicsItem::focusItem();
impl<'a> /*trait*/ QGraphicsItem_focusItem<()> for () {
  fn focusItem(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9focusItemEv()};
     unsafe {_ZNK13QGraphicsItem9focusItemEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QGraphicsObject * QGraphicsItem::parentObject();
impl /*struct*/ QGraphicsItem {
  pub fn parentObject<RetType, T: QGraphicsItem_parentObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.parentObject(self);
    // return 1;
  }
}

pub trait QGraphicsItem_parentObject<RetType> {
  fn parentObject(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsObject * QGraphicsItem::parentObject();
impl<'a> /*trait*/ QGraphicsItem_parentObject<()> for () {
  fn parentObject(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12parentObjectEv()};
     unsafe {_ZNK13QGraphicsItem12parentObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setTransformOriginPoint(const QPointF & origin);
impl /*struct*/ QGraphicsItem {
  pub fn setTransformOriginPoint<RetType, T: QGraphicsItem_setTransformOriginPoint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTransformOriginPoint(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setTransformOriginPoint<RetType> {
  fn setTransformOriginPoint(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setTransformOriginPoint(const QPointF & origin);
impl<'a> /*trait*/ QGraphicsItem_setTransformOriginPoint<()> for (QPointF) {
  fn setTransformOriginPoint(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem23setTransformOriginPointERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem23setTransformOriginPointERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::ungrabMouse();
impl /*struct*/ QGraphicsItem {
  pub fn ungrabMouse<RetType, T: QGraphicsItem_ungrabMouse<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.ungrabMouse(self);
    // return 1;
  }
}

pub trait QGraphicsItem_ungrabMouse<RetType> {
  fn ungrabMouse(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::ungrabMouse();
impl<'a> /*trait*/ QGraphicsItem_ungrabMouse<()> for () {
  fn ungrabMouse(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11ungrabMouseEv()};
     unsafe {_ZN13QGraphicsItem11ungrabMouseEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QGraphicsItem::type();
impl /*struct*/ QGraphicsItem {
  pub fn type_<RetType, T: QGraphicsItem_type_<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QGraphicsItem_type_<RetType> {
  fn type_(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  int QGraphicsItem::type();
impl<'a> /*trait*/ QGraphicsItem_type_<i32> for () {
  fn type_(self , rsthis: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem4typeEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isSelected();
impl /*struct*/ QGraphicsItem {
  pub fn isSelected<RetType, T: QGraphicsItem_isSelected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isSelected(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isSelected<RetType> {
  fn isSelected(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isSelected();
impl<'a> /*trait*/ QGraphicsItem_isSelected<i8> for () {
  fn isSelected(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10isSelectedEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem10isSelectedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapFromItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapFromItem<QPolygonF> for (QGraphicsItem, f64, f64, f64, f64) {
  fn mapFromItem(self , rsthis: &mut QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapFromItemEPKS_dddd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem11mapFromItemEPKS_dddd(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QGraphicsWidget * QGraphicsItem::parentWidget();
impl /*struct*/ QGraphicsItem {
  pub fn parentWidget<RetType, T: QGraphicsItem_parentWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.parentWidget(self);
    // return 1;
  }
}

pub trait QGraphicsItem_parentWidget<RetType> {
  fn parentWidget(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsWidget * QGraphicsItem::parentWidget();
impl<'a> /*trait*/ QGraphicsItem_parentWidget<()> for () {
  fn parentWidget(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12parentWidgetEv()};
     unsafe {_ZNK13QGraphicsItem12parentWidgetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::resetTransform();
impl /*struct*/ QGraphicsItem {
  pub fn resetTransform<RetType, T: QGraphicsItem_resetTransform<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.resetTransform(self);
    // return 1;
  }
}

pub trait QGraphicsItem_resetTransform<RetType> {
  fn resetTransform(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::resetTransform();
impl<'a> /*trait*/ QGraphicsItem_resetTransform<()> for () {
  fn resetTransform(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem14resetTransformEv()};
     unsafe {_ZN13QGraphicsItem14resetTransformEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRegion QGraphicsItem::boundingRegion(const QTransform & itemToDeviceTransform);
impl /*struct*/ QGraphicsItem {
  pub fn boundingRegion<RetType, T: QGraphicsItem_boundingRegion<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.boundingRegion(self);
    // return 1;
  }
}

pub trait QGraphicsItem_boundingRegion<RetType> {
  fn boundingRegion(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QRegion QGraphicsItem::boundingRegion(const QTransform & itemToDeviceTransform);
impl<'a> /*trait*/ QGraphicsItem_boundingRegion<QRegion> for (QTransform) {
  fn boundingRegion(self , rsthis: &mut QGraphicsItem) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14boundingRegionERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem14boundingRegionERK10QTransform(rsthis.qclsinst, arg0)};
    let mut ret1 = QRegion{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl /*struct*/ QGraphicsItem {
  pub fn paint<RetType, T: QGraphicsItem_paint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.paint(self);
    // return 1;
  }
}

pub trait QGraphicsItem_paint<RetType> {
  fn paint(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsItem_paint<()> for (QPainter, QStyleOptionGraphicsItem, QWidget) {
  fn paint(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isActive();
impl /*struct*/ QGraphicsItem {
  pub fn isActive<RetType, T: QGraphicsItem_isActive<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isActive(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isActive<RetType> {
  fn isActive(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isActive();
impl<'a> /*trait*/ QGraphicsItem_isActive<i8> for () {
  fn isActive(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8isActiveEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem8isActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapToParent(const QPolygonF & polygon);
impl /*struct*/ QGraphicsItem {
  pub fn mapToParent<RetType, T: QGraphicsItem_mapToParent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapToParent(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapToParent<RetType> {
  fn mapToParent(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QPolygonF QGraphicsItem::mapToParent(const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsItem_mapToParent<QPolygonF> for (QPolygonF) {
  fn mapToParent(self , rsthis: &mut QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapToParentERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem11mapToParentERK9QPolygonF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isWidget();
impl /*struct*/ QGraphicsItem {
  pub fn isWidget<RetType, T: QGraphicsItem_isWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isWidget(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isWidget<RetType> {
  fn isWidget(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isWidget();
impl<'a> /*trait*/ QGraphicsItem_isWidget<i8> for () {
  fn isWidget(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8isWidgetEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem8isWidgetEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setParentItem(QGraphicsItem * parent);
impl /*struct*/ QGraphicsItem {
  pub fn setParentItem<RetType, T: QGraphicsItem_setParentItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setParentItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setParentItem<RetType> {
  fn setParentItem(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setParentItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsItem_setParentItem<()> for (QGraphicsItem) {
  fn setParentItem(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem13setParentItemEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem13setParentItemEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem * item, const QRectF & rect);
impl /*struct*/ QGraphicsItem {
  pub fn mapToItem<RetType, T: QGraphicsItem_mapToItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapToItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapToItem<RetType> {
  fn mapToItem(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem * item, const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapToItem<QPolygonF> for (QGraphicsItem, QRectF) {
  fn mapToItem(self , rsthis: &mut QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9mapToItemEPKS_RK6QRectF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem9mapToItemEPKS_RK6QRectF(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QGraphicsWidget * QGraphicsItem::window();
impl /*struct*/ QGraphicsItem {
  pub fn window<RetType, T: QGraphicsItem_window<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.window(self);
    // return 1;
  }
}

pub trait QGraphicsItem_window<RetType> {
  fn window(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsWidget * QGraphicsItem::window();
impl<'a> /*trait*/ QGraphicsItem_window<()> for () {
  fn window(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem6windowEv()};
     unsafe {_ZNK13QGraphicsItem6windowEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::scenePos();
impl /*struct*/ QGraphicsItem {
  pub fn scenePos<RetType, T: QGraphicsItem_scenePos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.scenePos(self);
    // return 1;
  }
}

pub trait QGraphicsItem_scenePos<RetType> {
  fn scenePos(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QPointF QGraphicsItem::scenePos();
impl<'a> /*trait*/ QGraphicsItem_scenePos<QPointF> for () {
  fn scenePos(self , rsthis: &mut QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8scenePosEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem8scenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::handlesChildEvents();
impl /*struct*/ QGraphicsItem {
  pub fn handlesChildEvents<RetType, T: QGraphicsItem_handlesChildEvents<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.handlesChildEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_handlesChildEvents<RetType> {
  fn handlesChildEvents(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::handlesChildEvents();
impl<'a> /*trait*/ QGraphicsItem_handlesChildEvents<i8> for () {
  fn handlesChildEvents(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem18handlesChildEventsEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem18handlesChildEventsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setOpacity(qreal opacity);
impl /*struct*/ QGraphicsItem {
  pub fn setOpacity<RetType, T: QGraphicsItem_setOpacity<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setOpacity(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setOpacity<RetType> {
  fn setOpacity(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setOpacity(qreal opacity);
impl<'a> /*trait*/ QGraphicsItem_setOpacity<()> for (f64) {
  fn setOpacity(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem10setOpacityEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QGraphicsItem10setOpacityEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTransform QGraphicsItem::sceneTransform();
impl /*struct*/ QGraphicsItem {
  pub fn sceneTransform<RetType, T: QGraphicsItem_sceneTransform<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sceneTransform(self);
    // return 1;
  }
}

pub trait QGraphicsItem_sceneTransform<RetType> {
  fn sceneTransform(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QTransform QGraphicsItem::sceneTransform();
impl<'a> /*trait*/ QGraphicsItem_sceneTransform<QTransform> for () {
  fn sceneTransform(self , rsthis: &mut QGraphicsItem) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14sceneTransformEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem14sceneTransformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setZValue(qreal z);
impl /*struct*/ QGraphicsItem {
  pub fn setZValue<RetType, T: QGraphicsItem_setZValue<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setZValue(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setZValue<RetType> {
  fn setZValue(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setZValue(qreal z);
impl<'a> /*trait*/ QGraphicsItem_setZValue<()> for (f64) {
  fn setZValue(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem9setZValueEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QGraphicsItem9setZValueEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapFromParent(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapFromParent<QPolygonF> for (QRectF) {
  fn mapFromParent(self , rsthis: &mut QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapFromParentERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem13mapFromParentERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapFromParent(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapFromParent<QPolygonF> for (f64, f64, f64, f64) {
  fn mapFromParent(self , rsthis: &mut QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapFromParentEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem13mapFromParentEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isObscured(qreal x, qreal y, qreal w, qreal h);
impl /*struct*/ QGraphicsItem {
  pub fn isObscured<RetType, T: QGraphicsItem_isObscured<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isObscured(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isObscured<RetType> {
  fn isObscured(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isObscured(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_isObscured<i8> for (f64, f64, f64, f64) {
  fn isObscured(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10isObscuredEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem10isObscuredEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::installSceneEventFilter(QGraphicsItem * filterItem);
impl /*struct*/ QGraphicsItem {
  pub fn installSceneEventFilter<RetType, T: QGraphicsItem_installSceneEventFilter<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.installSceneEventFilter(self);
    // return 1;
  }
}

pub trait QGraphicsItem_installSceneEventFilter<RetType> {
  fn installSceneEventFilter(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::installSceneEventFilter(QGraphicsItem * filterItem);
impl<'a> /*trait*/ QGraphicsItem_installSceneEventFilter<()> for (QGraphicsItem) {
  fn installSceneEventFilter(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem23installSceneEventFilterEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem23installSceneEventFilterEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setY(qreal y);
impl /*struct*/ QGraphicsItem {
  pub fn setY<RetType, T: QGraphicsItem_setY<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setY(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setY<RetType> {
  fn setY(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setY(qreal y);
impl<'a> /*trait*/ QGraphicsItem_setY<()> for (f64) {
  fn setY(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem4setYEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QGraphicsItem4setYEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectToItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
impl /*struct*/ QGraphicsItem {
  pub fn mapRectToItem<RetType, T: QGraphicsItem_mapRectToItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapRectToItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapRectToItem<RetType> {
  fn mapRectToItem(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QRectF QGraphicsItem::mapRectToItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectToItem<QRectF> for (QGraphicsItem, f64, f64, f64, f64) {
  fn mapRectToItem(self , rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapRectToItemEPKS_dddd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem13mapRectToItemEPKS_dddd(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsItem::parentItem();
impl /*struct*/ QGraphicsItem {
  pub fn parentItem<RetType, T: QGraphicsItem_parentItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.parentItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_parentItem<RetType> {
  fn parentItem(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsItem * QGraphicsItem::parentItem();
impl<'a> /*trait*/ QGraphicsItem_parentItem<()> for () {
  fn parentItem(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10parentItemEv()};
     unsafe {_ZNK13QGraphicsItem10parentItemEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::clearFocus();
impl /*struct*/ QGraphicsItem {
  pub fn clearFocus<RetType, T: QGraphicsItem_clearFocus<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clearFocus(self);
    // return 1;
  }
}

pub trait QGraphicsItem_clearFocus<RetType> {
  fn clearFocus(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::clearFocus();
impl<'a> /*trait*/ QGraphicsItem_clearFocus<()> for () {
  fn clearFocus(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem10clearFocusEv()};
     unsafe {_ZN13QGraphicsItem10clearFocusEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isWindow();
impl /*struct*/ QGraphicsItem {
  pub fn isWindow<RetType, T: QGraphicsItem_isWindow<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isWindow(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isWindow<RetType> {
  fn isWindow(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isWindow();
impl<'a> /*trait*/ QGraphicsItem_isWindow<i8> for () {
  fn isWindow(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8isWindowEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem8isWindowEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::transformOriginPoint();
impl /*struct*/ QGraphicsItem {
  pub fn transformOriginPoint<RetType, T: QGraphicsItem_transformOriginPoint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.transformOriginPoint(self);
    // return 1;
  }
}

pub trait QGraphicsItem_transformOriginPoint<RetType> {
  fn transformOriginPoint(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QPointF QGraphicsItem::transformOriginPoint();
impl<'a> /*trait*/ QGraphicsItem_transformOriginPoint<QPointF> for () {
  fn transformOriginPoint(self , rsthis: &mut QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem20transformOriginPointEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem20transformOriginPointEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::boundingRect();
impl /*struct*/ QGraphicsItem {
  pub fn boundingRect<RetType, T: QGraphicsItem_boundingRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsItem_boundingRect<RetType> {
  fn boundingRect(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QRectF QGraphicsItem::boundingRect();
impl<'a> /*trait*/ QGraphicsItem_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12boundingRectEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::childrenBoundingRect();
impl /*struct*/ QGraphicsItem {
  pub fn childrenBoundingRect<RetType, T: QGraphicsItem_childrenBoundingRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.childrenBoundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsItem_childrenBoundingRect<RetType> {
  fn childrenBoundingRect(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QRectF QGraphicsItem::childrenBoundingRect();
impl<'a> /*trait*/ QGraphicsItem_childrenBoundingRect<QRectF> for () {
  fn childrenBoundingRect(self , rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem20childrenBoundingRectEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem20childrenBoundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isObscured(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_isObscured<i8> for (QRectF) {
  fn isObscured(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10isObscuredERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem10isObscuredERK6QRectF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapFromScene(const QRectF & rect);
impl /*struct*/ QGraphicsItem {
  pub fn mapFromScene<RetType, T: QGraphicsItem_mapFromScene<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapFromScene(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapFromScene<RetType> {
  fn mapFromScene(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QPolygonF QGraphicsItem::mapFromScene(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapFromScene<QPolygonF> for (QRectF) {
  fn mapFromScene(self , rsthis: &mut QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12mapFromSceneERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem12mapFromSceneERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::hasCursor();
impl /*struct*/ QGraphicsItem {
  pub fn hasCursor<RetType, T: QGraphicsItem_hasCursor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasCursor(self);
    // return 1;
  }
}

pub trait QGraphicsItem_hasCursor<RetType> {
  fn hasCursor(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::hasCursor();
impl<'a> /*trait*/ QGraphicsItem_hasCursor<i8> for () {
  fn hasCursor(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9hasCursorEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem9hasCursorEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setGraphicsEffect(QGraphicsEffect * effect);
impl /*struct*/ QGraphicsItem {
  pub fn setGraphicsEffect<RetType, T: QGraphicsItem_setGraphicsEffect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setGraphicsEffect(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setGraphicsEffect<RetType> {
  fn setGraphicsEffect(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setGraphicsEffect(QGraphicsEffect * effect);
impl<'a> /*trait*/ QGraphicsItem_setGraphicsEffect<()> for (QGraphicsEffect) {
  fn setGraphicsEffect(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem17setGraphicsEffectEP15QGraphicsEffect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem17setGraphicsEffectEP15QGraphicsEffect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsItem::mapToParent(const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsItem_mapToParent<QPainterPath> for (QPainterPath) {
  fn mapToParent(self , rsthis: &mut QGraphicsItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapToParentERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem11mapToParentERK12QPainterPath(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::ensureVisible(qreal x, qreal y, qreal w, qreal h, int xmargin, int ymargin);
impl /*struct*/ QGraphicsItem {
  pub fn ensureVisible<RetType, T: QGraphicsItem_ensureVisible<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.ensureVisible(self);
    // return 1;
  }
}

pub trait QGraphicsItem_ensureVisible<RetType> {
  fn ensureVisible(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::ensureVisible(qreal x, qreal y, qreal w, qreal h, int xmargin, int ymargin);
impl<'a> /*trait*/ QGraphicsItem_ensureVisible<()> for (f64, f64, f64, f64, i32, i32) {
  fn ensureVisible(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem13ensureVisibleEddddii()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
     unsafe {_ZN13QGraphicsItem13ensureVisibleEddddii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapToItem<QPolygonF> for (QGraphicsItem, f64, f64, f64, f64) {
  fn mapToItem(self , rsthis: &mut QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9mapToItemEPKS_dddd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem9mapToItemEPKS_dddd(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapToItem(const QGraphicsItem * item, qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsItem_mapToItem<QPointF> for (QGraphicsItem, f64, f64) {
  fn mapToItem(self , rsthis: &mut QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9mapToItemEPKS_dd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem9mapToItemEPKS_dd(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectToParent(const QRectF & rect);
impl /*struct*/ QGraphicsItem {
  pub fn mapRectToParent<RetType, T: QGraphicsItem_mapRectToParent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapRectToParent(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapRectToParent<RetType> {
  fn mapRectToParent(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QRectF QGraphicsItem::mapRectToParent(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectToParent<QRectF> for (QRectF) {
  fn mapRectToParent(self , rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15mapRectToParentERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem15mapRectToParentERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setToolTip(const QString & toolTip);
impl /*struct*/ QGraphicsItem {
  pub fn setToolTip<RetType, T: QGraphicsItem_setToolTip<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setToolTip(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setToolTip<RetType> {
  fn setToolTip(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setToolTip(const QString & toolTip);
impl<'a> /*trait*/ QGraphicsItem_setToolTip<()> for (QString) {
  fn setToolTip(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem10setToolTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsItem::rotation();
impl /*struct*/ QGraphicsItem {
  pub fn rotation<RetType, T: QGraphicsItem_rotation<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rotation(self);
    // return 1;
  }
}

pub trait QGraphicsItem_rotation<RetType> {
  fn rotation(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  qreal QGraphicsItem::rotation();
impl<'a> /*trait*/ QGraphicsItem_rotation<f64> for () {
  fn rotation(self , rsthis: &mut QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8rotationEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem8rotationEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QGraphicsScene * QGraphicsItem::scene();
impl /*struct*/ QGraphicsItem {
  pub fn scene<RetType, T: QGraphicsItem_scene<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.scene(self);
    // return 1;
  }
}

pub trait QGraphicsItem_scene<RetType> {
  fn scene(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsScene * QGraphicsItem::scene();
impl<'a> /*trait*/ QGraphicsItem_scene<()> for () {
  fn scene(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem5sceneEv()};
     unsafe {_ZNK13QGraphicsItem5sceneEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsItem::mapToItem(const QGraphicsItem * item, const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsItem_mapToItem<QPainterPath> for (QGraphicsItem, QPainterPath) {
  fn mapToItem(self , rsthis: &mut QGraphicsItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9mapToItemEPKS_RK12QPainterPath()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem9mapToItemEPKS_RK12QPainterPath(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectToParent(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectToParent<QRectF> for (f64, f64, f64, f64) {
  fn mapRectToParent(self , rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15mapRectToParentEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem15mapRectToParentEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapFromItem<QPolygonF> for (QGraphicsItem, QRectF) {
  fn mapFromItem(self , rsthis: &mut QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapFromItemEPKS_RK6QRectF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem11mapFromItemEPKS_RK6QRectF(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectFromParent(const QRectF & rect);
impl /*struct*/ QGraphicsItem {
  pub fn mapRectFromParent<RetType, T: QGraphicsItem_mapRectFromParent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapRectFromParent(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapRectFromParent<RetType> {
  fn mapRectFromParent(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QRectF QGraphicsItem::mapRectFromParent(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromParent<QRectF> for (QRectF) {
  fn mapRectFromParent(self , rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem17mapRectFromParentERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem17mapRectFromParentERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setFocusProxy(QGraphicsItem * item);
impl /*struct*/ QGraphicsItem {
  pub fn setFocusProxy<RetType, T: QGraphicsItem_setFocusProxy<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFocusProxy(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setFocusProxy<RetType> {
  fn setFocusProxy(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setFocusProxy(QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsItem_setFocusProxy<()> for (QGraphicsItem) {
  fn setFocusProxy(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem13setFocusProxyEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem13setFocusProxyEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::acceptDrops();
impl /*struct*/ QGraphicsItem {
  pub fn acceptDrops<RetType, T: QGraphicsItem_acceptDrops<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.acceptDrops(self);
    // return 1;
  }
}

pub trait QGraphicsItem_acceptDrops<RetType> {
  fn acceptDrops(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::acceptDrops();
impl<'a> /*trait*/ QGraphicsItem_acceptDrops<i8> for () {
  fn acceptDrops(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11acceptDropsEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem11acceptDropsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapToParent(const QPointF & point);
impl<'a> /*trait*/ QGraphicsItem_mapToParent<QPointF> for (QPointF) {
  fn mapToParent(self , rsthis: &mut QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapToParentERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem11mapToParentERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectFromScene(const QRectF & rect);
impl /*struct*/ QGraphicsItem {
  pub fn mapRectFromScene<RetType, T: QGraphicsItem_mapRectFromScene<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapRectFromScene(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapRectFromScene<RetType> {
  fn mapRectFromScene(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QRectF QGraphicsItem::mapRectFromScene(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromScene<QRectF> for (QRectF) {
  fn mapRectFromScene(self , rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem16mapRectFromSceneERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem16mapRectFromSceneERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsItem::focusScopeItem();
impl /*struct*/ QGraphicsItem {
  pub fn focusScopeItem<RetType, T: QGraphicsItem_focusScopeItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.focusScopeItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_focusScopeItem<RetType> {
  fn focusScopeItem(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsItem * QGraphicsItem::focusScopeItem();
impl<'a> /*trait*/ QGraphicsItem_focusScopeItem<()> for () {
  fn focusScopeItem(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14focusScopeItemEv()};
     unsafe {_ZNK13QGraphicsItem14focusScopeItemEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::removeSceneEventFilter(QGraphicsItem * filterItem);
impl /*struct*/ QGraphicsItem {
  pub fn removeSceneEventFilter<RetType, T: QGraphicsItem_removeSceneEventFilter<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removeSceneEventFilter(self);
    // return 1;
  }
}

pub trait QGraphicsItem_removeSceneEventFilter<RetType> {
  fn removeSceneEventFilter(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::removeSceneEventFilter(QGraphicsItem * filterItem);
impl<'a> /*trait*/ QGraphicsItem_removeSceneEventFilter<()> for (QGraphicsItem) {
  fn removeSceneEventFilter(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem22removeSceneEventFilterEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem22removeSceneEventFilterEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsItem::focusProxy();
impl /*struct*/ QGraphicsItem {
  pub fn focusProxy<RetType, T: QGraphicsItem_focusProxy<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.focusProxy(self);
    // return 1;
  }
}

pub trait QGraphicsItem_focusProxy<RetType> {
  fn focusProxy(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsItem * QGraphicsItem::focusProxy();
impl<'a> /*trait*/ QGraphicsItem_focusProxy<()> for () {
  fn focusProxy(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10focusProxyEv()};
     unsafe {_ZNK13QGraphicsItem10focusProxyEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapToItem(const QGraphicsItem * item, const QPointF & point);
impl<'a> /*trait*/ QGraphicsItem_mapToItem<QPointF> for (QGraphicsItem, QPointF) {
  fn mapToItem(self , rsthis: &mut QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9mapToItemEPKS_RK7QPointF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem9mapToItemEPKS_RK7QPointF(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::sceneBoundingRect();
impl /*struct*/ QGraphicsItem {
  pub fn sceneBoundingRect<RetType, T: QGraphicsItem_sceneBoundingRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sceneBoundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsItem_sceneBoundingRect<RetType> {
  fn sceneBoundingRect(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QRectF QGraphicsItem::sceneBoundingRect();
impl<'a> /*trait*/ QGraphicsItem_sceneBoundingRect<QRectF> for () {
  fn sceneBoundingRect(self , rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem17sceneBoundingRectEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem17sceneBoundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::~QGraphicsItem();
impl /*struct*/ QGraphicsItem {
  pub fn FreeQGraphicsItem<RetType, T: QGraphicsItem_FreeQGraphicsItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_FreeQGraphicsItem<RetType> {
  fn FreeQGraphicsItem(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::~QGraphicsItem();
impl<'a> /*trait*/ QGraphicsItem_FreeQGraphicsItem<()> for () {
  fn FreeQGraphicsItem(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItemD0Ev()};
     unsafe {_ZN13QGraphicsItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setX(qreal x);
impl /*struct*/ QGraphicsItem {
  pub fn setX<RetType, T: QGraphicsItem_setX<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setX(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setX<RetType> {
  fn setX(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setX(qreal x);
impl<'a> /*trait*/ QGraphicsItem_setX<()> for (f64) {
  fn setX(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem4setXEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QGraphicsItem4setXEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::update(qreal x, qreal y, qreal width, qreal height);
impl /*struct*/ QGraphicsItem {
  pub fn update<RetType, T: QGraphicsItem_update<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.update(self);
    // return 1;
  }
}

pub trait QGraphicsItem_update<RetType> {
  fn update(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::update(qreal x, qreal y, qreal width, qreal height);
impl<'a> /*trait*/ QGraphicsItem_update<()> for (f64, f64, f64, f64) {
  fn update(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6updateEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN13QGraphicsItem6updateEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setSelected(bool selected);
impl /*struct*/ QGraphicsItem {
  pub fn setSelected<RetType, T: QGraphicsItem_setSelected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSelected(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setSelected<RetType> {
  fn setSelected(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setSelected(bool selected);
impl<'a> /*trait*/ QGraphicsItem_setSelected<()> for (i8) {
  fn setSelected(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11setSelectedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QGraphicsItem11setSelectedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectToItem(const QGraphicsItem * item, const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectToItem<QRectF> for (QGraphicsItem, QRectF) {
  fn mapRectToItem(self , rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapRectToItemEPKS_RK6QRectF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem13mapRectToItemEPKS_RK6QRectF(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::stackBefore(const QGraphicsItem * sibling);
impl /*struct*/ QGraphicsItem {
  pub fn stackBefore<RetType, T: QGraphicsItem_stackBefore<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.stackBefore(self);
    // return 1;
  }
}

pub trait QGraphicsItem_stackBefore<RetType> {
  fn stackBefore(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::stackBefore(const QGraphicsItem * sibling);
impl<'a> /*trait*/ QGraphicsItem_stackBefore<()> for (QGraphicsItem) {
  fn stackBefore(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11stackBeforeEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem11stackBeforeEPKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapFromItem(const QGraphicsItem * item, qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsItem_mapFromItem<QPointF> for (QGraphicsItem, f64, f64) {
  fn mapFromItem(self , rsthis: &mut QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapFromItemEPKS_dd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem11mapFromItemEPKS_dd(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::resetMatrix();
impl /*struct*/ QGraphicsItem {
  pub fn resetMatrix<RetType, T: QGraphicsItem_resetMatrix<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.resetMatrix(self);
    // return 1;
  }
}

pub trait QGraphicsItem_resetMatrix<RetType> {
  fn resetMatrix(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::resetMatrix();
impl<'a> /*trait*/ QGraphicsItem_resetMatrix<()> for () {
  fn resetMatrix(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11resetMatrixEv()};
     unsafe {_ZN13QGraphicsItem11resetMatrixEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsItem::opaqueArea();
impl /*struct*/ QGraphicsItem {
  pub fn opaqueArea<RetType, T: QGraphicsItem_opaqueArea<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.opaqueArea(self);
    // return 1;
  }
}

pub trait QGraphicsItem_opaqueArea<RetType> {
  fn opaqueArea(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsItem_opaqueArea<QPainterPath> for () {
  fn opaqueArea(self , rsthis: &mut QGraphicsItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10opaqueAreaEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::unsetCursor();
impl /*struct*/ QGraphicsItem {
  pub fn unsetCursor<RetType, T: QGraphicsItem_unsetCursor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unsetCursor(self);
    // return 1;
  }
}

pub trait QGraphicsItem_unsetCursor<RetType> {
  fn unsetCursor(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::unsetCursor();
impl<'a> /*trait*/ QGraphicsItem_unsetCursor<()> for () {
  fn unsetCursor(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11unsetCursorEv()};
     unsafe {_ZN13QGraphicsItem11unsetCursorEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapFromParent(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsItem_mapFromParent<QPointF> for (f64, f64) {
  fn mapFromParent(self , rsthis: &mut QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapFromParentEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem13mapFromParentEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectToScene(const QRectF & rect);
impl /*struct*/ QGraphicsItem {
  pub fn mapRectToScene<RetType, T: QGraphicsItem_mapRectToScene<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapRectToScene(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapRectToScene<RetType> {
  fn mapRectToScene(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QRectF QGraphicsItem::mapRectToScene(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectToScene<QRectF> for (QRectF) {
  fn mapRectToScene(self , rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14mapRectToSceneERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem14mapRectToSceneERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectFromItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
impl /*struct*/ QGraphicsItem {
  pub fn mapRectFromItem<RetType, T: QGraphicsItem_mapRectFromItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapRectFromItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapRectFromItem<RetType> {
  fn mapRectFromItem(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QRectF QGraphicsItem::mapRectFromItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromItem<QRectF> for (QGraphicsItem, f64, f64, f64, f64) {
  fn mapRectFromItem(self , rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15mapRectFromItemEPKS_dddd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem15mapRectFromItemEPKS_dddd(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QGraphicsItem::scale();
impl /*struct*/ QGraphicsItem {
  pub fn scale<RetType, T: QGraphicsItem_scale<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.scale(self);
    // return 1;
  }
}

pub trait QGraphicsItem_scale<RetType> {
  fn scale(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  qreal QGraphicsItem::scale();
impl<'a> /*trait*/ QGraphicsItem_scale<f64> for () {
  fn scale(self , rsthis: &mut QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem5scaleEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem5scaleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setBoundingRegionGranularity(qreal granularity);
impl /*struct*/ QGraphicsItem {
  pub fn setBoundingRegionGranularity<RetType, T: QGraphicsItem_setBoundingRegionGranularity<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBoundingRegionGranularity(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setBoundingRegionGranularity<RetType> {
  fn setBoundingRegionGranularity(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setBoundingRegionGranularity(qreal granularity);
impl<'a> /*trait*/ QGraphicsItem_setBoundingRegionGranularity<()> for (f64) {
  fn setBoundingRegionGranularity(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem28setBoundingRegionGranularityEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QGraphicsItem28setBoundingRegionGranularityEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setAcceptDrops(bool on);
impl /*struct*/ QGraphicsItem {
  pub fn setAcceptDrops<RetType, T: QGraphicsItem_setAcceptDrops<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAcceptDrops(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setAcceptDrops<RetType> {
  fn setAcceptDrops(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setAcceptDrops(bool on);
impl<'a> /*trait*/ QGraphicsItem_setAcceptDrops<()> for (i8) {
  fn setAcceptDrops(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem14setAcceptDropsEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QGraphicsItem14setAcceptDropsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapFromScene(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapFromScene<QPolygonF> for (f64, f64, f64, f64) {
  fn mapFromScene(self , rsthis: &mut QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12mapFromSceneEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem12mapFromSceneEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::ungrabKeyboard();
impl /*struct*/ QGraphicsItem {
  pub fn ungrabKeyboard<RetType, T: QGraphicsItem_ungrabKeyboard<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.ungrabKeyboard(self);
    // return 1;
  }
}

pub trait QGraphicsItem_ungrabKeyboard<RetType> {
  fn ungrabKeyboard(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::ungrabKeyboard();
impl<'a> /*trait*/ QGraphicsItem_ungrabKeyboard<()> for () {
  fn ungrabKeyboard(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem14ungrabKeyboardEv()};
     unsafe {_ZN13QGraphicsItem14ungrabKeyboardEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setEnabled(bool enabled);
impl /*struct*/ QGraphicsItem {
  pub fn setEnabled<RetType, T: QGraphicsItem_setEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setEnabled(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setEnabled<RetType> {
  fn setEnabled(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setEnabled(bool enabled);
impl<'a> /*trait*/ QGraphicsItem_setEnabled<()> for (i8) {
  fn setEnabled(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem10setEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QGraphicsItem10setEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QGraphicsEffect * QGraphicsItem::graphicsEffect();
impl /*struct*/ QGraphicsItem {
  pub fn graphicsEffect<RetType, T: QGraphicsItem_graphicsEffect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.graphicsEffect(self);
    // return 1;
  }
}

pub trait QGraphicsItem_graphicsEffect<RetType> {
  fn graphicsEffect(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsEffect * QGraphicsItem::graphicsEffect();
impl<'a> /*trait*/ QGraphicsItem_graphicsEffect<()> for () {
  fn graphicsEffect(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14graphicsEffectEv()};
     unsafe {_ZNK13QGraphicsItem14graphicsEffectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::acceptHoverEvents();
impl /*struct*/ QGraphicsItem {
  pub fn acceptHoverEvents<RetType, T: QGraphicsItem_acceptHoverEvents<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.acceptHoverEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_acceptHoverEvents<RetType> {
  fn acceptHoverEvents(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::acceptHoverEvents();
impl<'a> /*trait*/ QGraphicsItem_acceptHoverEvents<i8> for () {
  fn acceptHoverEvents(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem17acceptHoverEventsEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem17acceptHoverEventsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QGraphicsWidget * QGraphicsItem::topLevelWidget();
impl /*struct*/ QGraphicsItem {
  pub fn topLevelWidget<RetType, T: QGraphicsItem_topLevelWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.topLevelWidget(self);
    // return 1;
  }
}

pub trait QGraphicsItem_topLevelWidget<RetType> {
  fn topLevelWidget(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsWidget * QGraphicsItem::topLevelWidget();
impl<'a> /*trait*/ QGraphicsItem_topLevelWidget<()> for () {
  fn topLevelWidget(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14topLevelWidgetEv()};
     unsafe {_ZNK13QGraphicsItem14topLevelWidgetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QGraphicsTransform *> QGraphicsItem::transformations();
impl /*struct*/ QGraphicsItem {
  pub fn transformations<RetType, T: QGraphicsItem_transformations<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.transformations(self);
    // return 1;
  }
}

pub trait QGraphicsItem_transformations<RetType> {
  fn transformations(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QList<QGraphicsTransform *> QGraphicsItem::transformations();
impl<'a> /*trait*/ QGraphicsItem_transformations<()> for () {
  fn transformations(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15transformationsEv()};
     unsafe {_ZNK13QGraphicsItem15transformationsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapToScene(qreal x, qreal y, qreal w, qreal h);
impl /*struct*/ QGraphicsItem {
  pub fn mapToScene<RetType, T: QGraphicsItem_mapToScene<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapToScene(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapToScene<RetType> {
  fn mapToScene(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QPolygonF QGraphicsItem::mapToScene(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapToScene<QPolygonF> for (f64, f64, f64, f64) {
  fn mapToScene(self , rsthis: &mut QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10mapToSceneEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem10mapToSceneEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapToScene(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsItem_mapToScene<QPointF> for (f64, f64) {
  fn mapToScene(self , rsthis: &mut QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10mapToSceneEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem10mapToSceneEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectFromScene(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromScene<QRectF> for (f64, f64, f64, f64) {
  fn mapRectFromScene(self , rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem16mapRectFromSceneEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem16mapRectFromSceneEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::advance(int phase);
impl /*struct*/ QGraphicsItem {
  pub fn advance<RetType, T: QGraphicsItem_advance<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.advance(self);
    // return 1;
  }
}

pub trait QGraphicsItem_advance<RetType> {
  fn advance(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::advance(int phase);
impl<'a> /*trait*/ QGraphicsItem_advance<()> for (i32) {
  fn advance(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem7advanceEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QGraphicsItem7advanceEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QMatrix QGraphicsItem::sceneMatrix();
impl /*struct*/ QGraphicsItem {
  pub fn sceneMatrix<RetType, T: QGraphicsItem_sceneMatrix<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sceneMatrix(self);
    // return 1;
  }
}

pub trait QGraphicsItem_sceneMatrix<RetType> {
  fn sceneMatrix(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QMatrix QGraphicsItem::sceneMatrix();
impl<'a> /*trait*/ QGraphicsItem_sceneMatrix<QMatrix> for () {
  fn sceneMatrix(self , rsthis: &mut QGraphicsItem) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11sceneMatrixEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem11sceneMatrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setFiltersChildEvents(bool enabled);
impl /*struct*/ QGraphicsItem {
  pub fn setFiltersChildEvents<RetType, T: QGraphicsItem_setFiltersChildEvents<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFiltersChildEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setFiltersChildEvents<RetType> {
  fn setFiltersChildEvents(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setFiltersChildEvents(bool enabled);
impl<'a> /*trait*/ QGraphicsItem_setFiltersChildEvents<()> for (i8) {
  fn setFiltersChildEvents(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem21setFiltersChildEventsEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QGraphicsItem21setFiltersChildEventsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapToScene(const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsItem_mapToScene<QPolygonF> for (QPolygonF) {
  fn mapToScene(self , rsthis: &mut QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10mapToSceneERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem10mapToSceneERK9QPolygonF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QTransform QGraphicsItem::itemTransform(const QGraphicsItem * other, bool * ok);
impl /*struct*/ QGraphicsItem {
  pub fn itemTransform<RetType, T: QGraphicsItem_itemTransform<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemTransform(self);
    // return 1;
  }
}

pub trait QGraphicsItem_itemTransform<RetType> {
  fn itemTransform(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QTransform QGraphicsItem::itemTransform(const QGraphicsItem * other, bool * ok);
impl<'a> /*trait*/ QGraphicsItem_itemTransform<QTransform> for (QGraphicsItem, &'a mut Vec<i8>) {
  fn itemTransform(self , rsthis: &mut QGraphicsItem) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13itemTransformEPKS_Pb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK13QGraphicsItem13itemTransformEPKS_Pb(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setTransformOriginPoint(qreal ax, qreal ay);
impl<'a> /*trait*/ QGraphicsItem_setTransformOriginPoint<()> for (f64, f64) {
  fn setTransformOriginPoint(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem23setTransformOriginPointEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN13QGraphicsItem23setTransformOriginPointEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::moveBy(qreal dx, qreal dy);
impl /*struct*/ QGraphicsItem {
  pub fn moveBy<RetType, T: QGraphicsItem_moveBy<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.moveBy(self);
    // return 1;
  }
}

pub trait QGraphicsItem_moveBy<RetType> {
  fn moveBy(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::moveBy(qreal dx, qreal dy);
impl<'a> /*trait*/ QGraphicsItem_moveBy<()> for (f64, f64) {
  fn moveBy(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6moveByEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN13QGraphicsItem6moveByEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapFromScene(const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsItem_mapFromScene<QPolygonF> for (QPolygonF) {
  fn mapFromScene(self , rsthis: &mut QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12mapFromSceneERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem12mapFromSceneERK9QPolygonF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QGraphicsItemGroup * QGraphicsItem::group();
impl /*struct*/ QGraphicsItem {
  pub fn group<RetType, T: QGraphicsItem_group<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.group(self);
    // return 1;
  }
}

pub trait QGraphicsItem_group<RetType> {
  fn group(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsItemGroup * QGraphicsItem::group();
impl<'a> /*trait*/ QGraphicsItem_group<()> for () {
  fn group(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem5groupEv()};
     unsafe {_ZNK13QGraphicsItem5groupEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsItem::shape();
impl /*struct*/ QGraphicsItem {
  pub fn shape<RetType, T: QGraphicsItem_shape<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.shape(self);
    // return 1;
  }
}

pub trait QGraphicsItem_shape<RetType> {
  fn shape(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsItem::shape();
impl<'a> /*trait*/ QGraphicsItem_shape<QPainterPath> for () {
  fn shape(self , rsthis: &mut QGraphicsItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem5shapeEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem5shapeEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapFromScene(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsItem_mapFromScene<QPointF> for (f64, f64) {
  fn mapFromScene(self , rsthis: &mut QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12mapFromSceneEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem12mapFromSceneEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::scroll(qreal dx, qreal dy, const QRectF & rect);
impl /*struct*/ QGraphicsItem {
  pub fn scroll<RetType, T: QGraphicsItem_scroll<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.scroll(self);
    // return 1;
  }
}

pub trait QGraphicsItem_scroll<RetType> {
  fn scroll(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::scroll(qreal dx, qreal dy, const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_scroll<()> for (f64, f64, QRectF) {
  fn scroll(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6scrollEddRK6QRectF()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem6scrollEddRK6QRectF(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isObscuredBy(const QGraphicsItem * item);
impl /*struct*/ QGraphicsItem {
  pub fn isObscuredBy<RetType, T: QGraphicsItem_isObscuredBy<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isObscuredBy(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isObscuredBy<RetType> {
  fn isObscuredBy(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsItem_isObscuredBy<i8> for (QGraphicsItem) {
  fn isObscuredBy(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12isObscuredByEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem12isObscuredByEPKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapFromParent(const QPointF & point);
impl<'a> /*trait*/ QGraphicsItem_mapFromParent<QPointF> for (QPointF) {
  fn mapFromParent(self , rsthis: &mut QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapFromParentERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem13mapFromParentERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setData(int key, const QVariant & value);
impl /*struct*/ QGraphicsItem {
  pub fn setData<RetType, T: QGraphicsItem_setData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setData(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setData<RetType> {
  fn setData(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setData(int key, const QVariant & value);
impl<'a> /*trait*/ QGraphicsItem_setData<()> for (i32, QVariant) {
  fn setData(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem7setDataEiRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem7setDataEiRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsItem::commonAncestorItem(const QGraphicsItem * other);
impl /*struct*/ QGraphicsItem {
  pub fn commonAncestorItem<RetType, T: QGraphicsItem_commonAncestorItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.commonAncestorItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_commonAncestorItem<RetType> {
  fn commonAncestorItem(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsItem * QGraphicsItem::commonAncestorItem(const QGraphicsItem * other);
impl<'a> /*trait*/ QGraphicsItem_commonAncestorItem<()> for (QGraphicsItem) {
  fn commonAncestorItem(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem18commonAncestorItemEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK13QGraphicsItem18commonAncestorItemEPKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsItem::mapFromScene(const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsItem_mapFromScene<QPainterPath> for (QPainterPath) {
  fn mapFromScene(self , rsthis: &mut QGraphicsItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12mapFromSceneERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem12mapFromSceneERK12QPainterPath(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsItem::mapToScene(const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsItem_mapToScene<QPainterPath> for (QPainterPath) {
  fn mapToScene(self , rsthis: &mut QGraphicsItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10mapToSceneERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem10mapToSceneERK12QPainterPath(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapToParent(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapToParent<QPolygonF> for (f64, f64, f64, f64) {
  fn mapToParent(self , rsthis: &mut QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapToParentEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem11mapToParentEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setGroup(QGraphicsItemGroup * group);
impl /*struct*/ QGraphicsItem {
  pub fn setGroup<RetType, T: QGraphicsItem_setGroup<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setGroup(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setGroup<RetType> {
  fn setGroup(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setGroup(QGraphicsItemGroup * group);
impl<'a> /*trait*/ QGraphicsItem_setGroup<()> for (QGraphicsItemGroup) {
  fn setGroup(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem8setGroupEP18QGraphicsItemGroup()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem8setGroupEP18QGraphicsItemGroup(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectFromParent(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromParent<QRectF> for (f64, f64, f64, f64) {
  fn mapRectFromParent(self , rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem17mapRectFromParentEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem17mapRectFromParentEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::show();
impl /*struct*/ QGraphicsItem {
  pub fn show<RetType, T: QGraphicsItem_show<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.show(self);
    // return 1;
  }
}

pub trait QGraphicsItem_show<RetType> {
  fn show(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::show();
impl<'a> /*trait*/ QGraphicsItem_show<()> for () {
  fn show(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem4showEv()};
     unsafe {_ZN13QGraphicsItem4showEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectFromItem(const QGraphicsItem * item, const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromItem<QRectF> for (QGraphicsItem, QRectF) {
  fn mapRectFromItem(self , rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15mapRectFromItemEPKS_RK6QRectF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem15mapRectFromItemEPKS_RK6QRectF(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QGraphicsItem::y();
impl /*struct*/ QGraphicsItem {
  pub fn y<RetType, T: QGraphicsItem_y<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QGraphicsItem_y<RetType> {
  fn y(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  qreal QGraphicsItem::y();
impl<'a> /*trait*/ QGraphicsItem_y<()> for () {
  fn y(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem1yEv()};
     unsafe {_ZNK13QGraphicsItem1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapFromScene(const QPointF & point);
impl<'a> /*trait*/ QGraphicsItem_mapFromScene<QPointF> for (QPointF) {
  fn mapFromScene(self , rsthis: &mut QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12mapFromSceneERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem12mapFromSceneERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::hasFocus();
impl /*struct*/ QGraphicsItem {
  pub fn hasFocus<RetType, T: QGraphicsItem_hasFocus<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasFocus(self);
    // return 1;
  }
}

pub trait QGraphicsItem_hasFocus<RetType> {
  fn hasFocus(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::hasFocus();
impl<'a> /*trait*/ QGraphicsItem_hasFocus<i8> for () {
  fn hasFocus(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8hasFocusEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem8hasFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsItem::clipPath();
impl /*struct*/ QGraphicsItem {
  pub fn clipPath<RetType, T: QGraphicsItem_clipPath<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clipPath(self);
    // return 1;
  }
}

pub trait QGraphicsItem_clipPath<RetType> {
  fn clipPath(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsItem::clipPath();
impl<'a> /*trait*/ QGraphicsItem_clipPath<QPainterPath> for () {
  fn clipPath(self , rsthis: &mut QGraphicsItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8clipPathEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem8clipPathEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setPos(qreal x, qreal y);
impl /*struct*/ QGraphicsItem {
  pub fn setPos<RetType, T: QGraphicsItem_setPos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPos(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setPos<RetType> {
  fn setPos(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setPos(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsItem_setPos<()> for (f64, f64) {
  fn setPos(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6setPosEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN13QGraphicsItem6setPosEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isEnabled();
impl /*struct*/ QGraphicsItem {
  pub fn isEnabled<RetType, T: QGraphicsItem_isEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEnabled(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isEnabled<RetType> {
  fn isEnabled(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isEnabled();
impl<'a> /*trait*/ QGraphicsItem_isEnabled<i8> for () {
  fn isEnabled(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9isEnabledEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem9isEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::contains(const QPointF & point);
impl /*struct*/ QGraphicsItem {
  pub fn contains<RetType, T: QGraphicsItem_contains<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QGraphicsItem_contains<RetType> {
  fn contains(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsItem_contains<i8> for (QPointF) {
  fn contains(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isPanel();
impl /*struct*/ QGraphicsItem {
  pub fn isPanel<RetType, T: QGraphicsItem_isPanel<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isPanel(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isPanel<RetType> {
  fn isPanel(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isPanel();
impl<'a> /*trait*/ QGraphicsItem_isPanel<i8> for () {
  fn isPanel(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem7isPanelEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem7isPanelEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::filtersChildEvents();
impl /*struct*/ QGraphicsItem {
  pub fn filtersChildEvents<RetType, T: QGraphicsItem_filtersChildEvents<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.filtersChildEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_filtersChildEvents<RetType> {
  fn filtersChildEvents(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::filtersChildEvents();
impl<'a> /*trait*/ QGraphicsItem_filtersChildEvents<i8> for () {
  fn filtersChildEvents(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem18filtersChildEventsEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem18filtersChildEventsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::grabKeyboard();
impl /*struct*/ QGraphicsItem {
  pub fn grabKeyboard<RetType, T: QGraphicsItem_grabKeyboard<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.grabKeyboard(self);
    // return 1;
  }
}

pub trait QGraphicsItem_grabKeyboard<RetType> {
  fn grabKeyboard(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::grabKeyboard();
impl<'a> /*trait*/ QGraphicsItem_grabKeyboard<()> for () {
  fn grabKeyboard(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem12grabKeyboardEv()};
     unsafe {_ZN13QGraphicsItem12grabKeyboardEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsItem_mapFromItem<QPainterPath> for (QGraphicsItem, QPainterPath) {
  fn mapFromItem(self , rsthis: &mut QGraphicsItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapFromItemEPKS_RK12QPainterPath()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem11mapFromItemEPKS_RK12QPainterPath(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setActive(bool active);
impl /*struct*/ QGraphicsItem {
  pub fn setActive<RetType, T: QGraphicsItem_setActive<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setActive(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setActive<RetType> {
  fn setActive(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setActive(bool active);
impl<'a> /*trait*/ QGraphicsItem_setActive<()> for (i8) {
  fn setActive(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem9setActiveEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QGraphicsItem9setActiveEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QGraphicsObject * QGraphicsItem::toGraphicsObject();
impl /*struct*/ QGraphicsItem {
  pub fn toGraphicsObject<RetType, T: QGraphicsItem_toGraphicsObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toGraphicsObject(self);
    // return 1;
  }
}

pub trait QGraphicsItem_toGraphicsObject<RetType> {
  fn toGraphicsObject(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsObject * QGraphicsItem::toGraphicsObject();
impl<'a> /*trait*/ QGraphicsItem_toGraphicsObject<()> for () {
  fn toGraphicsObject(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem16toGraphicsObjectEv()};
     unsafe {_ZN13QGraphicsItem16toGraphicsObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsItem_mapFromItem<QPolygonF> for (QGraphicsItem, QPolygonF) {
  fn mapFromItem(self , rsthis: &mut QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapFromItemEPKS_RK9QPolygonF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem11mapFromItemEPKS_RK9QPolygonF(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setHandlesChildEvents(bool enabled);
impl /*struct*/ QGraphicsItem {
  pub fn setHandlesChildEvents<RetType, T: QGraphicsItem_setHandlesChildEvents<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHandlesChildEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setHandlesChildEvents<RetType> {
  fn setHandlesChildEvents(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setHandlesChildEvents(bool enabled);
impl<'a> /*trait*/ QGraphicsItem_setHandlesChildEvents<()> for (i8) {
  fn setHandlesChildEvents(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem21setHandlesChildEventsEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QGraphicsItem21setHandlesChildEventsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapFromParent(const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsItem_mapFromParent<QPolygonF> for (QPolygonF) {
  fn mapFromParent(self , rsthis: &mut QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapFromParentERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem13mapFromParentERK9QPolygonF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapToParent(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsItem_mapToParent<QPointF> for (f64, f64) {
  fn mapToParent(self , rsthis: &mut QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapToParentEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem11mapToParentEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setMatrix(const QMatrix & matrix, bool combine);
impl /*struct*/ QGraphicsItem {
  pub fn setMatrix<RetType, T: QGraphicsItem_setMatrix<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMatrix(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setMatrix<RetType> {
  fn setMatrix(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setMatrix(const QMatrix & matrix, bool combine);
impl<'a> /*trait*/ QGraphicsItem_setMatrix<()> for (QMatrix, i8) {
  fn setMatrix(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem9setMatrixERK7QMatrixb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {_ZN13QGraphicsItem9setMatrixERK7QMatrixb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::update(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_update<()> for (QRectF) {
  fn update(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6updateERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem6updateERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem * item, const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsItem_mapToItem<QPolygonF> for (QGraphicsItem, QPolygonF) {
  fn mapToItem(self , rsthis: &mut QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9mapToItemEPKS_RK9QPolygonF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem9mapToItemEPKS_RK9QPolygonF(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QTransform QGraphicsItem::transform();
impl /*struct*/ QGraphicsItem {
  pub fn transform<RetType, T: QGraphicsItem_transform<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.transform(self);
    // return 1;
  }
}

pub trait QGraphicsItem_transform<RetType> {
  fn transform(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QTransform QGraphicsItem::transform();
impl<'a> /*trait*/ QGraphicsItem_transform<QTransform> for () {
  fn transform(self , rsthis: &mut QGraphicsItem) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9transformEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem9transformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QGraphicsItem::data(int key);
impl /*struct*/ QGraphicsItem {
  pub fn data<RetType, T: QGraphicsItem_data<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QGraphicsItem_data<RetType> {
  fn data(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QVariant QGraphicsItem::data(int key);
impl<'a> /*trait*/ QGraphicsItem_data<QVariant> for (i32) {
  fn data(self , rsthis: &mut QGraphicsItem) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem4dataEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK13QGraphicsItem4dataEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::hide();
impl /*struct*/ QGraphicsItem {
  pub fn hide<RetType, T: QGraphicsItem_hide<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hide(self);
    // return 1;
  }
}

pub trait QGraphicsItem_hide<RetType> {
  fn hide(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::hide();
impl<'a> /*trait*/ QGraphicsItem_hide<()> for () {
  fn hide(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem4hideEv()};
     unsafe {_ZN13QGraphicsItem4hideEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isUnderMouse();
impl /*struct*/ QGraphicsItem {
  pub fn isUnderMouse<RetType, T: QGraphicsItem_isUnderMouse<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isUnderMouse(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isUnderMouse<RetType> {
  fn isUnderMouse(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isUnderMouse();
impl<'a> /*trait*/ QGraphicsItem_isUnderMouse<i8> for () {
  fn isUnderMouse(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12isUnderMouseEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem12isUnderMouseEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setAcceptTouchEvents(bool enabled);
impl /*struct*/ QGraphicsItem {
  pub fn setAcceptTouchEvents<RetType, T: QGraphicsItem_setAcceptTouchEvents<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAcceptTouchEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setAcceptTouchEvents<RetType> {
  fn setAcceptTouchEvents(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setAcceptTouchEvents(bool enabled);
impl<'a> /*trait*/ QGraphicsItem_setAcceptTouchEvents<()> for (i8) {
  fn setAcceptTouchEvents(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem20setAcceptTouchEventsEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QGraphicsItem20setAcceptTouchEventsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setAcceptHoverEvents(bool enabled);
impl /*struct*/ QGraphicsItem {
  pub fn setAcceptHoverEvents<RetType, T: QGraphicsItem_setAcceptHoverEvents<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAcceptHoverEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setAcceptHoverEvents<RetType> {
  fn setAcceptHoverEvents(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setAcceptHoverEvents(bool enabled);
impl<'a> /*trait*/ QGraphicsItem_setAcceptHoverEvents<()> for (i8) {
  fn setAcceptHoverEvents(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem20setAcceptHoverEventsEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QGraphicsItem20setAcceptHoverEventsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QList<QGraphicsItem *> QGraphicsItem::childItems();
impl /*struct*/ QGraphicsItem {
  pub fn childItems<RetType, T: QGraphicsItem_childItems<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.childItems(self);
    // return 1;
  }
}

pub trait QGraphicsItem_childItems<RetType> {
  fn childItems(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QList<QGraphicsItem *> QGraphicsItem::childItems();
impl<'a> /*trait*/ QGraphicsItem_childItems<()> for () {
  fn childItems(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10childItemsEv()};
     unsafe {_ZNK13QGraphicsItem10childItemsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isAncestorOf(const QGraphicsItem * child);
impl /*struct*/ QGraphicsItem {
  pub fn isAncestorOf<RetType, T: QGraphicsItem_isAncestorOf<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isAncestorOf(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isAncestorOf<RetType> {
  fn isAncestorOf(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isAncestorOf(const QGraphicsItem * child);
impl<'a> /*trait*/ QGraphicsItem_isAncestorOf<i8> for (QGraphicsItem) {
  fn isAncestorOf(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12isAncestorOfEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem12isAncestorOfEPKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qreal QGraphicsItem::opacity();
impl /*struct*/ QGraphicsItem {
  pub fn opacity<RetType, T: QGraphicsItem_opacity<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.opacity(self);
    // return 1;
  }
}

pub trait QGraphicsItem_opacity<RetType> {
  fn opacity(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  qreal QGraphicsItem::opacity();
impl<'a> /*trait*/ QGraphicsItem_opacity<f64> for () {
  fn opacity(self , rsthis: &mut QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem7opacityEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem7opacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isVisibleTo(const QGraphicsItem * parent);
impl /*struct*/ QGraphicsItem {
  pub fn isVisibleTo<RetType, T: QGraphicsItem_isVisibleTo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isVisibleTo(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isVisibleTo<RetType> {
  fn isVisibleTo(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isVisibleTo(const QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsItem_isVisibleTo<i8> for (QGraphicsItem) {
  fn isVisibleTo(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11isVisibleToEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem11isVisibleToEPKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QGraphicsItem::toolTip();
impl /*struct*/ QGraphicsItem {
  pub fn toolTip<RetType, T: QGraphicsItem_toolTip<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toolTip(self);
    // return 1;
  }
}

pub trait QGraphicsItem_toolTip<RetType> {
  fn toolTip(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QString QGraphicsItem::toolTip();
impl<'a> /*trait*/ QGraphicsItem_toolTip<QString> for () {
  fn toolTip(self , rsthis: &mut QGraphicsItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem7toolTipEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem7toolTipEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QCursor QGraphicsItem::cursor();
impl /*struct*/ QGraphicsItem {
  pub fn cursor<RetType, T: QGraphicsItem_cursor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cursor(self);
    // return 1;
  }
}

pub trait QGraphicsItem_cursor<RetType> {
  fn cursor(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QCursor QGraphicsItem::cursor();
impl<'a> /*trait*/ QGraphicsItem_cursor<QCursor> for () {
  fn cursor(self , rsthis: &mut QGraphicsItem) -> QCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem6cursorEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem6cursorEv(rsthis.qclsinst)};
    let mut ret1 = QCursor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapToScene(const QPointF & point);
impl<'a> /*trait*/ QGraphicsItem_mapToScene<QPointF> for (QPointF) {
  fn mapToScene(self , rsthis: &mut QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10mapToSceneERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem10mapToSceneERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QGraphicsItem::zValue();
impl /*struct*/ QGraphicsItem {
  pub fn zValue<RetType, T: QGraphicsItem_zValue<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.zValue(self);
    // return 1;
  }
}

pub trait QGraphicsItem_zValue<RetType> {
  fn zValue(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  qreal QGraphicsItem::zValue();
impl<'a> /*trait*/ QGraphicsItem_zValue<f64> for () {
  fn zValue(self , rsthis: &mut QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem6zValueEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem6zValueEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QMatrix QGraphicsItem::matrix();
impl /*struct*/ QGraphicsItem {
  pub fn matrix<RetType, T: QGraphicsItem_matrix<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.matrix(self);
    // return 1;
  }
}

pub trait QGraphicsItem_matrix<RetType> {
  fn matrix(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QMatrix QGraphicsItem::matrix();
impl<'a> /*trait*/ QGraphicsItem_matrix<QMatrix> for () {
  fn matrix(self , rsthis: &mut QGraphicsItem) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem6matrixEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem6matrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectToScene(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectToScene<QRectF> for (f64, f64, f64, f64) {
  fn mapRectToScene(self , rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14mapRectToSceneEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem14mapRectToSceneEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsItem_setPos<()> for (QPointF) {
  fn setPos(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6setPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem6setPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsItem::panel();
impl /*struct*/ QGraphicsItem {
  pub fn panel<RetType, T: QGraphicsItem_panel<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.panel(self);
    // return 1;
  }
}

pub trait QGraphicsItem_panel<RetType> {
  fn panel(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsItem * QGraphicsItem::panel();
impl<'a> /*trait*/ QGraphicsItem_panel<()> for () {
  fn panel(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem5panelEv()};
     unsafe {_ZNK13QGraphicsItem5panelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isClipped();
impl /*struct*/ QGraphicsItem {
  pub fn isClipped<RetType, T: QGraphicsItem_isClipped<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isClipped(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isClipped<RetType> {
  fn isClipped(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isClipped();
impl<'a> /*trait*/ QGraphicsItem_isClipped<i8> for () {
  fn isClipped(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9isClippedEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem9isClippedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsItem::topLevelItem();
impl /*struct*/ QGraphicsItem {
  pub fn topLevelItem<RetType, T: QGraphicsItem_topLevelItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.topLevelItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_topLevelItem<RetType> {
  fn topLevelItem(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsItem * QGraphicsItem::topLevelItem();
impl<'a> /*trait*/ QGraphicsItem_topLevelItem<()> for () {
  fn topLevelItem(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12topLevelItemEv()};
     unsafe {_ZNK13QGraphicsItem12topLevelItemEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapToScene(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapToScene<QPolygonF> for (QRectF) {
  fn mapToScene(self , rsthis: &mut QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10mapToSceneERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem10mapToSceneERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setScale(qreal scale);
impl /*struct*/ QGraphicsItem {
  pub fn setScale<RetType, T: QGraphicsItem_setScale<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setScale(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setScale<RetType> {
  fn setScale(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setScale(qreal scale);
impl<'a> /*trait*/ QGraphicsItem_setScale<()> for (f64) {
  fn setScale(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem8setScaleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QGraphicsItem8setScaleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setCursor(const QCursor & cursor);
impl /*struct*/ QGraphicsItem {
  pub fn setCursor<RetType, T: QGraphicsItem_setCursor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCursor(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setCursor<RetType> {
  fn setCursor(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setCursor(const QCursor & cursor);
impl<'a> /*trait*/ QGraphicsItem_setCursor<()> for (QCursor) {
  fn setCursor(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem9setCursorERK7QCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem9setCursorERK7QCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isVisible();
impl /*struct*/ QGraphicsItem {
  pub fn isVisible<RetType, T: QGraphicsItem_isVisible<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isVisible(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isVisible<RetType> {
  fn isVisible(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isVisible();
impl<'a> /*trait*/ QGraphicsItem_isVisible<i8> for () {
  fn isVisible(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9isVisibleEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem9isVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::pos();
impl /*struct*/ QGraphicsItem {
  pub fn pos<RetType, T: QGraphicsItem_pos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QGraphicsItem_pos<RetType> {
  fn pos(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QPointF QGraphicsItem::pos();
impl<'a> /*trait*/ QGraphicsItem_pos<QPointF> for () {
  fn pos(self , rsthis: &mut QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem3posEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem3posEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isBlockedByModalPanel(QGraphicsItem ** blockingPanel);
impl /*struct*/ QGraphicsItem {
  pub fn isBlockedByModalPanel<RetType, T: QGraphicsItem_isBlockedByModalPanel<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isBlockedByModalPanel(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isBlockedByModalPanel<RetType> {
  fn isBlockedByModalPanel(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isBlockedByModalPanel(QGraphicsItem ** blockingPanel);
impl<'a> /*trait*/ QGraphicsItem_isBlockedByModalPanel<i8> for (QGraphicsItem) {
  fn isBlockedByModalPanel(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem21isBlockedByModalPanelEPPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem21isBlockedByModalPanelEPPS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qreal QGraphicsItem::effectiveOpacity();
impl /*struct*/ QGraphicsItem {
  pub fn effectiveOpacity<RetType, T: QGraphicsItem_effectiveOpacity<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.effectiveOpacity(self);
    // return 1;
  }
}

pub trait QGraphicsItem_effectiveOpacity<RetType> {
  fn effectiveOpacity(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  qreal QGraphicsItem::effectiveOpacity();
impl<'a> /*trait*/ QGraphicsItem_effectiveOpacity<f64> for () {
  fn effectiveOpacity(self , rsthis: &mut QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem16effectiveOpacityEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem16effectiveOpacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::ensureVisible(const QRectF & rect, int xmargin, int ymargin);
impl<'a> /*trait*/ QGraphicsItem_ensureVisible<()> for (QRectF, i32, i32) {
  fn ensureVisible(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem13ensureVisibleERK6QRectFii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN13QGraphicsItem13ensureVisibleERK6QRectFii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsItem::boundingRegionGranularity();
impl /*struct*/ QGraphicsItem {
  pub fn boundingRegionGranularity<RetType, T: QGraphicsItem_boundingRegionGranularity<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.boundingRegionGranularity(self);
    // return 1;
  }
}

pub trait QGraphicsItem_boundingRegionGranularity<RetType> {
  fn boundingRegionGranularity(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  qreal QGraphicsItem::boundingRegionGranularity();
impl<'a> /*trait*/ QGraphicsItem_boundingRegionGranularity<f64> for () {
  fn boundingRegionGranularity(self , rsthis: &mut QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem25boundingRegionGranularityEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem25boundingRegionGranularityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QGraphicsItem::x();
impl /*struct*/ QGraphicsItem {
  pub fn x<RetType, T: QGraphicsItem_x<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QGraphicsItem_x<RetType> {
  fn x(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  qreal QGraphicsItem::x();
impl<'a> /*trait*/ QGraphicsItem_x<()> for () {
  fn x(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem1xEv()};
     unsafe {_ZNK13QGraphicsItem1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::grabMouse();
impl /*struct*/ QGraphicsItem {
  pub fn grabMouse<RetType, T: QGraphicsItem_grabMouse<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.grabMouse(self);
    // return 1;
  }
}

pub trait QGraphicsItem_grabMouse<RetType> {
  fn grabMouse(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::grabMouse();
impl<'a> /*trait*/ QGraphicsItem_grabMouse<()> for () {
  fn grabMouse(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem9grabMouseEv()};
     unsafe {_ZN13QGraphicsItem9grabMouseEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setVisible(bool visible);
impl /*struct*/ QGraphicsItem {
  pub fn setVisible<RetType, T: QGraphicsItem_setVisible<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setVisible<RetType> {
  fn setVisible(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setVisible(bool visible);
impl<'a> /*trait*/ QGraphicsItem_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QGraphicsItem10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setRotation(qreal angle);
impl /*struct*/ QGraphicsItem {
  pub fn setRotation<RetType, T: QGraphicsItem_setRotation<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRotation(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setRotation<RetType> {
  fn setRotation(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setRotation(qreal angle);
impl<'a> /*trait*/ QGraphicsItem_setRotation<()> for (f64) {
  fn setRotation(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11setRotationEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QGraphicsItem11setRotationEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTransform QGraphicsItem::deviceTransform(const QTransform & viewportTransform);
impl /*struct*/ QGraphicsItem {
  pub fn deviceTransform<RetType, T: QGraphicsItem_deviceTransform<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.deviceTransform(self);
    // return 1;
  }
}

pub trait QGraphicsItem_deviceTransform<RetType> {
  fn deviceTransform(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  QTransform QGraphicsItem::deviceTransform(const QTransform & viewportTransform);
impl<'a> /*trait*/ QGraphicsItem_deviceTransform<QTransform> for (QTransform) {
  fn deviceTransform(self , rsthis: &mut QGraphicsItem) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15deviceTransformERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem15deviceTransformERK10QTransform(rsthis.qclsinst, arg0)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::acceptTouchEvents();
impl /*struct*/ QGraphicsItem {
  pub fn acceptTouchEvents<RetType, T: QGraphicsItem_acceptTouchEvents<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.acceptTouchEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_acceptTouchEvents<RetType> {
  fn acceptTouchEvents(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::acceptTouchEvents();
impl<'a> /*trait*/ QGraphicsItem_acceptTouchEvents<i8> for () {
  fn acceptTouchEvents(self , rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem17acceptTouchEventsEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem17acceptTouchEventsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setTransform(const QTransform & matrix, bool combine);
impl /*struct*/ QGraphicsItem {
  pub fn setTransform<RetType, T: QGraphicsItem_setTransform<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTransform(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setTransform<RetType> {
  fn setTransform(self , rsthis: &mut QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setTransform(const QTransform & matrix, bool combine);
impl<'a> /*trait*/ QGraphicsItem_setTransform<()> for (QTransform, i8) {
  fn setTransform(self , rsthis: &mut QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem12setTransformERK10QTransformb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {_ZN13QGraphicsItem12setTransformERK10QTransformb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapToParent(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapToParent<QPolygonF> for (QRectF) {
  fn mapToParent(self , rsthis: &mut QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapToParentERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem11mapToParentERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}
