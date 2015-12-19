// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qurlquery::QUrlQuery;
use super::qstring::QString;
use super::qbytearray::QByteArray;
use super::qstringlist::QStringList;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QUrl::isLocalFile();
  fn _ZNK4QUrl11isLocalFileEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QUrl::isEmpty();
  fn _ZNK4QUrl7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QUrl::setQuery(const QUrlQuery & query);
  fn _ZN4QUrl8setQueryERK9QUrlQuery(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QStringList QUrl::idnWhitelist();
  fn _ZN4QUrl12idnWhitelistEv() ;
  // proto:  void QUrl::FreeQUrl();
  fn _ZN4QUrlD0Ev(qthis: *mut c_void) ;
  // proto:  void QUrl::setScheme(const QString & scheme);
  fn _ZN4QUrl9setSchemeERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QUrl::isParentOf(const QUrl & url);
  fn _ZNK4QUrl10isParentOfERKS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QString QUrl::errorString();
  fn _ZNK4QUrl11errorStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QUrl::port(int defaultPort);
  fn _ZNK4QUrl4portEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QUrl::setPort(int port);
  fn _ZN4QUrl7setPortEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QUrl::NewQUrl(const QUrl & copy);
  fn _ZN4QUrlC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QString QUrl::fromAce(const QByteArray & );
  fn _ZN4QUrl7fromAceERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto:  QUrl QUrl::resolved(const QUrl & relative);
  fn _ZNK4QUrl8resolvedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString QUrl::toLocalFile();
  fn _ZNK4QUrl11toLocalFileEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QUrl::detach();
  fn _ZN4QUrl6detachEv(qthis: *mut c_void) ;
  // proto:  bool QUrl::hasFragment();
  fn _ZNK4QUrl11hasFragmentEv(qthis: *mut c_void) -> int8_t;
  // proto: static QByteArray QUrl::toAce(const QString & );
  fn _ZN4QUrl5toAceERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QUrl::hasQuery();
  fn _ZNK4QUrl8hasQueryEv(qthis: *mut c_void) -> int8_t;
  // proto: static QUrl QUrl::fromLocalFile(const QString & localfile);
  fn _ZN4QUrl13fromLocalFileERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QUrl::isValid();
  fn _ZNK4QUrl7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QUrl::NewQUrl();
  fn _ZN4QUrlC1Ev(qthis: *mut c_void) ;
  // proto:  bool QUrl::isDetached();
  fn _ZNK4QUrl10isDetachedEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QUrl::isRelative();
  fn _ZNK4QUrl10isRelativeEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QUrl::scheme();
  fn _ZNK4QUrl6schemeEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QByteArray QUrl::toPercentEncoding(const QString & , const QByteArray & exclude, const QByteArray & include);
  fn _ZN4QUrl17toPercentEncodingERK7QStringRK10QByteArrayS5_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto: static void QUrl::setIdnWhitelist(const QStringList & );
  fn _ZN4QUrl15setIdnWhitelistERK11QStringList(arg0: *mut c_void) ;
  // proto:  void QUrl::swap(QUrl & other);
  fn _ZN4QUrl4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QString QUrl::fromPercentEncoding(const QByteArray & );
  fn _ZN4QUrl19fromPercentEncodingERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto: static QUrl QUrl::fromUserInput(const QString & userInput);
  fn _ZN4QUrl13fromUserInputERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QUrl::clear();
  fn _ZN4QUrl5clearEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QUrl)=8
pub struct QUrl {
  pub qclsinst: *mut c_void,
}

// proto:  bool QUrl::isLocalFile();
impl /*struct*/ QUrl {
  pub fn isLocalFile<RetType, T: QUrl_isLocalFile<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isLocalFile(self);
    // return 1;
  }
}

pub trait QUrl_isLocalFile<RetType> {
  fn isLocalFile(self , rsthis: &mut QUrl) -> RetType;
}

// proto:  bool QUrl::isLocalFile();
impl<'a> /*trait*/ QUrl_isLocalFile<i8> for () {
  fn isLocalFile(self , rsthis: &mut QUrl) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl11isLocalFileEv()};
    let mut ret = unsafe {_ZNK4QUrl11isLocalFileEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  bool QUrl::isEmpty();
impl /*struct*/ QUrl {
  pub fn isEmpty<RetType, T: QUrl_isEmpty<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QUrl_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QUrl) -> RetType;
}

