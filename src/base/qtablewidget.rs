// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtablewidgetitem::QTableWidgetItem;
use super::qstringlist::QStringList;
use super::qrect::QRect;
use super::qwidget::QWidget;
use super::qtablewidgetselectionrange::QTableWidgetSelectionRange;
use super::qpoint::QPoint;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QTableWidget::itemActivated(QTableWidgetItem * item);
  fn _ZN12QTableWidget13itemActivatedEP16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidget::setColumnCount(int columns);
  fn _ZN12QTableWidget14setColumnCountEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTableWidget::FreeQTableWidget();
  fn _ZN12QTableWidgetD0Ev(qthis: *mut c_void) ;
  // proto:  void QTableWidget::itemDoubleClicked(QTableWidgetItem * item);
  fn _ZN12QTableWidget17itemDoubleClickedEP16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidget::cellChanged(int row, int column);
  fn _ZN12QTableWidget11cellChangedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  QList<QTableWidgetItem *> QTableWidget::selectedItems();
  fn _ZNK12QTableWidget13selectedItemsEv(qthis: *mut c_void) ;
  // proto:  bool QTableWidget::isSortingEnabled();
  fn _ZNK12QTableWidget16isSortingEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  const QMetaObject * QTableWidget::metaObject();
  fn _ZNK12QTableWidget10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QTableWidget::NewQTableWidget(const QTableWidget & );
  fn _ZN12QTableWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidget::closePersistentEditor(QTableWidgetItem * item);
  fn _ZN12QTableWidget21closePersistentEditorEP16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidget::setHorizontalHeaderLabels(const QStringList & labels);
  fn _ZN12QTableWidget25setHorizontalHeaderLabelsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidget::itemSelectionChanged();
  fn _ZN12QTableWidget20itemSelectionChangedEv(qthis: *mut c_void) ;
  // proto:  void QTableWidget::setItemSelected(const QTableWidgetItem * item, bool select);
  fn _ZN12QTableWidget15setItemSelectedEPK16QTableWidgetItemb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
  // proto:  QTableWidgetItem * QTableWidget::takeItem(int row, int column);
  fn _ZN12QTableWidget8takeItemEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QTableWidget::itemChanged(QTableWidgetItem * item);
  fn _ZN12QTableWidget11itemChangedEP16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidget::removeCellWidget(int row, int column);
  fn _ZN12QTableWidget16removeCellWidgetEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QTableWidget::setVerticalHeaderItem(int row, QTableWidgetItem * item);
  fn _ZN12QTableWidget21setVerticalHeaderItemEiP16QTableWidgetItem(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QTableWidget::cellClicked(int row, int column);
  fn _ZN12QTableWidget11cellClickedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  QRect QTableWidget::visualItemRect(const QTableWidgetItem * item);
  fn _ZNK12QTableWidget14visualItemRectEPK16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QTableWidgetItem * QTableWidget::currentItem();
  fn _ZNK12QTableWidget11currentItemEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTableWidget::row(const QTableWidgetItem * item);
  fn _ZNK12QTableWidget3rowEPK16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QTableWidget::removeRow(int row);
  fn _ZN12QTableWidget9removeRowEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTableWidget::setItemPrototype(const QTableWidgetItem * item);
  fn _ZN12QTableWidget16setItemPrototypeEPK16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidget::NewQTableWidget(int rows, int columns, QWidget * parent);
  fn _ZN12QTableWidgetC1EiiP7QWidget(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) ;
  // proto:  int QTableWidget::visualRow(int logicalRow);
  fn _ZNK12QTableWidget9visualRowEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QTableWidget::cellEntered(int row, int column);
  fn _ZN12QTableWidget11cellEnteredEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QTableWidget::setCellWidget(int row, int column, QWidget * widget);
  fn _ZN12QTableWidget13setCellWidgetEiiP7QWidget(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) ;
  // proto:  void QTableWidget::openPersistentEditor(QTableWidgetItem * item);
  fn _ZN12QTableWidget20openPersistentEditorEP16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTableWidget::columnCount();
  fn _ZNK12QTableWidget11columnCountEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTableWidget::currentRow();
  fn _ZNK12QTableWidget10currentRowEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTableWidget::currentItemChanged(QTableWidgetItem * current, QTableWidgetItem * previous);
  fn _ZN12QTableWidget18currentItemChangedEP16QTableWidgetItemS1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QTableWidget::setCurrentItem(QTableWidgetItem * item);
  fn _ZN12QTableWidget14setCurrentItemEP16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QWidget * QTableWidget::cellWidget(int row, int column);
  fn _ZNK12QTableWidget10cellWidgetEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QTableWidget::setSortingEnabled(bool enable);
  fn _ZN12QTableWidget17setSortingEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTableWidget::setItem(int row, int column, QTableWidgetItem * item);
  fn _ZN12QTableWidget7setItemEiiP16QTableWidgetItem(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) ;
  // proto:  QTableWidgetItem * QTableWidget::horizontalHeaderItem(int column);
  fn _ZNK12QTableWidget20horizontalHeaderItemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTableWidget::cellPressed(int row, int column);
  fn _ZN12QTableWidget11cellPressedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QTableWidget::editItem(QTableWidgetItem * item);
  fn _ZN12QTableWidget8editItemEP16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QList<QTableWidgetSelectionRange> QTableWidget::selectedRanges();
  fn _ZNK12QTableWidget14selectedRangesEv(qthis: *mut c_void) ;
  // proto:  int QTableWidget::currentColumn();
  fn _ZNK12QTableWidget13currentColumnEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTableWidget::removeColumn(int column);
  fn _ZN12QTableWidget12removeColumnEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTableWidget::setRangeSelected(const QTableWidgetSelectionRange & range, bool select);
  fn _ZN12QTableWidget16setRangeSelectedERK26QTableWidgetSelectionRangeb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
  // proto:  int QTableWidget::column(const QTableWidgetItem * item);
  fn _ZNK12QTableWidget6columnEPK16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  bool QTableWidget::isItemSelected(const QTableWidgetItem * item);
  fn _ZNK12QTableWidget14isItemSelectedEPK16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QTableWidgetItem * QTableWidget::takeVerticalHeaderItem(int row);
  fn _ZN12QTableWidget22takeVerticalHeaderItemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTableWidget::insertRow(int row);
  fn _ZN12QTableWidget9insertRowEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTableWidget::currentCellChanged(int currentRow, int currentColumn, int previousRow, int previousColumn);
  fn _ZN12QTableWidget18currentCellChangedEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QTableWidget::cellDoubleClicked(int row, int column);
  fn _ZN12QTableWidget17cellDoubleClickedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  int QTableWidget::rowCount();
  fn _ZNK12QTableWidget8rowCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QTableWidgetItem * QTableWidget::item(int row, int column);
  fn _ZNK12QTableWidget4itemEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QTableWidget::NewQTableWidget(QWidget * parent);
  fn _ZN12QTableWidgetC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidget::setVerticalHeaderLabels(const QStringList & labels);
  fn _ZN12QTableWidget23setVerticalHeaderLabelsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QTableWidgetItem * QTableWidget::itemPrototype();
  fn _ZNK12QTableWidget13itemPrototypeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTableWidgetItem * QTableWidget::itemAt(const QPoint & p);
  fn _ZNK12QTableWidget6itemAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTableWidget::clearContents();
  fn _ZN12QTableWidget13clearContentsEv(qthis: *mut c_void) ;
  // proto:  void QTableWidget::itemPressed(QTableWidgetItem * item);
  fn _ZN12QTableWidget11itemPressedEP16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTableWidgetItem * QTableWidget::itemAt(int x, int y);
  fn _ZNK12QTableWidget6itemAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QTableWidget::setCurrentCell(int row, int column);
  fn _ZN12QTableWidget14setCurrentCellEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QTableWidget::setRowCount(int rows);
  fn _ZN12QTableWidget11setRowCountEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTableWidget::setHorizontalHeaderItem(int column, QTableWidgetItem * item);
  fn _ZN12QTableWidget23setHorizontalHeaderItemEiP16QTableWidgetItem(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  int QTableWidget::visualColumn(int logicalColumn);
  fn _ZNK12QTableWidget12visualColumnEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QTableWidget::itemEntered(QTableWidgetItem * item);
  fn _ZN12QTableWidget11itemEnteredEP16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTableWidgetItem * QTableWidget::takeHorizontalHeaderItem(int column);
  fn _ZN12QTableWidget24takeHorizontalHeaderItemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QTableWidgetItem * QTableWidget::verticalHeaderItem(int row);
  fn _ZNK12QTableWidget18verticalHeaderItemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTableWidget::clear();
  fn _ZN12QTableWidget5clearEv(qthis: *mut c_void) ;
  // proto:  void QTableWidget::insertColumn(int column);
  fn _ZN12QTableWidget12insertColumnEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTableWidget::cellActivated(int row, int column);
  fn _ZN12QTableWidget13cellActivatedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QTableWidget::itemClicked(QTableWidgetItem * item);
  fn _ZN12QTableWidget11itemClickedEP16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QTableWidget)=1
pub struct QTableWidget {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTableWidget {
  pub fn itemActivated<T: QTableWidget_itemActivated>(&mut self, value: T)  {
     value.itemActivated(self);
    // return 1;
  }
}

pub trait QTableWidget_itemActivated {
  fn itemActivated(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::itemActivated(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_itemActivated for (&'a mut QTableWidgetItem) {
  fn itemActivated(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget13itemActivatedEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget13itemActivatedEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn setColumnCount<T: QTableWidget_setColumnCount>(&mut self, value: T)  {
     value.setColumnCount(self);
    // return 1;
  }
}

pub trait QTableWidget_setColumnCount {
  fn setColumnCount(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::setColumnCount(int columns);
impl<'a> /*trait*/ QTableWidget_setColumnCount for (i32) {
  fn setColumnCount(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget14setColumnCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QTableWidget14setColumnCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn FreeQTableWidget<T: QTableWidget_FreeQTableWidget>(&mut self, value: T)  {
     value.FreeQTableWidget(self);
    // return 1;
  }
}

pub trait QTableWidget_FreeQTableWidget {
  fn FreeQTableWidget(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::FreeQTableWidget();
impl<'a> /*trait*/ QTableWidget_FreeQTableWidget for () {
  fn FreeQTableWidget(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidgetD0Ev()};
     unsafe {_ZN12QTableWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn itemDoubleClicked<T: QTableWidget_itemDoubleClicked>(&mut self, value: T)  {
     value.itemDoubleClicked(self);
    // return 1;
  }
}

pub trait QTableWidget_itemDoubleClicked {
  fn itemDoubleClicked(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::itemDoubleClicked(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_itemDoubleClicked for (&'a mut QTableWidgetItem) {
  fn itemDoubleClicked(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget17itemDoubleClickedEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget17itemDoubleClickedEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn cellChanged<T: QTableWidget_cellChanged>(&mut self, value: T)  {
     value.cellChanged(self);
    // return 1;
  }
}

pub trait QTableWidget_cellChanged {
  fn cellChanged(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::cellChanged(int row, int column);
impl<'a> /*trait*/ QTableWidget_cellChanged for (i32, i32) {
  fn cellChanged(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget11cellChangedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QTableWidget11cellChangedEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn selectedItems<T: QTableWidget_selectedItems>(&mut self, value: T)  {
     value.selectedItems(self);
    // return 1;
  }
}

pub trait QTableWidget_selectedItems {
  fn selectedItems(self, rsthis: &mut QTableWidget) ;
}

// proto:  QList<QTableWidgetItem *> QTableWidget::selectedItems();
impl<'a> /*trait*/ QTableWidget_selectedItems for () {
  fn selectedItems(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget13selectedItemsEv()};
     unsafe {_ZNK12QTableWidget13selectedItemsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn isSortingEnabled<T: QTableWidget_isSortingEnabled>(&mut self, value: T) -> i8 {
    return value.isSortingEnabled(self);
    // return 1;
  }
}

pub trait QTableWidget_isSortingEnabled {
  fn isSortingEnabled(self, rsthis: &mut QTableWidget) -> i8;
}

// proto:  bool QTableWidget::isSortingEnabled();
impl<'a> /*trait*/ QTableWidget_isSortingEnabled for () {
  fn isSortingEnabled(self, rsthis: &mut QTableWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget16isSortingEnabledEv()};
    let mut ret = unsafe {_ZNK12QTableWidget16isSortingEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn metaObject<T: QTableWidget_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QTableWidget_metaObject {
  fn metaObject(self, rsthis: &mut QTableWidget) ;
}

// proto:  const QMetaObject * QTableWidget::metaObject();
impl<'a> /*trait*/ QTableWidget_metaObject for () {
  fn metaObject(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget10metaObjectEv()};
     unsafe {_ZNK12QTableWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn NewQTableWidget<T: QTableWidget_NewQTableWidget>(value: T) -> QTableWidget {
    let rsthis = value.NewQTableWidget();
    return rsthis;
    // return 1;
  }
}

pub trait QTableWidget_NewQTableWidget {
  fn NewQTableWidget(self) -> QTableWidget;
}

// proto: void QTableWidget::NewQTableWidget(const QTableWidget & );
impl<'a> /*trait*/ QTableWidget_NewQTableWidget for (&'a  QTableWidget) {
  fn NewQTableWidget(self) -> QTableWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QTableWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QTableWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn closePersistentEditor<T: QTableWidget_closePersistentEditor>(&mut self, value: T)  {
     value.closePersistentEditor(self);
    // return 1;
  }
}

pub trait QTableWidget_closePersistentEditor {
  fn closePersistentEditor(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::closePersistentEditor(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_closePersistentEditor for (&'a mut QTableWidgetItem) {
  fn closePersistentEditor(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget21closePersistentEditorEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget21closePersistentEditorEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn setHorizontalHeaderLabels<T: QTableWidget_setHorizontalHeaderLabels>(&mut self, value: T)  {
     value.setHorizontalHeaderLabels(self);
    // return 1;
  }
}

pub trait QTableWidget_setHorizontalHeaderLabels {
  fn setHorizontalHeaderLabels(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::setHorizontalHeaderLabels(const QStringList & labels);
impl<'a> /*trait*/ QTableWidget_setHorizontalHeaderLabels for (&'a  QStringList) {
  fn setHorizontalHeaderLabels(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget25setHorizontalHeaderLabelsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget25setHorizontalHeaderLabelsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn itemSelectionChanged<T: QTableWidget_itemSelectionChanged>(&mut self, value: T)  {
     value.itemSelectionChanged(self);
    // return 1;
  }
}

pub trait QTableWidget_itemSelectionChanged {
  fn itemSelectionChanged(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::itemSelectionChanged();
impl<'a> /*trait*/ QTableWidget_itemSelectionChanged for () {
  fn itemSelectionChanged(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget20itemSelectionChangedEv()};
     unsafe {_ZN12QTableWidget20itemSelectionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn setItemSelected<T: QTableWidget_setItemSelected>(&mut self, value: T)  {
     value.setItemSelected(self);
    // return 1;
  }
}

pub trait QTableWidget_setItemSelected {
  fn setItemSelected(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::setItemSelected(const QTableWidgetItem * item, bool select);
impl<'a> /*trait*/ QTableWidget_setItemSelected for (&'a  QTableWidgetItem, i8) {
  fn setItemSelected(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget15setItemSelectedEPK16QTableWidgetItemb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN12QTableWidget15setItemSelectedEPK16QTableWidgetItemb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn takeItem<T: QTableWidget_takeItem>(&mut self, value: T) -> QTableWidgetItem {
    return value.takeItem(self);
    // return 1;
  }
}

pub trait QTableWidget_takeItem {
  fn takeItem(self, rsthis: &mut QTableWidget) -> QTableWidgetItem;
}

// proto:  QTableWidgetItem * QTableWidget::takeItem(int row, int column);
impl<'a> /*trait*/ QTableWidget_takeItem for (i32, i32) {
  fn takeItem(self, rsthis: &mut QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget8takeItemEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN12QTableWidget8takeItemEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTableWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn itemChanged<T: QTableWidget_itemChanged>(&mut self, value: T)  {
     value.itemChanged(self);
    // return 1;
  }
}

pub trait QTableWidget_itemChanged {
  fn itemChanged(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::itemChanged(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_itemChanged for (&'a mut QTableWidgetItem) {
  fn itemChanged(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget11itemChangedEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget11itemChangedEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn removeCellWidget<T: QTableWidget_removeCellWidget>(&mut self, value: T)  {
     value.removeCellWidget(self);
    // return 1;
  }
}

pub trait QTableWidget_removeCellWidget {
  fn removeCellWidget(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::removeCellWidget(int row, int column);
impl<'a> /*trait*/ QTableWidget_removeCellWidget for (i32, i32) {
  fn removeCellWidget(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget16removeCellWidgetEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QTableWidget16removeCellWidgetEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn setVerticalHeaderItem<T: QTableWidget_setVerticalHeaderItem>(&mut self, value: T)  {
     value.setVerticalHeaderItem(self);
    // return 1;
  }
}

pub trait QTableWidget_setVerticalHeaderItem {
  fn setVerticalHeaderItem(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::setVerticalHeaderItem(int row, QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_setVerticalHeaderItem for (i32, &'a mut QTableWidgetItem) {
  fn setVerticalHeaderItem(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget21setVerticalHeaderItemEiP16QTableWidgetItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget21setVerticalHeaderItemEiP16QTableWidgetItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn cellClicked<T: QTableWidget_cellClicked>(&mut self, value: T)  {
     value.cellClicked(self);
    // return 1;
  }
}

pub trait QTableWidget_cellClicked {
  fn cellClicked(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::cellClicked(int row, int column);
impl<'a> /*trait*/ QTableWidget_cellClicked for (i32, i32) {
  fn cellClicked(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget11cellClickedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QTableWidget11cellClickedEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn visualItemRect<T: QTableWidget_visualItemRect>(&mut self, value: T) -> QRect {
    return value.visualItemRect(self);
    // return 1;
  }
}

pub trait QTableWidget_visualItemRect {
  fn visualItemRect(self, rsthis: &mut QTableWidget) -> QRect;
}

// proto:  QRect QTableWidget::visualItemRect(const QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_visualItemRect for (&'a  QTableWidgetItem) {
  fn visualItemRect(self, rsthis: &mut QTableWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget14visualItemRectEPK16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QTableWidget14visualItemRectEPK16QTableWidgetItem(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn currentItem<T: QTableWidget_currentItem>(&mut self, value: T) -> QTableWidgetItem {
    return value.currentItem(self);
    // return 1;
  }
}

pub trait QTableWidget_currentItem {
  fn currentItem(self, rsthis: &mut QTableWidget) -> QTableWidgetItem;
}

// proto:  QTableWidgetItem * QTableWidget::currentItem();
impl<'a> /*trait*/ QTableWidget_currentItem for () {
  fn currentItem(self, rsthis: &mut QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget11currentItemEv()};
    let mut ret = unsafe {_ZNK12QTableWidget11currentItemEv(rsthis.qclsinst)};
    let mut ret1 = QTableWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn row<T: QTableWidget_row>(&mut self, value: T) -> i32 {
    return value.row(self);
    // return 1;
  }
}

pub trait QTableWidget_row {
  fn row(self, rsthis: &mut QTableWidget) -> i32;
}

// proto:  int QTableWidget::row(const QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_row for (&'a  QTableWidgetItem) {
  fn row(self, rsthis: &mut QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget3rowEPK16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QTableWidget3rowEPK16QTableWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn removeRow<T: QTableWidget_removeRow>(&mut self, value: T)  {
     value.removeRow(self);
    // return 1;
  }
}

pub trait QTableWidget_removeRow {
  fn removeRow(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::removeRow(int row);
impl<'a> /*trait*/ QTableWidget_removeRow for (i32) {
  fn removeRow(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget9removeRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QTableWidget9removeRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn setItemPrototype<T: QTableWidget_setItemPrototype>(&mut self, value: T)  {
     value.setItemPrototype(self);
    // return 1;
  }
}

pub trait QTableWidget_setItemPrototype {
  fn setItemPrototype(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::setItemPrototype(const QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_setItemPrototype for (&'a  QTableWidgetItem) {
  fn setItemPrototype(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget16setItemPrototypeEPK16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget16setItemPrototypeEPK16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QTableWidget::NewQTableWidget(int rows, int columns, QWidget * parent);
impl<'a> /*trait*/ QTableWidget_NewQTableWidget for (i32, i32, &'a mut QWidget) {
  fn NewQTableWidget(self) -> QTableWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidgetC1EiiP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN12QTableWidgetC1EiiP7QWidget(qthis, arg0, arg1, arg2)};
    let rsthis = QTableWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn visualRow<T: QTableWidget_visualRow>(&mut self, value: T) -> i32 {
    return value.visualRow(self);
    // return 1;
  }
}

pub trait QTableWidget_visualRow {
  fn visualRow(self, rsthis: &mut QTableWidget) -> i32;
}

// proto:  int QTableWidget::visualRow(int logicalRow);
impl<'a> /*trait*/ QTableWidget_visualRow for (i32) {
  fn visualRow(self, rsthis: &mut QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget9visualRowEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK12QTableWidget9visualRowEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn cellEntered<T: QTableWidget_cellEntered>(&mut self, value: T)  {
     value.cellEntered(self);
    // return 1;
  }
}

pub trait QTableWidget_cellEntered {
  fn cellEntered(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::cellEntered(int row, int column);
impl<'a> /*trait*/ QTableWidget_cellEntered for (i32, i32) {
  fn cellEntered(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget11cellEnteredEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QTableWidget11cellEnteredEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn setCellWidget<T: QTableWidget_setCellWidget>(&mut self, value: T)  {
     value.setCellWidget(self);
    // return 1;
  }
}

pub trait QTableWidget_setCellWidget {
  fn setCellWidget(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::setCellWidget(int row, int column, QWidget * widget);
impl<'a> /*trait*/ QTableWidget_setCellWidget for (i32, i32, &'a mut QWidget) {
  fn setCellWidget(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget13setCellWidgetEiiP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget13setCellWidgetEiiP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn openPersistentEditor<T: QTableWidget_openPersistentEditor>(&mut self, value: T)  {
     value.openPersistentEditor(self);
    // return 1;
  }
}

pub trait QTableWidget_openPersistentEditor {
  fn openPersistentEditor(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::openPersistentEditor(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_openPersistentEditor for (&'a mut QTableWidgetItem) {
  fn openPersistentEditor(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget20openPersistentEditorEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget20openPersistentEditorEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn columnCount<T: QTableWidget_columnCount>(&mut self, value: T) -> i32 {
    return value.columnCount(self);
    // return 1;
  }
}

pub trait QTableWidget_columnCount {
  fn columnCount(self, rsthis: &mut QTableWidget) -> i32;
}

// proto:  int QTableWidget::columnCount();
impl<'a> /*trait*/ QTableWidget_columnCount for () {
  fn columnCount(self, rsthis: &mut QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget11columnCountEv()};
    let mut ret = unsafe {_ZNK12QTableWidget11columnCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn currentRow<T: QTableWidget_currentRow>(&mut self, value: T) -> i32 {
    return value.currentRow(self);
    // return 1;
  }
}

pub trait QTableWidget_currentRow {
  fn currentRow(self, rsthis: &mut QTableWidget) -> i32;
}

// proto:  int QTableWidget::currentRow();
impl<'a> /*trait*/ QTableWidget_currentRow for () {
  fn currentRow(self, rsthis: &mut QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget10currentRowEv()};
    let mut ret = unsafe {_ZNK12QTableWidget10currentRowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn currentItemChanged<T: QTableWidget_currentItemChanged>(&mut self, value: T)  {
     value.currentItemChanged(self);
    // return 1;
  }
}

pub trait QTableWidget_currentItemChanged {
  fn currentItemChanged(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::currentItemChanged(QTableWidgetItem * current, QTableWidgetItem * previous);
impl<'a> /*trait*/ QTableWidget_currentItemChanged for (&'a mut QTableWidgetItem, &'a mut QTableWidgetItem) {
  fn currentItemChanged(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget18currentItemChangedEP16QTableWidgetItemS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget18currentItemChangedEP16QTableWidgetItemS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn setCurrentItem<T: QTableWidget_setCurrentItem>(&mut self, value: T)  {
     value.setCurrentItem(self);
    // return 1;
  }
}

pub trait QTableWidget_setCurrentItem {
  fn setCurrentItem(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::setCurrentItem(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_setCurrentItem for (&'a mut QTableWidgetItem) {
  fn setCurrentItem(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget14setCurrentItemEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget14setCurrentItemEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn cellWidget<T: QTableWidget_cellWidget>(&mut self, value: T) -> QWidget {
    return value.cellWidget(self);
    // return 1;
  }
}

pub trait QTableWidget_cellWidget {
  fn cellWidget(self, rsthis: &mut QTableWidget) -> QWidget;
}

// proto:  QWidget * QTableWidget::cellWidget(int row, int column);
impl<'a> /*trait*/ QTableWidget_cellWidget for (i32, i32) {
  fn cellWidget(self, rsthis: &mut QTableWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget10cellWidgetEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK12QTableWidget10cellWidgetEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn setSortingEnabled<T: QTableWidget_setSortingEnabled>(&mut self, value: T)  {
     value.setSortingEnabled(self);
    // return 1;
  }
}

pub trait QTableWidget_setSortingEnabled {
  fn setSortingEnabled(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::setSortingEnabled(bool enable);
impl<'a> /*trait*/ QTableWidget_setSortingEnabled for (i8) {
  fn setSortingEnabled(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget17setSortingEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN12QTableWidget17setSortingEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn setItem<T: QTableWidget_setItem>(&mut self, value: T)  {
     value.setItem(self);
    // return 1;
  }
}

pub trait QTableWidget_setItem {
  fn setItem(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::setItem(int row, int column, QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_setItem for (i32, i32, &'a mut QTableWidgetItem) {
  fn setItem(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget7setItemEiiP16QTableWidgetItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget7setItemEiiP16QTableWidgetItem(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn horizontalHeaderItem<T: QTableWidget_horizontalHeaderItem>(&mut self, value: T) -> QTableWidgetItem {
    return value.horizontalHeaderItem(self);
    // return 1;
  }
}

pub trait QTableWidget_horizontalHeaderItem {
  fn horizontalHeaderItem(self, rsthis: &mut QTableWidget) -> QTableWidgetItem;
}

// proto:  QTableWidgetItem * QTableWidget::horizontalHeaderItem(int column);
impl<'a> /*trait*/ QTableWidget_horizontalHeaderItem for (i32) {
  fn horizontalHeaderItem(self, rsthis: &mut QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget20horizontalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK12QTableWidget20horizontalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTableWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn cellPressed<T: QTableWidget_cellPressed>(&mut self, value: T)  {
     value.cellPressed(self);
    // return 1;
  }
}

pub trait QTableWidget_cellPressed {
  fn cellPressed(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::cellPressed(int row, int column);
impl<'a> /*trait*/ QTableWidget_cellPressed for (i32, i32) {
  fn cellPressed(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget11cellPressedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QTableWidget11cellPressedEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn editItem<T: QTableWidget_editItem>(&mut self, value: T)  {
     value.editItem(self);
    // return 1;
  }
}

pub trait QTableWidget_editItem {
  fn editItem(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::editItem(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_editItem for (&'a mut QTableWidgetItem) {
  fn editItem(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget8editItemEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget8editItemEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn selectedRanges<T: QTableWidget_selectedRanges>(&mut self, value: T)  {
     value.selectedRanges(self);
    // return 1;
  }
}

pub trait QTableWidget_selectedRanges {
  fn selectedRanges(self, rsthis: &mut QTableWidget) ;
}

// proto:  QList<QTableWidgetSelectionRange> QTableWidget::selectedRanges();
impl<'a> /*trait*/ QTableWidget_selectedRanges for () {
  fn selectedRanges(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget14selectedRangesEv()};
     unsafe {_ZNK12QTableWidget14selectedRangesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn currentColumn<T: QTableWidget_currentColumn>(&mut self, value: T) -> i32 {
    return value.currentColumn(self);
    // return 1;
  }
}

pub trait QTableWidget_currentColumn {
  fn currentColumn(self, rsthis: &mut QTableWidget) -> i32;
}

// proto:  int QTableWidget::currentColumn();
impl<'a> /*trait*/ QTableWidget_currentColumn for () {
  fn currentColumn(self, rsthis: &mut QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget13currentColumnEv()};
    let mut ret = unsafe {_ZNK12QTableWidget13currentColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn removeColumn<T: QTableWidget_removeColumn>(&mut self, value: T)  {
     value.removeColumn(self);
    // return 1;
  }
}

pub trait QTableWidget_removeColumn {
  fn removeColumn(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::removeColumn(int column);
impl<'a> /*trait*/ QTableWidget_removeColumn for (i32) {
  fn removeColumn(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget12removeColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QTableWidget12removeColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn setRangeSelected<T: QTableWidget_setRangeSelected>(&mut self, value: T)  {
     value.setRangeSelected(self);
    // return 1;
  }
}

pub trait QTableWidget_setRangeSelected {
  fn setRangeSelected(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::setRangeSelected(const QTableWidgetSelectionRange & range, bool select);
impl<'a> /*trait*/ QTableWidget_setRangeSelected for (&'a  QTableWidgetSelectionRange, i8) {
  fn setRangeSelected(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget16setRangeSelectedERK26QTableWidgetSelectionRangeb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN12QTableWidget16setRangeSelectedERK26QTableWidgetSelectionRangeb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn column<T: QTableWidget_column>(&mut self, value: T) -> i32 {
    return value.column(self);
    // return 1;
  }
}

pub trait QTableWidget_column {
  fn column(self, rsthis: &mut QTableWidget) -> i32;
}

// proto:  int QTableWidget::column(const QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_column for (&'a  QTableWidgetItem) {
  fn column(self, rsthis: &mut QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget6columnEPK16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QTableWidget6columnEPK16QTableWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn isItemSelected<T: QTableWidget_isItemSelected>(&mut self, value: T) -> i8 {
    return value.isItemSelected(self);
    // return 1;
  }
}

pub trait QTableWidget_isItemSelected {
  fn isItemSelected(self, rsthis: &mut QTableWidget) -> i8;
}

// proto:  bool QTableWidget::isItemSelected(const QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_isItemSelected for (&'a  QTableWidgetItem) {
  fn isItemSelected(self, rsthis: &mut QTableWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget14isItemSelectedEPK16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QTableWidget14isItemSelectedEPK16QTableWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn takeVerticalHeaderItem<T: QTableWidget_takeVerticalHeaderItem>(&mut self, value: T) -> QTableWidgetItem {
    return value.takeVerticalHeaderItem(self);
    // return 1;
  }
}

pub trait QTableWidget_takeVerticalHeaderItem {
  fn takeVerticalHeaderItem(self, rsthis: &mut QTableWidget) -> QTableWidgetItem;
}

// proto:  QTableWidgetItem * QTableWidget::takeVerticalHeaderItem(int row);
impl<'a> /*trait*/ QTableWidget_takeVerticalHeaderItem for (i32) {
  fn takeVerticalHeaderItem(self, rsthis: &mut QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget22takeVerticalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN12QTableWidget22takeVerticalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTableWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn insertRow<T: QTableWidget_insertRow>(&mut self, value: T)  {
     value.insertRow(self);
    // return 1;
  }
}

pub trait QTableWidget_insertRow {
  fn insertRow(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::insertRow(int row);
impl<'a> /*trait*/ QTableWidget_insertRow for (i32) {
  fn insertRow(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget9insertRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QTableWidget9insertRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn currentCellChanged<T: QTableWidget_currentCellChanged>(&mut self, value: T)  {
     value.currentCellChanged(self);
    // return 1;
  }
}

pub trait QTableWidget_currentCellChanged {
  fn currentCellChanged(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::currentCellChanged(int currentRow, int currentColumn, int previousRow, int previousColumn);
impl<'a> /*trait*/ QTableWidget_currentCellChanged for (i32, i32, i32, i32) {
  fn currentCellChanged(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget18currentCellChangedEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN12QTableWidget18currentCellChangedEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn cellDoubleClicked<T: QTableWidget_cellDoubleClicked>(&mut self, value: T)  {
     value.cellDoubleClicked(self);
    // return 1;
  }
}

pub trait QTableWidget_cellDoubleClicked {
  fn cellDoubleClicked(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::cellDoubleClicked(int row, int column);
impl<'a> /*trait*/ QTableWidget_cellDoubleClicked for (i32, i32) {
  fn cellDoubleClicked(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget17cellDoubleClickedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QTableWidget17cellDoubleClickedEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn rowCount<T: QTableWidget_rowCount>(&mut self, value: T) -> i32 {
    return value.rowCount(self);
    // return 1;
  }
}

pub trait QTableWidget_rowCount {
  fn rowCount(self, rsthis: &mut QTableWidget) -> i32;
}

// proto:  int QTableWidget::rowCount();
impl<'a> /*trait*/ QTableWidget_rowCount for () {
  fn rowCount(self, rsthis: &mut QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget8rowCountEv()};
    let mut ret = unsafe {_ZNK12QTableWidget8rowCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn item<T: QTableWidget_item>(&mut self, value: T) -> QTableWidgetItem {
    return value.item(self);
    // return 1;
  }
}

pub trait QTableWidget_item {
  fn item(self, rsthis: &mut QTableWidget) -> QTableWidgetItem;
}

// proto:  QTableWidgetItem * QTableWidget::item(int row, int column);
impl<'a> /*trait*/ QTableWidget_item for (i32, i32) {
  fn item(self, rsthis: &mut QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget4itemEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK12QTableWidget4itemEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTableWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTableWidget::NewQTableWidget(QWidget * parent);
impl<'a> /*trait*/ QTableWidget_NewQTableWidget for (&'a mut QWidget) {
  fn NewQTableWidget(self) -> QTableWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidgetC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QTableWidgetC1EP7QWidget(qthis, arg0)};
    let rsthis = QTableWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn setVerticalHeaderLabels<T: QTableWidget_setVerticalHeaderLabels>(&mut self, value: T)  {
     value.setVerticalHeaderLabels(self);
    // return 1;
  }
}

pub trait QTableWidget_setVerticalHeaderLabels {
  fn setVerticalHeaderLabels(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::setVerticalHeaderLabels(const QStringList & labels);
impl<'a> /*trait*/ QTableWidget_setVerticalHeaderLabels for (&'a  QStringList) {
  fn setVerticalHeaderLabels(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget23setVerticalHeaderLabelsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget23setVerticalHeaderLabelsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn itemPrototype<T: QTableWidget_itemPrototype>(&mut self, value: T) -> QTableWidgetItem {
    return value.itemPrototype(self);
    // return 1;
  }
}

pub trait QTableWidget_itemPrototype {
  fn itemPrototype(self, rsthis: &mut QTableWidget) -> QTableWidgetItem;
}

// proto:  const QTableWidgetItem * QTableWidget::itemPrototype();
impl<'a> /*trait*/ QTableWidget_itemPrototype for () {
  fn itemPrototype(self, rsthis: &mut QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget13itemPrototypeEv()};
    let mut ret = unsafe {_ZNK12QTableWidget13itemPrototypeEv(rsthis.qclsinst)};
    let mut ret1 = QTableWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn itemAt<T: QTableWidget_itemAt>(&mut self, value: T) -> QTableWidgetItem {
    return value.itemAt(self);
    // return 1;
  }
}

pub trait QTableWidget_itemAt {
  fn itemAt(self, rsthis: &mut QTableWidget) -> QTableWidgetItem;
}

// proto:  QTableWidgetItem * QTableWidget::itemAt(const QPoint & p);
impl<'a> /*trait*/ QTableWidget_itemAt for (&'a  QPoint) {
  fn itemAt(self, rsthis: &mut QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget6itemAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QTableWidget6itemAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QTableWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn clearContents<T: QTableWidget_clearContents>(&mut self, value: T)  {
     value.clearContents(self);
    // return 1;
  }
}

pub trait QTableWidget_clearContents {
  fn clearContents(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::clearContents();
impl<'a> /*trait*/ QTableWidget_clearContents for () {
  fn clearContents(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget13clearContentsEv()};
     unsafe {_ZN12QTableWidget13clearContentsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn itemPressed<T: QTableWidget_itemPressed>(&mut self, value: T)  {
     value.itemPressed(self);
    // return 1;
  }
}

pub trait QTableWidget_itemPressed {
  fn itemPressed(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::itemPressed(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_itemPressed for (&'a mut QTableWidgetItem) {
  fn itemPressed(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget11itemPressedEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget11itemPressedEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QTableWidgetItem * QTableWidget::itemAt(int x, int y);
impl<'a> /*trait*/ QTableWidget_itemAt for (i32, i32) {
  fn itemAt(self, rsthis: &mut QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget6itemAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK12QTableWidget6itemAtEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTableWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn setCurrentCell<T: QTableWidget_setCurrentCell>(&mut self, value: T)  {
     value.setCurrentCell(self);
    // return 1;
  }
}

pub trait QTableWidget_setCurrentCell {
  fn setCurrentCell(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::setCurrentCell(int row, int column);
impl<'a> /*trait*/ QTableWidget_setCurrentCell for (i32, i32) {
  fn setCurrentCell(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget14setCurrentCellEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QTableWidget14setCurrentCellEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn setRowCount<T: QTableWidget_setRowCount>(&mut self, value: T)  {
     value.setRowCount(self);
    // return 1;
  }
}

pub trait QTableWidget_setRowCount {
  fn setRowCount(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::setRowCount(int rows);
impl<'a> /*trait*/ QTableWidget_setRowCount for (i32) {
  fn setRowCount(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget11setRowCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QTableWidget11setRowCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn setHorizontalHeaderItem<T: QTableWidget_setHorizontalHeaderItem>(&mut self, value: T)  {
     value.setHorizontalHeaderItem(self);
    // return 1;
  }
}

pub trait QTableWidget_setHorizontalHeaderItem {
  fn setHorizontalHeaderItem(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::setHorizontalHeaderItem(int column, QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_setHorizontalHeaderItem for (i32, &'a mut QTableWidgetItem) {
  fn setHorizontalHeaderItem(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget23setHorizontalHeaderItemEiP16QTableWidgetItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget23setHorizontalHeaderItemEiP16QTableWidgetItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn visualColumn<T: QTableWidget_visualColumn>(&mut self, value: T) -> i32 {
    return value.visualColumn(self);
    // return 1;
  }
}

pub trait QTableWidget_visualColumn {
  fn visualColumn(self, rsthis: &mut QTableWidget) -> i32;
}

// proto:  int QTableWidget::visualColumn(int logicalColumn);
impl<'a> /*trait*/ QTableWidget_visualColumn for (i32) {
  fn visualColumn(self, rsthis: &mut QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget12visualColumnEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK12QTableWidget12visualColumnEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn itemEntered<T: QTableWidget_itemEntered>(&mut self, value: T)  {
     value.itemEntered(self);
    // return 1;
  }
}

pub trait QTableWidget_itemEntered {
  fn itemEntered(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::itemEntered(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_itemEntered for (&'a mut QTableWidgetItem) {
  fn itemEntered(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget11itemEnteredEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget11itemEnteredEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn takeHorizontalHeaderItem<T: QTableWidget_takeHorizontalHeaderItem>(&mut self, value: T) -> QTableWidgetItem {
    return value.takeHorizontalHeaderItem(self);
    // return 1;
  }
}

pub trait QTableWidget_takeHorizontalHeaderItem {
  fn takeHorizontalHeaderItem(self, rsthis: &mut QTableWidget) -> QTableWidgetItem;
}

// proto:  QTableWidgetItem * QTableWidget::takeHorizontalHeaderItem(int column);
impl<'a> /*trait*/ QTableWidget_takeHorizontalHeaderItem for (i32) {
  fn takeHorizontalHeaderItem(self, rsthis: &mut QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget24takeHorizontalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN12QTableWidget24takeHorizontalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTableWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn verticalHeaderItem<T: QTableWidget_verticalHeaderItem>(&mut self, value: T) -> QTableWidgetItem {
    return value.verticalHeaderItem(self);
    // return 1;
  }
}

pub trait QTableWidget_verticalHeaderItem {
  fn verticalHeaderItem(self, rsthis: &mut QTableWidget) -> QTableWidgetItem;
}

// proto:  QTableWidgetItem * QTableWidget::verticalHeaderItem(int row);
impl<'a> /*trait*/ QTableWidget_verticalHeaderItem for (i32) {
  fn verticalHeaderItem(self, rsthis: &mut QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget18verticalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK12QTableWidget18verticalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTableWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn clear<T: QTableWidget_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QTableWidget_clear {
  fn clear(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::clear();
impl<'a> /*trait*/ QTableWidget_clear for () {
  fn clear(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget5clearEv()};
     unsafe {_ZN12QTableWidget5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn insertColumn<T: QTableWidget_insertColumn>(&mut self, value: T)  {
     value.insertColumn(self);
    // return 1;
  }
}

pub trait QTableWidget_insertColumn {
  fn insertColumn(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::insertColumn(int column);
impl<'a> /*trait*/ QTableWidget_insertColumn for (i32) {
  fn insertColumn(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget12insertColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QTableWidget12insertColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn cellActivated<T: QTableWidget_cellActivated>(&mut self, value: T)  {
     value.cellActivated(self);
    // return 1;
  }
}

pub trait QTableWidget_cellActivated {
  fn cellActivated(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::cellActivated(int row, int column);
impl<'a> /*trait*/ QTableWidget_cellActivated for (i32, i32) {
  fn cellActivated(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget13cellActivatedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QTableWidget13cellActivatedEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn itemClicked<T: QTableWidget_itemClicked>(&mut self, value: T)  {
     value.itemClicked(self);
    // return 1;
  }
}

pub trait QTableWidget_itemClicked {
  fn itemClicked(self, rsthis: &mut QTableWidget) ;
}

// proto:  void QTableWidget::itemClicked(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_itemClicked for (&'a mut QTableWidgetItem) {
  fn itemClicked(self, rsthis: &mut QTableWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget11itemClickedEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget11itemClickedEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

