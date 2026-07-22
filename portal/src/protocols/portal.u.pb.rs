const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.34.0-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__portal__v1__IdRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct IdRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<IdRequest>
}

impl ::protobuf::Message for IdRequest {}

impl ::std::default::Default for IdRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for IdRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `IdRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `IdRequestMut`.
unsafe impl Sync for IdRequest {}

// SAFETY:
// - `IdRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for IdRequest {}

impl ::protobuf::Proxied for IdRequest {
  type View<'msg> = IdRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for IdRequest {}

impl ::protobuf::MutProxied for IdRequest {
  type Mut<'msg> = IdRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct IdRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, IdRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for IdRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for IdRequestView<'msg> {
  type Message = IdRequest;
}

impl ::std::fmt::Debug for IdRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for IdRequestView<'_> {
  fn default() -> IdRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, IdRequest>> for IdRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, IdRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> IdRequestView<'msg> {

  pub fn to_owned(&self) -> IdRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // id: optional int64
  pub fn id(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `IdRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for IdRequestView<'_> {}

// SAFETY:
// - `IdRequestView` is `Send` because while its alive a `IdRequestMut` cannot.
// - `IdRequestView` does not use thread-local data.
unsafe impl Send for IdRequestView<'_> {}

impl<'msg> ::protobuf::AsView for IdRequestView<'msg> {
  type Proxied = IdRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, IdRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for IdRequestView<'msg> {
  fn into_view<'shorter>(self) -> IdRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<IdRequest> for IdRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> IdRequest {
    let mut dst = IdRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<IdRequest> for IdRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> IdRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for IdRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for IdRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for IdRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct IdRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, IdRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for IdRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for IdRequestMut<'msg> {
  type Message = IdRequest;
}

impl ::std::fmt::Debug for IdRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, IdRequest>> for IdRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, IdRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> IdRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, IdRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> IdRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // id: optional int64
  pub fn id(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_id(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        0, val.into()
      )
    }
  }

}

// SAFETY:
// - `IdRequestMut` does not perform any shared mutation.
unsafe impl Send for IdRequestMut<'_> {}

// SAFETY:
// - `IdRequestMut` does not perform any shared mutation.
unsafe impl Sync for IdRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for IdRequestMut<'msg> {
  type Proxied = IdRequest;
  fn as_view(&self) -> ::protobuf::View<'_, IdRequest> {
    IdRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for IdRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, IdRequest>
  where
      'msg: 'shorter {
    IdRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for IdRequestMut<'msg> {
  type MutProxied = IdRequest;
  fn as_mut(&mut self) -> IdRequestMut<'msg> {
    IdRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for IdRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> IdRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl IdRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, IdRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> IdRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> IdRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // id: optional int64
  pub fn id(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_id(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        0, val.into()
      )
    }
  }

}  // impl IdRequest

impl ::std::ops::Drop for IdRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for IdRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for IdRequest {
  type Proxied = Self;
  fn as_view(&self) -> IdRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for IdRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> IdRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for IdRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__portal__v1__IdRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$+P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__portal__v1__IdRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__portal__v1__IdRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for IdRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for IdRequest {
  type Msg = IdRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IdRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for IdRequest {
  type Msg = IdRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IdRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for IdRequestMut<'_> {
  type Msg = IdRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IdRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for IdRequestMut<'_> {
  type Msg = IdRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IdRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for IdRequestView<'_> {
  type Msg = IdRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IdRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for IdRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__portal__v1__Page_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Page {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Page>
}

impl ::protobuf::Message for Page {}

impl ::std::default::Default for Page {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Page {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Page` is `Sync` because it does not implement interior mutability.
//    Neither does `PageMut`.
unsafe impl Sync for Page {}

// SAFETY:
// - `Page` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Page {}

impl ::protobuf::Proxied for Page {
  type View<'msg> = PageView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Page {}

impl ::protobuf::MutProxied for Page {
  type Mut<'msg> = PageMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PageView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Page>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PageView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PageView<'msg> {
  type Message = Page;
}

impl ::std::fmt::Debug for PageView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PageView<'_> {
  fn default() -> PageView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Page>> for PageView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Page>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PageView<'msg> {

  pub fn to_owned(&self) -> Page {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // index: optional int64
  pub fn index(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // size: optional int64
  pub fn size(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `PageView` is `Sync` because it does not support mutation.
unsafe impl Sync for PageView<'_> {}

// SAFETY:
// - `PageView` is `Send` because while its alive a `PageMut` cannot.
// - `PageView` does not use thread-local data.
unsafe impl Send for PageView<'_> {}

impl<'msg> ::protobuf::AsView for PageView<'msg> {
  type Proxied = Page;
  fn as_view(&self) -> ::protobuf::View<'msg, Page> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PageView<'msg> {
  fn into_view<'shorter>(self) -> PageView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Page> for PageView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Page {
    let mut dst = Page::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Page> for PageMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Page {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Page {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PageView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PageMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PageMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Page>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PageMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PageMut<'msg> {
  type Message = Page;
}

impl ::std::fmt::Debug for PageMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Page>> for PageMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Page>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PageMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Page> {
    self.inner
  }

  pub fn to_owned(&self) -> Page {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // index: optional int64
  pub fn index(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_index(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        0, val.into()
      )
    }
  }

  // size: optional int64
  pub fn size(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_size(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `PageMut` does not perform any shared mutation.
unsafe impl Send for PageMut<'_> {}

// SAFETY:
// - `PageMut` does not perform any shared mutation.
unsafe impl Sync for PageMut<'_> {}

impl<'msg> ::protobuf::AsView for PageMut<'msg> {
  type Proxied = Page;
  fn as_view(&self) -> ::protobuf::View<'_, Page> {
    PageView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PageMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Page>
  where
      'msg: 'shorter {
    PageView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for PageMut<'msg> {
  type MutProxied = Page;
  fn as_mut(&mut self) -> PageMut<'msg> {
    PageMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PageMut<'msg> {
  fn into_mut<'shorter>(self) -> PageMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Page {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Page> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PageView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PageMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // index: optional int64
  pub fn index(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_index(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        0, val.into()
      )
    }
  }

  // size: optional int64
  pub fn size(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_size(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        1, val.into()
      )
    }
  }

}  // impl Page

impl ::std::ops::Drop for Page {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Page {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Page {
  type Proxied = Self;
  fn as_view(&self) -> PageView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Page {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PageMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Page {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__portal__v1__Page_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$+P+P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__portal__v1__Page_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__portal__v1__Page_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Page {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Page {
  type Msg = Page;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Page> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Page {
  type Msg = Page;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Page> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PageMut<'_> {
  type Msg = Page;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Page> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PageMut<'_> {
  type Msg = Page;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Page> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PageView<'_> {
  type Msg = Page;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Page> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PageMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__portal__v1__Pagination_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Pagination {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Pagination>
}

impl ::protobuf::Message for Pagination {}

impl ::std::default::Default for Pagination {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Pagination {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Pagination` is `Sync` because it does not implement interior mutability.
//    Neither does `PaginationMut`.
unsafe impl Sync for Pagination {}

// SAFETY:
// - `Pagination` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Pagination {}

impl ::protobuf::Proxied for Pagination {
  type View<'msg> = PaginationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Pagination {}

impl ::protobuf::MutProxied for Pagination {
  type Mut<'msg> = PaginationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PaginationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Pagination>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PaginationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PaginationView<'msg> {
  type Message = Pagination;
}

impl ::std::fmt::Debug for PaginationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PaginationView<'_> {
  fn default() -> PaginationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Pagination>> for PaginationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Pagination>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PaginationView<'msg> {

  pub fn to_owned(&self) -> Pagination {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // current: optional message palm.portal.v1.Page
  pub fn has_current(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn current_opt(self) -> ::protobuf::Optional<super::PageView<'msg>> {
        ::protobuf::Optional::new(self.current(), self.has_current())
  }
  pub fn current(self) -> super::PageView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PageView::default())
  }

  // has_previous: optional bool
  pub fn has_previous(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }

  // has_next: optional bool
  pub fn has_next(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }

  // pages: optional int64
  pub fn pages(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // total: optional int64
  pub fn total(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        4, (0i64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `PaginationView` is `Sync` because it does not support mutation.
unsafe impl Sync for PaginationView<'_> {}

// SAFETY:
// - `PaginationView` is `Send` because while its alive a `PaginationMut` cannot.
// - `PaginationView` does not use thread-local data.
unsafe impl Send for PaginationView<'_> {}

impl<'msg> ::protobuf::AsView for PaginationView<'msg> {
  type Proxied = Pagination;
  fn as_view(&self) -> ::protobuf::View<'msg, Pagination> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PaginationView<'msg> {
  fn into_view<'shorter>(self) -> PaginationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Pagination> for PaginationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Pagination {
    let mut dst = Pagination::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Pagination> for PaginationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Pagination {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Pagination {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PaginationView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PaginationMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PaginationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Pagination>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PaginationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PaginationMut<'msg> {
  type Message = Pagination;
}

impl ::std::fmt::Debug for PaginationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Pagination>> for PaginationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Pagination>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PaginationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Pagination> {
    self.inner
  }

  pub fn to_owned(&self) -> Pagination {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // current: optional message palm.portal.v1.Page
  pub fn has_current(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_current(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn current_opt(&self) -> ::protobuf::Optional<super::PageView<'_>> {
        ::protobuf::Optional::new(self.current(), self.has_current())
  }
  pub fn current(&self) -> super::PageView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PageView::default())
  }
  pub fn current_mut(&mut self) -> super::PageMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_current(&mut self,
    val: impl ::protobuf::IntoProxied<super::Page>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // has_previous: optional bool
  pub fn has_previous(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_has_previous(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

  // has_next: optional bool
  pub fn has_next(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_has_next(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        2, val.into()
      )
    }
  }

  // pages: optional int64
  pub fn pages(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_pages(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        3, val.into()
      )
    }
  }

  // total: optional int64
  pub fn total(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        4, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        4, val.into()
      )
    }
  }

}

// SAFETY:
// - `PaginationMut` does not perform any shared mutation.
unsafe impl Send for PaginationMut<'_> {}

// SAFETY:
// - `PaginationMut` does not perform any shared mutation.
unsafe impl Sync for PaginationMut<'_> {}

impl<'msg> ::protobuf::AsView for PaginationMut<'msg> {
  type Proxied = Pagination;
  fn as_view(&self) -> ::protobuf::View<'_, Pagination> {
    PaginationView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PaginationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Pagination>
  where
      'msg: 'shorter {
    PaginationView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for PaginationMut<'msg> {
  type MutProxied = Pagination;
  fn as_mut(&mut self) -> PaginationMut<'msg> {
    PaginationMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PaginationMut<'msg> {
  fn into_mut<'shorter>(self) -> PaginationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Pagination {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Pagination> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PaginationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PaginationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // current: optional message palm.portal.v1.Page
  pub fn has_current(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_current(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn current_opt(&self) -> ::protobuf::Optional<super::PageView<'_>> {
        ::protobuf::Optional::new(self.current(), self.has_current())
  }
  pub fn current(&self) -> super::PageView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PageView::default())
  }
  pub fn current_mut(&mut self) -> super::PageMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_current(&mut self,
    val: impl ::protobuf::IntoProxied<super::Page>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // has_previous: optional bool
  pub fn has_previous(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_has_previous(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

  // has_next: optional bool
  pub fn has_next(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_has_next(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        2, val.into()
      )
    }
  }

  // pages: optional int64
  pub fn pages(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_pages(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        3, val.into()
      )
    }
  }

  // total: optional int64
  pub fn total(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        4, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        4, val.into()
      )
    }
  }

}  // impl Pagination

impl ::std::ops::Drop for Pagination {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Pagination {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Pagination {
  type Proxied = Self;
  fn as_view(&self) -> PaginationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Pagination {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PaginationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Pagination {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__portal__v1__Pagination_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3/P/P+P+P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__portal__v1__Pagination_msg_init.0, &[<super::Page as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__portal__v1__Pagination_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Pagination {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Pagination {
  type Msg = Pagination;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Pagination> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Pagination {
  type Msg = Pagination;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Pagination> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PaginationMut<'_> {
  type Msg = Pagination;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Pagination> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PaginationMut<'_> {
  type Msg = Pagination;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Pagination> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PaginationView<'_> {
  type Msg = Pagination;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Pagination> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PaginationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__portal__v1__File_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct File {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<File>
}

impl ::protobuf::Message for File {}

impl ::std::default::Default for File {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for File {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `File` is `Sync` because it does not implement interior mutability.
//    Neither does `FileMut`.
unsafe impl Sync for File {}

// SAFETY:
// - `File` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for File {}

impl ::protobuf::Proxied for File {
  type View<'msg> = FileView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for File {}

impl ::protobuf::MutProxied for File {
  type Mut<'msg> = FileMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FileView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, File>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FileView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FileView<'msg> {
  type Message = File;
}

impl ::std::fmt::Debug for FileView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FileView<'_> {
  fn default() -> FileView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, File>> for FileView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, File>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FileView<'msg> {

  pub fn to_owned(&self) -> File {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // bucket: optional string
  pub fn bucket(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // object: optional string
  pub fn object(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `FileView` is `Sync` because it does not support mutation.
unsafe impl Sync for FileView<'_> {}

// SAFETY:
// - `FileView` is `Send` because while its alive a `FileMut` cannot.
// - `FileView` does not use thread-local data.
unsafe impl Send for FileView<'_> {}

impl<'msg> ::protobuf::AsView for FileView<'msg> {
  type Proxied = File;
  fn as_view(&self) -> ::protobuf::View<'msg, File> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FileView<'msg> {
  fn into_view<'shorter>(self) -> FileView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<File> for FileView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> File {
    let mut dst = File::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<File> for FileMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> File {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for File {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for FileView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for FileMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FileMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, File>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FileMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FileMut<'msg> {
  type Message = File;
}

impl ::std::fmt::Debug for FileMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, File>> for FileMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, File>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FileMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, File> {
    self.inner
  }

  pub fn to_owned(&self) -> File {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // bucket: optional string
  pub fn bucket(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_bucket(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // object: optional string
  pub fn object(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_object(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `FileMut` does not perform any shared mutation.
unsafe impl Send for FileMut<'_> {}

// SAFETY:
// - `FileMut` does not perform any shared mutation.
unsafe impl Sync for FileMut<'_> {}

impl<'msg> ::protobuf::AsView for FileMut<'msg> {
  type Proxied = File;
  fn as_view(&self) -> ::protobuf::View<'_, File> {
    FileView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FileMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, File>
  where
      'msg: 'shorter {
    FileView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for FileMut<'msg> {
  type MutProxied = File;
  fn as_mut(&mut self) -> FileMut<'msg> {
    FileMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FileMut<'msg> {
  fn into_mut<'shorter>(self) -> FileMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl File {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, File> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FileView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FileMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // bucket: optional string
  pub fn bucket(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_bucket(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // object: optional string
  pub fn object(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_object(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl File

impl ::std::ops::Drop for File {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for File {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for File {
  type Proxied = Self;
  fn as_view(&self) -> FileView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for File {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FileMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for File {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__portal__v1__File_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__portal__v1__File_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__portal__v1__File_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for File {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for File {
  type Msg = File;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<File> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for File {
  type Msg = File;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<File> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FileMut<'_> {
  type Msg = File;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<File> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FileMut<'_> {
  type Msg = File;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<File> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FileView<'_> {
  type Msg = File;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<File> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FileMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__portal__v1__RichText_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RichText {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RichText>
}

impl ::protobuf::Message for RichText {}

impl ::std::default::Default for RichText {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RichText {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RichText` is `Sync` because it does not implement interior mutability.
//    Neither does `RichTextMut`.
unsafe impl Sync for RichText {}

// SAFETY:
// - `RichText` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for RichText {}

impl ::protobuf::Proxied for RichText {
  type View<'msg> = RichTextView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RichText {}

impl ::protobuf::MutProxied for RichText {
  type Mut<'msg> = RichTextMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RichTextView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RichText>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RichTextView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RichTextView<'msg> {
  type Message = RichText;
}

impl ::std::fmt::Debug for RichTextView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RichTextView<'_> {
  fn default() -> RichTextView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RichText>> for RichTextView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RichText>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RichTextView<'msg> {

  pub fn to_owned(&self) -> RichText {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // editor: optional enum palm.portal.v1.RichText.Editor
  pub fn editor(self) -> super::rich_text::Editor {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::rich_text::Editor::CkEditor).into()
      ).try_into().unwrap()
    }
  }

  // body: optional string
  pub fn body(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // attachments: repeated message palm.portal.v1.File
  pub fn attachments(self) -> ::protobuf::RepeatedView<'msg, super::File> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::File>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `RichTextView` is `Sync` because it does not support mutation.
unsafe impl Sync for RichTextView<'_> {}

// SAFETY:
// - `RichTextView` is `Send` because while its alive a `RichTextMut` cannot.
// - `RichTextView` does not use thread-local data.
unsafe impl Send for RichTextView<'_> {}

impl<'msg> ::protobuf::AsView for RichTextView<'msg> {
  type Proxied = RichText;
  fn as_view(&self) -> ::protobuf::View<'msg, RichText> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RichTextView<'msg> {
  fn into_view<'shorter>(self) -> RichTextView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RichText> for RichTextView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RichText {
    let mut dst = RichText::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RichText> for RichTextMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RichText {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for RichText {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for RichTextView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for RichTextMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RichTextMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RichText>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RichTextMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RichTextMut<'msg> {
  type Message = RichText;
}

impl ::std::fmt::Debug for RichTextMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RichText>> for RichTextMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RichText>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RichTextMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RichText> {
    self.inner
  }

  pub fn to_owned(&self) -> RichText {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // editor: optional enum palm.portal.v1.RichText.Editor
  pub fn editor(&self) -> super::rich_text::Editor {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::rich_text::Editor::CkEditor).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_editor(&mut self, val: super::rich_text::Editor) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // body: optional string
  pub fn body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // attachments: repeated message palm.portal.v1.File
  pub fn attachments(&self) -> ::protobuf::RepeatedView<'_, super::File> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::File>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn attachments_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::File> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        2,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_attachments(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::File>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}

// SAFETY:
// - `RichTextMut` does not perform any shared mutation.
unsafe impl Send for RichTextMut<'_> {}

// SAFETY:
// - `RichTextMut` does not perform any shared mutation.
unsafe impl Sync for RichTextMut<'_> {}

impl<'msg> ::protobuf::AsView for RichTextMut<'msg> {
  type Proxied = RichText;
  fn as_view(&self) -> ::protobuf::View<'_, RichText> {
    RichTextView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RichTextMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RichText>
  where
      'msg: 'shorter {
    RichTextView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for RichTextMut<'msg> {
  type MutProxied = RichText;
  fn as_mut(&mut self) -> RichTextMut<'msg> {
    RichTextMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RichTextMut<'msg> {
  fn into_mut<'shorter>(self) -> RichTextMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RichText {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RichText> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RichTextView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RichTextMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // editor: optional enum palm.portal.v1.RichText.Editor
  pub fn editor(&self) -> super::rich_text::Editor {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::rich_text::Editor::CkEditor).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_editor(&mut self, val: super::rich_text::Editor) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // body: optional string
  pub fn body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // attachments: repeated message palm.portal.v1.File
  pub fn attachments(&self) -> ::protobuf::RepeatedView<'_, super::File> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::File>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn attachments_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::File> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        2,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_attachments(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::File>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}  // impl RichText

impl ::std::ops::Drop for RichText {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RichText {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RichText {
  type Proxied = Self;
  fn as_view(&self) -> RichTextView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RichText {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RichTextMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RichText {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__portal__v1__RichText_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P1XG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__portal__v1__RichText_msg_init.0, &[<super::File as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__portal__v1__RichText_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RichText {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RichText {
  type Msg = RichText;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RichText> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RichText {
  type Msg = RichText;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RichText> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RichTextMut<'_> {
  type Msg = RichText;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RichText> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RichTextMut<'_> {
  type Msg = RichText;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RichText> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RichTextView<'_> {
  type Msg = RichText;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RichText> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RichTextMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod rich_text {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Editor(i32);

#[allow(non_upper_case_globals)]
impl Editor {
  pub const CkEditor: Editor = Editor(0);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "CkEditor",
      _ => return None
    })
  }
}

impl ::std::convert::From<Editor> for i32 {
  fn from(val: Editor) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for Editor {
  fn from(val: i32) -> Editor {
    Self(val)
  }
}

impl ::std::default::Default for Editor {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for Editor {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "Editor::{}", constant_name)
    } else {
      write!(f, "Editor::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for Editor {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for Editor {}

impl ::protobuf::Proxied for Editor {
  type View<'a> = Editor;
}

impl ::protobuf::AsView for Editor {
  type Proxied = Editor;

  fn as_view(&self) -> Editor {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Editor {
  fn into_view<'shorter>(self) -> Editor where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for Editor {
  const NAME: &'static str = "Editor";

  fn is_known(value: i32) -> bool {
    matches!(value, 0)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Editor {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


}  // pub mod rich_text


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__portal__v1__Location_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Location {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Location>
}

impl ::protobuf::Message for Location {}

impl ::std::default::Default for Location {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Location {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Location` is `Sync` because it does not implement interior mutability.
//    Neither does `LocationMut`.
unsafe impl Sync for Location {}

// SAFETY:
// - `Location` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Location {}

impl ::protobuf::Proxied for Location {
  type View<'msg> = LocationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Location {}

impl ::protobuf::MutProxied for Location {
  type Mut<'msg> = LocationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LocationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Location>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LocationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LocationView<'msg> {
  type Message = Location;
}

impl ::std::fmt::Debug for LocationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LocationView<'_> {
  fn default() -> LocationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Location>> for LocationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Location>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LocationView<'msg> {

  pub fn to_owned(&self) -> Location {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // map: optional enum palm.portal.v1.Location.Map
  pub fn map(self) -> super::location::Map {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::location::Map::Google).into()
      ).try_into().unwrap()
    }
  }

  // address: optional string
  pub fn address(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `LocationView` is `Sync` because it does not support mutation.
unsafe impl Sync for LocationView<'_> {}

// SAFETY:
// - `LocationView` is `Send` because while its alive a `LocationMut` cannot.
// - `LocationView` does not use thread-local data.
unsafe impl Send for LocationView<'_> {}

impl<'msg> ::protobuf::AsView for LocationView<'msg> {
  type Proxied = Location;
  fn as_view(&self) -> ::protobuf::View<'msg, Location> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LocationView<'msg> {
  fn into_view<'shorter>(self) -> LocationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Location> for LocationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Location {
    let mut dst = Location::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Location> for LocationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Location {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Location {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for LocationView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for LocationMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LocationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Location>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LocationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LocationMut<'msg> {
  type Message = Location;
}

impl ::std::fmt::Debug for LocationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Location>> for LocationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Location>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LocationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Location> {
    self.inner
  }

  pub fn to_owned(&self) -> Location {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // map: optional enum palm.portal.v1.Location.Map
  pub fn map(&self) -> super::location::Map {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::location::Map::Google).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_map(&mut self, val: super::location::Map) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // address: optional string
  pub fn address(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_address(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `LocationMut` does not perform any shared mutation.
unsafe impl Send for LocationMut<'_> {}

// SAFETY:
// - `LocationMut` does not perform any shared mutation.
unsafe impl Sync for LocationMut<'_> {}

impl<'msg> ::protobuf::AsView for LocationMut<'msg> {
  type Proxied = Location;
  fn as_view(&self) -> ::protobuf::View<'_, Location> {
    LocationView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LocationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Location>
  where
      'msg: 'shorter {
    LocationView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for LocationMut<'msg> {
  type MutProxied = Location;
  fn as_mut(&mut self) -> LocationMut<'msg> {
    LocationMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LocationMut<'msg> {
  fn into_mut<'shorter>(self) -> LocationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Location {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Location> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LocationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LocationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // map: optional enum palm.portal.v1.Location.Map
  pub fn map(&self) -> super::location::Map {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::location::Map::Google).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_map(&mut self, val: super::location::Map) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // address: optional string
  pub fn address(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_address(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl Location

impl ::std::ops::Drop for Location {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Location {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Location {
  type Proxied = Self;
  fn as_view(&self) -> LocationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Location {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LocationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Location {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__portal__v1__Location_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__portal__v1__Location_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__portal__v1__Location_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Location {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Location {
  type Msg = Location;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Location> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Location {
  type Msg = Location;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Location> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LocationMut<'_> {
  type Msg = Location;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Location> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocationMut<'_> {
  type Msg = Location;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Location> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocationView<'_> {
  type Msg = Location;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Location> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LocationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod location {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Map(i32);

#[allow(non_upper_case_globals)]
impl Map {
  pub const Google: Map = Map(0);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Google",
      _ => return None
    })
  }
}

impl ::std::convert::From<Map> for i32 {
  fn from(val: Map) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for Map {
  fn from(val: i32) -> Map {
    Self(val)
  }
}

impl ::std::default::Default for Map {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for Map {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "Map::{}", constant_name)
    } else {
      write!(f, "Map::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for Map {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for Map {}

impl ::protobuf::Proxied for Map {
  type View<'a> = Map;
}

impl ::protobuf::AsView for Map {
  type Proxied = Map;

  fn as_view(&self) -> Map {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Map {
  fn into_view<'shorter>(self) -> Map where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for Map {
  const NAME: &'static str = "Map";

  fn is_known(value: i32) -> bool {
    matches!(value, 0)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Map {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


}  // pub mod location


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__portal__v1__SiteHeartbeatResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SiteHeartbeatResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SiteHeartbeatResponse>
}

impl ::protobuf::Message for SiteHeartbeatResponse {}

impl ::std::default::Default for SiteHeartbeatResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SiteHeartbeatResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SiteHeartbeatResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `SiteHeartbeatResponseMut`.
unsafe impl Sync for SiteHeartbeatResponse {}

// SAFETY:
// - `SiteHeartbeatResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SiteHeartbeatResponse {}

impl ::protobuf::Proxied for SiteHeartbeatResponse {
  type View<'msg> = SiteHeartbeatResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SiteHeartbeatResponse {}

impl ::protobuf::MutProxied for SiteHeartbeatResponse {
  type Mut<'msg> = SiteHeartbeatResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SiteHeartbeatResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SiteHeartbeatResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SiteHeartbeatResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SiteHeartbeatResponseView<'msg> {
  type Message = SiteHeartbeatResponse;
}

impl ::std::fmt::Debug for SiteHeartbeatResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SiteHeartbeatResponseView<'_> {
  fn default() -> SiteHeartbeatResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SiteHeartbeatResponse>> for SiteHeartbeatResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SiteHeartbeatResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SiteHeartbeatResponseView<'msg> {

  pub fn to_owned(&self) -> SiteHeartbeatResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // version: optional string
  pub fn version(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // created_at: optional message google.protobuf.Timestamp
  pub fn has_created_at(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn created_at_opt(self) -> ::protobuf::Optional<super::TimestampView<'msg>> {
        ::protobuf::Optional::new(self.created_at(), self.has_created_at())
  }
  pub fn created_at(self) -> super::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TimestampView::default())
  }

}

// SAFETY:
// - `SiteHeartbeatResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for SiteHeartbeatResponseView<'_> {}

// SAFETY:
// - `SiteHeartbeatResponseView` is `Send` because while its alive a `SiteHeartbeatResponseMut` cannot.
// - `SiteHeartbeatResponseView` does not use thread-local data.
unsafe impl Send for SiteHeartbeatResponseView<'_> {}

impl<'msg> ::protobuf::AsView for SiteHeartbeatResponseView<'msg> {
  type Proxied = SiteHeartbeatResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, SiteHeartbeatResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SiteHeartbeatResponseView<'msg> {
  fn into_view<'shorter>(self) -> SiteHeartbeatResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SiteHeartbeatResponse> for SiteHeartbeatResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SiteHeartbeatResponse {
    let mut dst = SiteHeartbeatResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SiteHeartbeatResponse> for SiteHeartbeatResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SiteHeartbeatResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for SiteHeartbeatResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SiteHeartbeatResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SiteHeartbeatResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SiteHeartbeatResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SiteHeartbeatResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SiteHeartbeatResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SiteHeartbeatResponseMut<'msg> {
  type Message = SiteHeartbeatResponse;
}

impl ::std::fmt::Debug for SiteHeartbeatResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SiteHeartbeatResponse>> for SiteHeartbeatResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SiteHeartbeatResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SiteHeartbeatResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SiteHeartbeatResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> SiteHeartbeatResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // version: optional string
  pub fn version(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_version(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // created_at: optional message google.protobuf.Timestamp
  pub fn has_created_at(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_created_at(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn created_at_opt(&self) -> ::protobuf::Optional<super::TimestampView<'_>> {
        ::protobuf::Optional::new(self.created_at(), self.has_created_at())
  }
  pub fn created_at(&self) -> super::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TimestampView::default())
  }
  pub fn created_at_mut(&mut self) -> super::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_created_at(&mut self,
    val: impl ::protobuf::IntoProxied<super::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}

// SAFETY:
// - `SiteHeartbeatResponseMut` does not perform any shared mutation.
unsafe impl Send for SiteHeartbeatResponseMut<'_> {}

// SAFETY:
// - `SiteHeartbeatResponseMut` does not perform any shared mutation.
unsafe impl Sync for SiteHeartbeatResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for SiteHeartbeatResponseMut<'msg> {
  type Proxied = SiteHeartbeatResponse;
  fn as_view(&self) -> ::protobuf::View<'_, SiteHeartbeatResponse> {
    SiteHeartbeatResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SiteHeartbeatResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SiteHeartbeatResponse>
  where
      'msg: 'shorter {
    SiteHeartbeatResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for SiteHeartbeatResponseMut<'msg> {
  type MutProxied = SiteHeartbeatResponse;
  fn as_mut(&mut self) -> SiteHeartbeatResponseMut<'msg> {
    SiteHeartbeatResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SiteHeartbeatResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> SiteHeartbeatResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SiteHeartbeatResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SiteHeartbeatResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SiteHeartbeatResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SiteHeartbeatResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // version: optional string
  pub fn version(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_version(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // created_at: optional message google.protobuf.Timestamp
  pub fn has_created_at(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_created_at(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn created_at_opt(&self) -> ::protobuf::Optional<super::TimestampView<'_>> {
        ::protobuf::Optional::new(self.created_at(), self.has_created_at())
  }
  pub fn created_at(&self) -> super::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TimestampView::default())
  }
  pub fn created_at_mut(&mut self) -> super::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_created_at(&mut self,
    val: impl ::protobuf::IntoProxied<super::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl SiteHeartbeatResponse

impl ::std::ops::Drop for SiteHeartbeatResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SiteHeartbeatResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SiteHeartbeatResponse {
  type Proxied = Self;
  fn as_view(&self) -> SiteHeartbeatResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SiteHeartbeatResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SiteHeartbeatResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SiteHeartbeatResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__portal__v1__SiteHeartbeatResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__portal__v1__SiteHeartbeatResponse_msg_init.0, &[<super::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__portal__v1__SiteHeartbeatResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SiteHeartbeatResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SiteHeartbeatResponse {
  type Msg = SiteHeartbeatResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SiteHeartbeatResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SiteHeartbeatResponse {
  type Msg = SiteHeartbeatResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SiteHeartbeatResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SiteHeartbeatResponseMut<'_> {
  type Msg = SiteHeartbeatResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SiteHeartbeatResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SiteHeartbeatResponseMut<'_> {
  type Msg = SiteHeartbeatResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SiteHeartbeatResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SiteHeartbeatResponseView<'_> {
  type Msg = SiteHeartbeatResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SiteHeartbeatResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SiteHeartbeatResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