// proto:  bool QUrl::isEmpty();
impl<'a> /*trait*/ QUrl_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QUrl) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl7isEmptyEv()};
    let mut ret = unsafe {_ZNK4QUrl7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QUrl::setQuery(const QUrlQuery & query);
impl /*struct*/ QUrl {
  pub fn setQuery<RetType, T: QUrl_setQuery<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setQuery(self);
    // return 1;
  }
}

pub trait QUrl_setQuery<RetType> {
  fn setQuery(self , rsthis: &mut QUrl) -> RetType;
}

// proto:  void QUrl::setQuery(const QUrlQuery & query);
impl<'a> /*trait*/ QUrl_setQuery<()> for (&'a  QUrlQuery) {
  fn setQuery(self , rsthis: &mut QUrl) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl8setQueryERK9QUrlQuery()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QUrl8setQueryERK9QUrlQuery(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: static QStringList QUrl::idnWhitelist();
impl /*struct*/ QUrl {
  pub fn idnWhitelist_s<RetType, T: QUrl_idnWhitelist_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.idnWhitelist_s();
    // return 1;
  }
}

pub trait QUrl_idnWhitelist_s<RetType> {
  fn idnWhitelist_s(self ) -> RetType;
}

// proto: static QStringList QUrl::idnWhitelist();
impl<'a> /*trait*/ QUrl_idnWhitelist_s<()> for () {
  fn idnWhitelist_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl12idnWhitelistEv()};
     unsafe {_ZN4QUrl12idnWhitelistEv()};
    // return 1;
  }
}

// proto:  void QUrl::FreeQUrl();
impl /*struct*/ QUrl {
  pub fn FreeQUrl<RetType, T: QUrl_FreeQUrl<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQUrl(self);
    // return 1;
  }
}

pub trait QUrl_FreeQUrl<RetType> {
  fn FreeQUrl(self , rsthis: &mut QUrl) -> RetType;
}

// proto:  void QUrl::FreeQUrl();
impl<'a> /*trait*/ QUrl_FreeQUrl<()> for () {
  fn FreeQUrl(self , rsthis: &mut QUrl) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrlD0Ev()};
     unsafe {_ZN4QUrlD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QUrl::setScheme(const QString & scheme);
impl /*struct*/ QUrl {
  pub fn setScheme<RetType, T: QUrl_setScheme<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setScheme(self);
    // return 1;
  }
}

pub trait QUrl_setScheme<RetType> {
  fn setScheme(self , rsthis: &mut QUrl) -> RetType;
}

// proto:  void QUrl::setScheme(const QString & scheme);
impl<'a> /*trait*/ QUrl_setScheme<()> for (&'a  QString) {
  fn setScheme(self , rsthis: &mut QUrl) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl9setSchemeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QUrl9setSchemeERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QUrl::isParentOf(const QUrl & url);
impl /*struct*/ QUrl {
  pub fn isParentOf<RetType, T: QUrl_isParentOf<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isParentOf(self);
    // return 1;
  }
}

pub trait QUrl_isParentOf<RetType> {
  fn isParentOf(self , rsthis: &mut QUrl) -> RetType;
}

// proto:  bool QUrl::isParentOf(const QUrl & url);
impl<'a> /*trait*/ QUrl_isParentOf<i8> for (&'a  QUrl) {
  fn isParentOf(self , rsthis: &mut QUrl) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl10isParentOfERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK4QUrl10isParentOfERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QString QUrl::errorString();
impl /*struct*/ QUrl {
  pub fn errorString<RetType, T: QUrl_errorString<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QUrl_errorString<RetType> {
  fn errorString(self , rsthis: &mut QUrl) -> RetType;
}

// proto:  QString QUrl::errorString();
impl<'a> /*trait*/ QUrl_errorString<QString> for () {
  fn errorString(self , rsthis: &mut QUrl) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl11errorStringEv()};
    let mut ret = unsafe {_ZNK4QUrl11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  int QUrl::port(int defaultPort);
impl /*struct*/ QUrl {
  pub fn port<RetType, T: QUrl_port<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.port(self);
    // return 1;
  }
}

pub trait QUrl_port<RetType> {
  fn port(self , rsthis: &mut QUrl) -> RetType;
}

// proto:  int QUrl::port(int defaultPort);
impl<'a> /*trait*/ QUrl_port<i32> for (i32) {
  fn port(self , rsthis: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl4portEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK4QUrl4portEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QUrl::setPort(int port);
impl /*struct*/ QUrl {
  pub fn setPort<RetType, T: QUrl_setPort<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setPort(self);
    // return 1;
  }
}

pub trait QUrl_setPort<RetType> {
  fn setPort(self , rsthis: &mut QUrl) -> RetType;
}

// proto:  void QUrl::setPort(int port);
impl<'a> /*trait*/ QUrl_setPort<()> for (i32) {
  fn setPort(self , rsthis: &mut QUrl) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl7setPortEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN4QUrl7setPortEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn NewQUrl<T: QUrl_NewQUrl>(value: T) -> QUrl {
    let rsthis = value.NewQUrl();
    return rsthis;
    // return 1;
  }
}

pub trait QUrl_NewQUrl {
  fn NewQUrl(self) -> QUrl;
}

// proto: void QUrl::NewQUrl(const QUrl & copy);
impl<'a> /*trait*/ QUrl_NewQUrl for (&'a  QUrl) {
  fn NewQUrl(self) -> QUrl {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrlC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN4QUrlC1ERKS_(qthis, arg0)};
    let rsthis = QUrl{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: static QString QUrl::fromAce(const QByteArray & );
impl /*struct*/ QUrl {
  pub fn fromAce_s<RetType, T: QUrl_fromAce_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromAce_s();
    // return 1;
  }
}

pub trait QUrl_fromAce_s<RetType> {
  fn fromAce_s(self ) -> RetType;
}

// proto: static QString QUrl::fromAce(const QByteArray & );
impl<'a> /*trait*/ QUrl_fromAce_s<QString> for (&'a  QByteArray) {
  fn fromAce_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl7fromAceERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QUrl7fromAceERK10QByteArray(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QUrl QUrl::resolved(const QUrl & relative);
impl /*struct*/ QUrl {
  pub fn resolved<RetType, T: QUrl_resolved<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.resolved(self);
    // return 1;
  }
}

pub trait QUrl_resolved<RetType> {
  fn resolved(self , rsthis: &mut QUrl) -> RetType;
}

// proto:  QUrl QUrl::resolved(const QUrl & relative);
impl<'a> /*trait*/ QUrl_resolved<QUrl> for (&'a  QUrl) {
  fn resolved(self , rsthis: &mut QUrl) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl8resolvedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK4QUrl8resolvedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QUrl{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QString QUrl::toLocalFile();
impl /*struct*/ QUrl {
  pub fn toLocalFile<RetType, T: QUrl_toLocalFile<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.toLocalFile(self);
    // return 1;
  }
}

pub trait QUrl_toLocalFile<RetType> {
  fn toLocalFile(self , rsthis: &mut QUrl) -> RetType;
}

// proto:  QString QUrl::toLocalFile();
impl<'a> /*trait*/ QUrl_toLocalFile<QString> for () {
  fn toLocalFile(self , rsthis: &mut QUrl) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl11toLocalFileEv()};
    let mut ret = unsafe {_ZNK4QUrl11toLocalFileEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QUrl::detach();
impl /*struct*/ QUrl {
  pub fn detach<RetType, T: QUrl_detach<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.detach(self);
    // return 1;
  }
}

pub trait QUrl_detach<RetType> {
  fn detach(self , rsthis: &mut QUrl) -> RetType;
}

// proto:  void QUrl::detach();
impl<'a> /*trait*/ QUrl_detach<()> for () {
  fn detach(self , rsthis: &mut QUrl) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl6detachEv()};
     unsafe {_ZN4QUrl6detachEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  bool QUrl::hasFragment();
impl /*struct*/ QUrl {
  pub fn hasFragment<RetType, T: QUrl_hasFragment<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.hasFragment(self);
    // return 1;
  }
}

pub trait QUrl_hasFragment<RetType> {
  fn hasFragment(self , rsthis: &mut QUrl) -> RetType;
}

// proto:  bool QUrl::hasFragment();
impl<'a> /*trait*/ QUrl_hasFragment<i8> for () {
  fn hasFragment(self , rsthis: &mut QUrl) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl11hasFragmentEv()};
    let mut ret = unsafe {_ZNK4QUrl11hasFragmentEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: static QByteArray QUrl::toAce(const QString & );
impl /*struct*/ QUrl {
  pub fn toAce_s<RetType, T: QUrl_toAce_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.toAce_s();
    // return 1;
  }
}

pub trait QUrl_toAce_s<RetType> {
  fn toAce_s(self ) -> RetType;
}

// proto: static QByteArray QUrl::toAce(const QString & );
impl<'a> /*trait*/ QUrl_toAce_s<QByteArray> for (&'a  QString) {
  fn toAce_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl5toAceERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QUrl5toAceERK7QString(arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QUrl::hasQuery();
impl /*struct*/ QUrl {
  pub fn hasQuery<RetType, T: QUrl_hasQuery<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.hasQuery(self);
    // return 1;
  }
}

pub trait QUrl_hasQuery<RetType> {
  fn hasQuery(self , rsthis: &mut QUrl) -> RetType;
}

// proto:  bool QUrl::hasQuery();
impl<'a> /*trait*/ QUrl_hasQuery<i8> for () {
  fn hasQuery(self , rsthis: &mut QUrl) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl8hasQueryEv()};
    let mut ret = unsafe {_ZNK4QUrl8hasQueryEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: static QUrl QUrl::fromLocalFile(const QString & localfile);
impl /*struct*/ QUrl {
  pub fn fromLocalFile_s<RetType, T: QUrl_fromLocalFile_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromLocalFile_s();
    // return 1;
  }
}

pub trait QUrl_fromLocalFile_s<RetType> {
  fn fromLocalFile_s(self ) -> RetType;
}

// proto: static QUrl QUrl::fromLocalFile(const QString & localfile);
impl<'a> /*trait*/ QUrl_fromLocalFile_s<QUrl> for (&'a  QString) {
  fn fromLocalFile_s(self ) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl13fromLocalFileERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QUrl13fromLocalFileERK7QString(arg0)};
    let mut ret1 = QUrl{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QUrl::isValid();
impl /*struct*/ QUrl {
  pub fn isValid<RetType, T: QUrl_isValid<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QUrl_isValid<RetType> {
  fn isValid(self , rsthis: &mut QUrl) -> RetType;
}

// proto:  bool QUrl::isValid();
impl<'a> /*trait*/ QUrl_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QUrl) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl7isValidEv()};
    let mut ret = unsafe {_ZNK4QUrl7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QUrl::NewQUrl();
impl<'a> /*trait*/ QUrl_NewQUrl for () {
  fn NewQUrl(self) -> QUrl {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrlC1Ev()};
    unsafe {_ZN4QUrlC1Ev(qthis)};
    let rsthis = QUrl{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  bool QUrl::isDetached();
impl /*struct*/ QUrl {
  pub fn isDetached<RetType, T: QUrl_isDetached<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isDetached(self);
    // return 1;
  }
}

pub trait QUrl_isDetached<RetType> {
  fn isDetached(self , rsthis: &mut QUrl) -> RetType;
}

// proto:  bool QUrl::isDetached();
impl<'a> /*trait*/ QUrl_isDetached<i8> for () {
  fn isDetached(self , rsthis: &mut QUrl) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl10isDetachedEv()};
    let mut ret = unsafe {_ZNK4QUrl10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  bool QUrl::isRelative();
impl /*struct*/ QUrl {
  pub fn isRelative<RetType, T: QUrl_isRelative<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isRelative(self);
    // return 1;
  }
}

pub trait QUrl_isRelative<RetType> {
  fn isRelative(self , rsthis: &mut QUrl) -> RetType;
}

// proto:  bool QUrl::isRelative();
impl<'a> /*trait*/ QUrl_isRelative<i8> for () {
  fn isRelative(self , rsthis: &mut QUrl) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl10isRelativeEv()};
    let mut ret = unsafe {_ZNK4QUrl10isRelativeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QString QUrl::scheme();
impl /*struct*/ QUrl {
  pub fn scheme<RetType, T: QUrl_scheme<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.scheme(self);
    // return 1;
  }
}

pub trait QUrl_scheme<RetType> {
  fn scheme(self , rsthis: &mut QUrl) -> RetType;
}

// proto:  QString QUrl::scheme();
impl<'a> /*trait*/ QUrl_scheme<QString> for () {
  fn scheme(self , rsthis: &mut QUrl) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl6schemeEv()};
    let mut ret = unsafe {_ZNK4QUrl6schemeEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QByteArray QUrl::toPercentEncoding(const QString & , const QByteArray & exclude, const QByteArray & include);
impl /*struct*/ QUrl {
  pub fn toPercentEncoding_s<RetType, T: QUrl_toPercentEncoding_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.toPercentEncoding_s();
    // return 1;
  }
}

pub trait QUrl_toPercentEncoding_s<RetType> {
  fn toPercentEncoding_s(self ) -> RetType;
}

// proto: static QByteArray QUrl::toPercentEncoding(const QString & , const QByteArray & exclude, const QByteArray & include);
impl<'a> /*trait*/ QUrl_toPercentEncoding_s<QByteArray> for (&'a  QString, &'a  QByteArray, &'a  QByteArray) {
  fn toPercentEncoding_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl17toPercentEncodingERK7QStringRK10QByteArrayS5_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QUrl17toPercentEncodingERK7QStringRK10QByteArrayS5_(arg0, arg1, arg2)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static void QUrl::setIdnWhitelist(const QStringList & );
impl /*struct*/ QUrl {
  pub fn setIdnWhitelist_s<RetType, T: QUrl_setIdnWhitelist_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setIdnWhitelist_s();
    // return 1;
  }
}

pub trait QUrl_setIdnWhitelist_s<RetType> {
  fn setIdnWhitelist_s(self ) -> RetType;
}

// proto: static void QUrl::setIdnWhitelist(const QStringList & );
impl<'a> /*trait*/ QUrl_setIdnWhitelist_s<()> for (&'a  QStringList) {
  fn setIdnWhitelist_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl15setIdnWhitelistERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QUrl15setIdnWhitelistERK11QStringList(arg0)};
    // return 1;
  }
}

// proto:  void QUrl::swap(QUrl & other);
impl /*struct*/ QUrl {
  pub fn swap<RetType, T: QUrl_swap<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QUrl_swap<RetType> {
  fn swap(self , rsthis: &mut QUrl) -> RetType;
}

// proto:  void QUrl::swap(QUrl & other);
impl<'a> /*trait*/ QUrl_swap<()> for (&'a mut QUrl) {
  fn swap(self , rsthis: &mut QUrl) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QUrl4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: static QString QUrl::fromPercentEncoding(const QByteArray & );
impl /*struct*/ QUrl {
  pub fn fromPercentEncoding_s<RetType, T: QUrl_fromPercentEncoding_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromPercentEncoding_s();
    // return 1;
  }
}

pub trait QUrl_fromPercentEncoding_s<RetType> {
  fn fromPercentEncoding_s(self ) -> RetType;
}

// proto: static QString QUrl::fromPercentEncoding(const QByteArray & );
impl<'a> /*trait*/ QUrl_fromPercentEncoding_s<QString> for (&'a  QByteArray) {
  fn fromPercentEncoding_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl19fromPercentEncodingERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QUrl19fromPercentEncodingERK10QByteArray(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QUrl QUrl::fromUserInput(const QString & userInput);
impl /*struct*/ QUrl {
  pub fn fromUserInput_s<RetType, T: QUrl_fromUserInput_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromUserInput_s();
    // return 1;
  }
}

pub trait QUrl_fromUserInput_s<RetType> {
  fn fromUserInput_s(self ) -> RetType;
}

// proto: static QUrl QUrl::fromUserInput(const QString & userInput);
impl<'a> /*trait*/ QUrl_fromUserInput_s<QUrl> for (&'a  QString) {
  fn fromUserInput_s(self ) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl13fromUserInputERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QUrl13fromUserInputERK7QString(arg0)};
    let mut ret1 = QUrl{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QUrl::clear();
impl /*struct*/ QUrl {
  pub fn clear<RetType, T: QUrl_clear<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QUrl_clear<RetType> {
  fn clear(self , rsthis: &mut QUrl) -> RetType;
}

// proto:  void QUrl::clear();
impl<'a> /*trait*/ QUrl_clear<()> for () {
  fn clear(self , rsthis: &mut QUrl) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl5clearEv()};
     unsafe {_ZN4QUrl5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

