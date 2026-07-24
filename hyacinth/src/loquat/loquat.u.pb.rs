const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.34.0-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__loquat__v1__Empty_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Empty {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Empty>
}

impl ::protobuf::Message for Empty {}

impl ::std::default::Default for Empty {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Empty {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Empty` is `Sync` because it does not implement interior mutability.
//    Neither does `EmptyMut`.
unsafe impl Sync for Empty {}

// SAFETY:
// - `Empty` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Empty {}

impl ::protobuf::Proxied for Empty {
  type View<'msg> = EmptyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Empty {}

impl ::protobuf::MutProxied for Empty {
  type Mut<'msg> = EmptyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct EmptyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Empty>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EmptyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for EmptyView<'msg> {
  type Message = Empty;
}

impl ::std::fmt::Debug for EmptyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for EmptyView<'_> {
  fn default() -> EmptyView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Empty>> for EmptyView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Empty>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EmptyView<'msg> {

  pub fn to_owned(&self) -> Empty {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `EmptyView` is `Sync` because it does not support mutation.
unsafe impl Sync for EmptyView<'_> {}

// SAFETY:
// - `EmptyView` is `Send` because while its alive a `EmptyMut` cannot.
// - `EmptyView` does not use thread-local data.
unsafe impl Send for EmptyView<'_> {}

impl<'msg> ::protobuf::AsView for EmptyView<'msg> {
  type Proxied = Empty;
  fn as_view(&self) -> ::protobuf::View<'msg, Empty> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EmptyView<'msg> {
  fn into_view<'shorter>(self) -> EmptyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Empty> for EmptyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Empty {
    let mut dst = Empty::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Empty> for EmptyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Empty {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Empty {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for EmptyView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for EmptyMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct EmptyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Empty>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EmptyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for EmptyMut<'msg> {
  type Message = Empty;
}

impl ::std::fmt::Debug for EmptyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Empty>> for EmptyMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Empty>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EmptyMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Empty> {
    self.inner
  }

  pub fn to_owned(&self) -> Empty {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `EmptyMut` does not perform any shared mutation.
unsafe impl Send for EmptyMut<'_> {}

// SAFETY:
// - `EmptyMut` does not perform any shared mutation.
unsafe impl Sync for EmptyMut<'_> {}

impl<'msg> ::protobuf::AsView for EmptyMut<'msg> {
  type Proxied = Empty;
  fn as_view(&self) -> ::protobuf::View<'_, Empty> {
    EmptyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EmptyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Empty>
  where
      'msg: 'shorter {
    EmptyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for EmptyMut<'msg> {
  type MutProxied = Empty;
  fn as_mut(&mut self) -> EmptyMut<'msg> {
    EmptyMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for EmptyMut<'msg> {
  fn into_mut<'shorter>(self) -> EmptyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Empty {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Empty> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> EmptyView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> EmptyMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl Empty

impl ::std::ops::Drop for Empty {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Empty {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Empty {
  type Proxied = Self;
  fn as_view(&self) -> EmptyView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Empty {
  type MutProxied = Self;
  fn as_mut(&mut self) -> EmptyMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Empty {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__loquat__v1__Empty_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__loquat__v1__Empty_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__loquat__v1__Empty_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Empty {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Empty {
  type Msg = Empty;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Empty> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Empty {
  type Msg = Empty;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Empty> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EmptyMut<'_> {
  type Msg = Empty;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Empty> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EmptyMut<'_> {
  type Msg = Empty;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Empty> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EmptyView<'_> {
  type Msg = Empty;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Empty> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EmptyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__loquat__v1__JwtVerifyRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct JwtVerifyRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<JwtVerifyRequest>
}

impl ::protobuf::Message for JwtVerifyRequest {}

impl ::std::default::Default for JwtVerifyRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for JwtVerifyRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `JwtVerifyRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `JwtVerifyRequestMut`.
unsafe impl Sync for JwtVerifyRequest {}

// SAFETY:
// - `JwtVerifyRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for JwtVerifyRequest {}

impl ::protobuf::Proxied for JwtVerifyRequest {
  type View<'msg> = JwtVerifyRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for JwtVerifyRequest {}

impl ::protobuf::MutProxied for JwtVerifyRequest {
  type Mut<'msg> = JwtVerifyRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct JwtVerifyRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JwtVerifyRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JwtVerifyRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for JwtVerifyRequestView<'msg> {
  type Message = JwtVerifyRequest;
}

impl ::std::fmt::Debug for JwtVerifyRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for JwtVerifyRequestView<'_> {
  fn default() -> JwtVerifyRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, JwtVerifyRequest>> for JwtVerifyRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JwtVerifyRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JwtVerifyRequestView<'msg> {

  pub fn to_owned(&self) -> JwtVerifyRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // token: optional string
  pub fn token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // issuer: optional string
  pub fn issuer(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // audience: optional string
  pub fn audience(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `JwtVerifyRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for JwtVerifyRequestView<'_> {}

// SAFETY:
// - `JwtVerifyRequestView` is `Send` because while its alive a `JwtVerifyRequestMut` cannot.
// - `JwtVerifyRequestView` does not use thread-local data.
unsafe impl Send for JwtVerifyRequestView<'_> {}

impl<'msg> ::protobuf::AsView for JwtVerifyRequestView<'msg> {
  type Proxied = JwtVerifyRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, JwtVerifyRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JwtVerifyRequestView<'msg> {
  fn into_view<'shorter>(self) -> JwtVerifyRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<JwtVerifyRequest> for JwtVerifyRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JwtVerifyRequest {
    let mut dst = JwtVerifyRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<JwtVerifyRequest> for JwtVerifyRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JwtVerifyRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for JwtVerifyRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for JwtVerifyRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for JwtVerifyRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct JwtVerifyRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JwtVerifyRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JwtVerifyRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for JwtVerifyRequestMut<'msg> {
  type Message = JwtVerifyRequest;
}

impl ::std::fmt::Debug for JwtVerifyRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, JwtVerifyRequest>> for JwtVerifyRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JwtVerifyRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JwtVerifyRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, JwtVerifyRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> JwtVerifyRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // token: optional string
  pub fn token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // issuer: optional string
  pub fn issuer(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_issuer(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // audience: optional string
  pub fn audience(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_audience(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `JwtVerifyRequestMut` does not perform any shared mutation.
unsafe impl Send for JwtVerifyRequestMut<'_> {}

// SAFETY:
// - `JwtVerifyRequestMut` does not perform any shared mutation.
unsafe impl Sync for JwtVerifyRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for JwtVerifyRequestMut<'msg> {
  type Proxied = JwtVerifyRequest;
  fn as_view(&self) -> ::protobuf::View<'_, JwtVerifyRequest> {
    JwtVerifyRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JwtVerifyRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, JwtVerifyRequest>
  where
      'msg: 'shorter {
    JwtVerifyRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for JwtVerifyRequestMut<'msg> {
  type MutProxied = JwtVerifyRequest;
  fn as_mut(&mut self) -> JwtVerifyRequestMut<'msg> {
    JwtVerifyRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for JwtVerifyRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> JwtVerifyRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl JwtVerifyRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, JwtVerifyRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> JwtVerifyRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> JwtVerifyRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // token: optional string
  pub fn token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // issuer: optional string
  pub fn issuer(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_issuer(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // audience: optional string
  pub fn audience(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_audience(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl JwtVerifyRequest

impl ::std::ops::Drop for JwtVerifyRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for JwtVerifyRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for JwtVerifyRequest {
  type Proxied = Self;
  fn as_view(&self) -> JwtVerifyRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for JwtVerifyRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> JwtVerifyRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for JwtVerifyRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__loquat__v1__JwtVerifyRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1P1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__loquat__v1__JwtVerifyRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__loquat__v1__JwtVerifyRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JwtVerifyRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JwtVerifyRequest {
  type Msg = JwtVerifyRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JwtVerifyRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JwtVerifyRequest {
  type Msg = JwtVerifyRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JwtVerifyRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JwtVerifyRequestMut<'_> {
  type Msg = JwtVerifyRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JwtVerifyRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JwtVerifyRequestMut<'_> {
  type Msg = JwtVerifyRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JwtVerifyRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JwtVerifyRequestView<'_> {
  type Msg = JwtVerifyRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JwtVerifyRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JwtVerifyRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__loquat__v1__JwtVerifyResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct JwtVerifyResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<JwtVerifyResponse>
}

impl ::protobuf::Message for JwtVerifyResponse {}

impl ::std::default::Default for JwtVerifyResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for JwtVerifyResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `JwtVerifyResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `JwtVerifyResponseMut`.
unsafe impl Sync for JwtVerifyResponse {}

// SAFETY:
// - `JwtVerifyResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for JwtVerifyResponse {}

impl ::protobuf::Proxied for JwtVerifyResponse {
  type View<'msg> = JwtVerifyResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for JwtVerifyResponse {}

impl ::protobuf::MutProxied for JwtVerifyResponse {
  type Mut<'msg> = JwtVerifyResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct JwtVerifyResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JwtVerifyResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JwtVerifyResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for JwtVerifyResponseView<'msg> {
  type Message = JwtVerifyResponse;
}

impl ::std::fmt::Debug for JwtVerifyResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for JwtVerifyResponseView<'_> {
  fn default() -> JwtVerifyResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, JwtVerifyResponse>> for JwtVerifyResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JwtVerifyResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JwtVerifyResponseView<'msg> {

  pub fn to_owned(&self) -> JwtVerifyResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // subject: optional string
  pub fn subject(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // payload: optional bytes
  pub fn has_payload(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn payload_opt(self) -> ::protobuf::Optional<&'msg [u8]> {
        ::protobuf::Optional::new(self.payload(), self.has_payload())
  }
  pub fn payload(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `JwtVerifyResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for JwtVerifyResponseView<'_> {}

// SAFETY:
// - `JwtVerifyResponseView` is `Send` because while its alive a `JwtVerifyResponseMut` cannot.
// - `JwtVerifyResponseView` does not use thread-local data.
unsafe impl Send for JwtVerifyResponseView<'_> {}

impl<'msg> ::protobuf::AsView for JwtVerifyResponseView<'msg> {
  type Proxied = JwtVerifyResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, JwtVerifyResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JwtVerifyResponseView<'msg> {
  fn into_view<'shorter>(self) -> JwtVerifyResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<JwtVerifyResponse> for JwtVerifyResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JwtVerifyResponse {
    let mut dst = JwtVerifyResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<JwtVerifyResponse> for JwtVerifyResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JwtVerifyResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for JwtVerifyResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for JwtVerifyResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for JwtVerifyResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct JwtVerifyResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JwtVerifyResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JwtVerifyResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for JwtVerifyResponseMut<'msg> {
  type Message = JwtVerifyResponse;
}

impl ::std::fmt::Debug for JwtVerifyResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, JwtVerifyResponse>> for JwtVerifyResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JwtVerifyResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JwtVerifyResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, JwtVerifyResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> JwtVerifyResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // subject: optional string
  pub fn subject(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subject(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // payload: optional bytes
  pub fn has_payload(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_payload(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn payload_opt(&self) -> ::protobuf::Optional<&'_ [u8]> {
        ::protobuf::Optional::new(self.payload(), self.has_payload())
  }
  pub fn payload(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_payload(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `JwtVerifyResponseMut` does not perform any shared mutation.
unsafe impl Send for JwtVerifyResponseMut<'_> {}

// SAFETY:
// - `JwtVerifyResponseMut` does not perform any shared mutation.
unsafe impl Sync for JwtVerifyResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for JwtVerifyResponseMut<'msg> {
  type Proxied = JwtVerifyResponse;
  fn as_view(&self) -> ::protobuf::View<'_, JwtVerifyResponse> {
    JwtVerifyResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JwtVerifyResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, JwtVerifyResponse>
  where
      'msg: 'shorter {
    JwtVerifyResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for JwtVerifyResponseMut<'msg> {
  type MutProxied = JwtVerifyResponse;
  fn as_mut(&mut self) -> JwtVerifyResponseMut<'msg> {
    JwtVerifyResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for JwtVerifyResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> JwtVerifyResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl JwtVerifyResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, JwtVerifyResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> JwtVerifyResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> JwtVerifyResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // subject: optional string
  pub fn subject(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subject(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // payload: optional bytes
  pub fn has_payload(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_payload(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn payload_opt(&self) -> ::protobuf::Optional<&'_ [u8]> {
        ::protobuf::Optional::new(self.payload(), self.has_payload())
  }
  pub fn payload(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_payload(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl JwtVerifyResponse

impl ::std::ops::Drop for JwtVerifyResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for JwtVerifyResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for JwtVerifyResponse {
  type Proxied = Self;
  fn as_view(&self) -> JwtVerifyResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for JwtVerifyResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> JwtVerifyResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for JwtVerifyResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__loquat__v1__JwtVerifyResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X0");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__loquat__v1__JwtVerifyResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__loquat__v1__JwtVerifyResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JwtVerifyResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JwtVerifyResponse {
  type Msg = JwtVerifyResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JwtVerifyResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JwtVerifyResponse {
  type Msg = JwtVerifyResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JwtVerifyResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JwtVerifyResponseMut<'_> {
  type Msg = JwtVerifyResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JwtVerifyResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JwtVerifyResponseMut<'_> {
  type Msg = JwtVerifyResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JwtVerifyResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JwtVerifyResponseView<'_> {
  type Msg = JwtVerifyResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JwtVerifyResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JwtVerifyResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__loquat__v1__JwtSignRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct JwtSignRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<JwtSignRequest>
}

impl ::protobuf::Message for JwtSignRequest {}

impl ::std::default::Default for JwtSignRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for JwtSignRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `JwtSignRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `JwtSignRequestMut`.
unsafe impl Sync for JwtSignRequest {}

// SAFETY:
// - `JwtSignRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for JwtSignRequest {}

impl ::protobuf::Proxied for JwtSignRequest {
  type View<'msg> = JwtSignRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for JwtSignRequest {}

impl ::protobuf::MutProxied for JwtSignRequest {
  type Mut<'msg> = JwtSignRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct JwtSignRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JwtSignRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JwtSignRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for JwtSignRequestView<'msg> {
  type Message = JwtSignRequest;
}

impl ::std::fmt::Debug for JwtSignRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for JwtSignRequestView<'_> {
  fn default() -> JwtSignRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, JwtSignRequest>> for JwtSignRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JwtSignRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JwtSignRequestView<'msg> {

  pub fn to_owned(&self) -> JwtSignRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // jwt_id: optional string
  pub fn has_jwt_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn jwt_id_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.jwt_id(), self.has_jwt_id())
  }
  pub fn jwt_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // key_id: optional string
  pub fn has_key_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn key_id_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.key_id(), self.has_key_id())
  }
  pub fn key_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // issuer: optional string
  pub fn issuer(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // subject: optional string
  pub fn subject(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // audiences: repeated string
  pub fn audiences(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // issued_at: optional int64
  pub fn issued_at(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        5, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // not_before: optional int64
  pub fn not_before(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        6, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // expired_at: optional int64
  pub fn expired_at(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        7, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // payload: optional bytes
  pub fn has_payload(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn payload_opt(self) -> ::protobuf::Optional<&'msg [u8]> {
        ::protobuf::Optional::new(self.payload(), self.has_payload())
  }
  pub fn payload(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `JwtSignRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for JwtSignRequestView<'_> {}

// SAFETY:
// - `JwtSignRequestView` is `Send` because while its alive a `JwtSignRequestMut` cannot.
// - `JwtSignRequestView` does not use thread-local data.
unsafe impl Send for JwtSignRequestView<'_> {}

impl<'msg> ::protobuf::AsView for JwtSignRequestView<'msg> {
  type Proxied = JwtSignRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, JwtSignRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JwtSignRequestView<'msg> {
  fn into_view<'shorter>(self) -> JwtSignRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<JwtSignRequest> for JwtSignRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JwtSignRequest {
    let mut dst = JwtSignRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<JwtSignRequest> for JwtSignRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JwtSignRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for JwtSignRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for JwtSignRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for JwtSignRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct JwtSignRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JwtSignRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JwtSignRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for JwtSignRequestMut<'msg> {
  type Message = JwtSignRequest;
}

impl ::std::fmt::Debug for JwtSignRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, JwtSignRequest>> for JwtSignRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JwtSignRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JwtSignRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, JwtSignRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> JwtSignRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // jwt_id: optional string
  pub fn has_jwt_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_jwt_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn jwt_id_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.jwt_id(), self.has_jwt_id())
  }
  pub fn jwt_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_jwt_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // key_id: optional string
  pub fn has_key_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_key_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn key_id_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.key_id(), self.has_key_id())
  }
  pub fn key_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_key_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // issuer: optional string
  pub fn issuer(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_issuer(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // subject: optional string
  pub fn subject(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subject(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // audiences: repeated string
  pub fn audiences(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn audiences_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        4,
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
  pub fn set_audiences(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // issued_at: optional int64
  pub fn issued_at(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        5, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_issued_at(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        5, val.into()
      )
    }
  }

  // not_before: optional int64
  pub fn not_before(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        6, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_not_before(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        6, val.into()
      )
    }
  }

  // expired_at: optional int64
  pub fn expired_at(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        7, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_expired_at(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        7, val.into()
      )
    }
  }

  // payload: optional bytes
  pub fn has_payload(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_payload(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn payload_opt(&self) -> ::protobuf::Optional<&'_ [u8]> {
        ::protobuf::Optional::new(self.payload(), self.has_payload())
  }
  pub fn payload(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_payload(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

}

// SAFETY:
// - `JwtSignRequestMut` does not perform any shared mutation.
unsafe impl Send for JwtSignRequestMut<'_> {}

// SAFETY:
// - `JwtSignRequestMut` does not perform any shared mutation.
unsafe impl Sync for JwtSignRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for JwtSignRequestMut<'msg> {
  type Proxied = JwtSignRequest;
  fn as_view(&self) -> ::protobuf::View<'_, JwtSignRequest> {
    JwtSignRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JwtSignRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, JwtSignRequest>
  where
      'msg: 'shorter {
    JwtSignRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for JwtSignRequestMut<'msg> {
  type MutProxied = JwtSignRequest;
  fn as_mut(&mut self) -> JwtSignRequestMut<'msg> {
    JwtSignRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for JwtSignRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> JwtSignRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl JwtSignRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, JwtSignRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> JwtSignRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> JwtSignRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // jwt_id: optional string
  pub fn has_jwt_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_jwt_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn jwt_id_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.jwt_id(), self.has_jwt_id())
  }
  pub fn jwt_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_jwt_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // key_id: optional string
  pub fn has_key_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_key_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn key_id_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.key_id(), self.has_key_id())
  }
  pub fn key_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_key_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // issuer: optional string
  pub fn issuer(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_issuer(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // subject: optional string
  pub fn subject(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subject(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // audiences: repeated string
  pub fn audiences(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn audiences_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        4,
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
  pub fn set_audiences(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // issued_at: optional int64
  pub fn issued_at(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        5, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_issued_at(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        5, val.into()
      )
    }
  }

  // not_before: optional int64
  pub fn not_before(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        6, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_not_before(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        6, val.into()
      )
    }
  }

  // expired_at: optional int64
  pub fn expired_at(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        7, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_expired_at(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        7, val.into()
      )
    }
  }

  // payload: optional bytes
  pub fn has_payload(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_payload(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn payload_opt(&self) -> ::protobuf::Optional<&'_ [u8]> {
        ::protobuf::Optional::new(self.payload(), self.has_payload())
  }
  pub fn payload(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_payload(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

}  // impl JwtSignRequest

impl ::std::ops::Drop for JwtSignRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for JwtSignRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for JwtSignRequest {
  type Proxied = Self;
  fn as_view(&self) -> JwtSignRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for JwtSignRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> JwtSignRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for JwtSignRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__loquat__v1__JwtSignRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1T1T1X1XET+P+P+P0");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__loquat__v1__JwtSignRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__loquat__v1__JwtSignRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JwtSignRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JwtSignRequest {
  type Msg = JwtSignRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JwtSignRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JwtSignRequest {
  type Msg = JwtSignRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JwtSignRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JwtSignRequestMut<'_> {
  type Msg = JwtSignRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JwtSignRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JwtSignRequestMut<'_> {
  type Msg = JwtSignRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JwtSignRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JwtSignRequestView<'_> {
  type Msg = JwtSignRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JwtSignRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JwtSignRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__loquat__v1__JwtSignResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct JwtSignResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<JwtSignResponse>
}

impl ::protobuf::Message for JwtSignResponse {}

impl ::std::default::Default for JwtSignResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for JwtSignResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `JwtSignResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `JwtSignResponseMut`.
unsafe impl Sync for JwtSignResponse {}

// SAFETY:
// - `JwtSignResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for JwtSignResponse {}

impl ::protobuf::Proxied for JwtSignResponse {
  type View<'msg> = JwtSignResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for JwtSignResponse {}

impl ::protobuf::MutProxied for JwtSignResponse {
  type Mut<'msg> = JwtSignResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct JwtSignResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JwtSignResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JwtSignResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for JwtSignResponseView<'msg> {
  type Message = JwtSignResponse;
}

impl ::std::fmt::Debug for JwtSignResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for JwtSignResponseView<'_> {
  fn default() -> JwtSignResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, JwtSignResponse>> for JwtSignResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JwtSignResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JwtSignResponseView<'msg> {

  pub fn to_owned(&self) -> JwtSignResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // token: optional string
  pub fn token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `JwtSignResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for JwtSignResponseView<'_> {}

// SAFETY:
// - `JwtSignResponseView` is `Send` because while its alive a `JwtSignResponseMut` cannot.
// - `JwtSignResponseView` does not use thread-local data.
unsafe impl Send for JwtSignResponseView<'_> {}

impl<'msg> ::protobuf::AsView for JwtSignResponseView<'msg> {
  type Proxied = JwtSignResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, JwtSignResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JwtSignResponseView<'msg> {
  fn into_view<'shorter>(self) -> JwtSignResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<JwtSignResponse> for JwtSignResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JwtSignResponse {
    let mut dst = JwtSignResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<JwtSignResponse> for JwtSignResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JwtSignResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for JwtSignResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for JwtSignResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for JwtSignResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct JwtSignResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JwtSignResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JwtSignResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for JwtSignResponseMut<'msg> {
  type Message = JwtSignResponse;
}

impl ::std::fmt::Debug for JwtSignResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, JwtSignResponse>> for JwtSignResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JwtSignResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JwtSignResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, JwtSignResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> JwtSignResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // token: optional string
  pub fn token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `JwtSignResponseMut` does not perform any shared mutation.
unsafe impl Send for JwtSignResponseMut<'_> {}

// SAFETY:
// - `JwtSignResponseMut` does not perform any shared mutation.
unsafe impl Sync for JwtSignResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for JwtSignResponseMut<'msg> {
  type Proxied = JwtSignResponse;
  fn as_view(&self) -> ::protobuf::View<'_, JwtSignResponse> {
    JwtSignResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JwtSignResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, JwtSignResponse>
  where
      'msg: 'shorter {
    JwtSignResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for JwtSignResponseMut<'msg> {
  type MutProxied = JwtSignResponse;
  fn as_mut(&mut self) -> JwtSignResponseMut<'msg> {
    JwtSignResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for JwtSignResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> JwtSignResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl JwtSignResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, JwtSignResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> JwtSignResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> JwtSignResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // token: optional string
  pub fn token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl JwtSignResponse

impl ::std::ops::Drop for JwtSignResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for JwtSignResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for JwtSignResponse {
  type Proxied = Self;
  fn as_view(&self) -> JwtSignResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for JwtSignResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> JwtSignResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for JwtSignResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__loquat__v1__JwtSignResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__loquat__v1__JwtSignResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__loquat__v1__JwtSignResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JwtSignResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JwtSignResponse {
  type Msg = JwtSignResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JwtSignResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JwtSignResponse {
  type Msg = JwtSignResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JwtSignResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JwtSignResponseMut<'_> {
  type Msg = JwtSignResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JwtSignResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JwtSignResponseMut<'_> {
  type Msg = JwtSignResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JwtSignResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JwtSignResponseView<'_> {
  type Msg = JwtSignResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JwtSignResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JwtSignResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__loquat__v1__HMacSignRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HMacSignRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HMacSignRequest>
}

impl ::protobuf::Message for HMacSignRequest {}

impl ::std::default::Default for HMacSignRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HMacSignRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HMacSignRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `HMacSignRequestMut`.
unsafe impl Sync for HMacSignRequest {}

// SAFETY:
// - `HMacSignRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for HMacSignRequest {}

impl ::protobuf::Proxied for HMacSignRequest {
  type View<'msg> = HMacSignRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HMacSignRequest {}

impl ::protobuf::MutProxied for HMacSignRequest {
  type Mut<'msg> = HMacSignRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HMacSignRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HMacSignRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HMacSignRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HMacSignRequestView<'msg> {
  type Message = HMacSignRequest;
}

impl ::std::fmt::Debug for HMacSignRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HMacSignRequestView<'_> {
  fn default() -> HMacSignRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HMacSignRequest>> for HMacSignRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HMacSignRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HMacSignRequestView<'msg> {

  pub fn to_owned(&self) -> HMacSignRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // plain: optional bytes
  pub fn plain(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `HMacSignRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for HMacSignRequestView<'_> {}

// SAFETY:
// - `HMacSignRequestView` is `Send` because while its alive a `HMacSignRequestMut` cannot.
// - `HMacSignRequestView` does not use thread-local data.
unsafe impl Send for HMacSignRequestView<'_> {}

impl<'msg> ::protobuf::AsView for HMacSignRequestView<'msg> {
  type Proxied = HMacSignRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, HMacSignRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HMacSignRequestView<'msg> {
  fn into_view<'shorter>(self) -> HMacSignRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HMacSignRequest> for HMacSignRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HMacSignRequest {
    let mut dst = HMacSignRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HMacSignRequest> for HMacSignRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HMacSignRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for HMacSignRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for HMacSignRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for HMacSignRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HMacSignRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HMacSignRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HMacSignRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HMacSignRequestMut<'msg> {
  type Message = HMacSignRequest;
}

impl ::std::fmt::Debug for HMacSignRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HMacSignRequest>> for HMacSignRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HMacSignRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HMacSignRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HMacSignRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> HMacSignRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // plain: optional bytes
  pub fn plain(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_plain(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `HMacSignRequestMut` does not perform any shared mutation.
unsafe impl Send for HMacSignRequestMut<'_> {}

// SAFETY:
// - `HMacSignRequestMut` does not perform any shared mutation.
unsafe impl Sync for HMacSignRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for HMacSignRequestMut<'msg> {
  type Proxied = HMacSignRequest;
  fn as_view(&self) -> ::protobuf::View<'_, HMacSignRequest> {
    HMacSignRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HMacSignRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HMacSignRequest>
  where
      'msg: 'shorter {
    HMacSignRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for HMacSignRequestMut<'msg> {
  type MutProxied = HMacSignRequest;
  fn as_mut(&mut self) -> HMacSignRequestMut<'msg> {
    HMacSignRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HMacSignRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> HMacSignRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HMacSignRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HMacSignRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HMacSignRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HMacSignRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // plain: optional bytes
  pub fn plain(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_plain(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl HMacSignRequest

impl ::std::ops::Drop for HMacSignRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HMacSignRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HMacSignRequest {
  type Proxied = Self;
  fn as_view(&self) -> HMacSignRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HMacSignRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HMacSignRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HMacSignRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__loquat__v1__HMacSignRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$0P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__loquat__v1__HMacSignRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__loquat__v1__HMacSignRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HMacSignRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HMacSignRequest {
  type Msg = HMacSignRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HMacSignRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HMacSignRequest {
  type Msg = HMacSignRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HMacSignRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HMacSignRequestMut<'_> {
  type Msg = HMacSignRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HMacSignRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HMacSignRequestMut<'_> {
  type Msg = HMacSignRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HMacSignRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HMacSignRequestView<'_> {
  type Msg = HMacSignRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HMacSignRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HMacSignRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__loquat__v1__HMacSignResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HMacSignResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HMacSignResponse>
}

impl ::protobuf::Message for HMacSignResponse {}

impl ::std::default::Default for HMacSignResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HMacSignResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HMacSignResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `HMacSignResponseMut`.
unsafe impl Sync for HMacSignResponse {}

// SAFETY:
// - `HMacSignResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for HMacSignResponse {}

impl ::protobuf::Proxied for HMacSignResponse {
  type View<'msg> = HMacSignResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HMacSignResponse {}

impl ::protobuf::MutProxied for HMacSignResponse {
  type Mut<'msg> = HMacSignResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HMacSignResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HMacSignResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HMacSignResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HMacSignResponseView<'msg> {
  type Message = HMacSignResponse;
}

impl ::std::fmt::Debug for HMacSignResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HMacSignResponseView<'_> {
  fn default() -> HMacSignResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HMacSignResponse>> for HMacSignResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HMacSignResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HMacSignResponseView<'msg> {

  pub fn to_owned(&self) -> HMacSignResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // hashed: optional bytes
  pub fn hashed(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `HMacSignResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for HMacSignResponseView<'_> {}

// SAFETY:
// - `HMacSignResponseView` is `Send` because while its alive a `HMacSignResponseMut` cannot.
// - `HMacSignResponseView` does not use thread-local data.
unsafe impl Send for HMacSignResponseView<'_> {}

impl<'msg> ::protobuf::AsView for HMacSignResponseView<'msg> {
  type Proxied = HMacSignResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, HMacSignResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HMacSignResponseView<'msg> {
  fn into_view<'shorter>(self) -> HMacSignResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HMacSignResponse> for HMacSignResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HMacSignResponse {
    let mut dst = HMacSignResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HMacSignResponse> for HMacSignResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HMacSignResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for HMacSignResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for HMacSignResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for HMacSignResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HMacSignResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HMacSignResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HMacSignResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HMacSignResponseMut<'msg> {
  type Message = HMacSignResponse;
}

impl ::std::fmt::Debug for HMacSignResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HMacSignResponse>> for HMacSignResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HMacSignResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HMacSignResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HMacSignResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> HMacSignResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // hashed: optional bytes
  pub fn hashed(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_hashed(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `HMacSignResponseMut` does not perform any shared mutation.
unsafe impl Send for HMacSignResponseMut<'_> {}

// SAFETY:
// - `HMacSignResponseMut` does not perform any shared mutation.
unsafe impl Sync for HMacSignResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for HMacSignResponseMut<'msg> {
  type Proxied = HMacSignResponse;
  fn as_view(&self) -> ::protobuf::View<'_, HMacSignResponse> {
    HMacSignResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HMacSignResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HMacSignResponse>
  where
      'msg: 'shorter {
    HMacSignResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for HMacSignResponseMut<'msg> {
  type MutProxied = HMacSignResponse;
  fn as_mut(&mut self) -> HMacSignResponseMut<'msg> {
    HMacSignResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HMacSignResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> HMacSignResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HMacSignResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HMacSignResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HMacSignResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HMacSignResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // hashed: optional bytes
  pub fn hashed(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_hashed(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl HMacSignResponse

impl ::std::ops::Drop for HMacSignResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HMacSignResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HMacSignResponse {
  type Proxied = Self;
  fn as_view(&self) -> HMacSignResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HMacSignResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HMacSignResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HMacSignResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__loquat__v1__HMacSignResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$0P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__loquat__v1__HMacSignResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__loquat__v1__HMacSignResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HMacSignResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HMacSignResponse {
  type Msg = HMacSignResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HMacSignResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HMacSignResponse {
  type Msg = HMacSignResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HMacSignResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HMacSignResponseMut<'_> {
  type Msg = HMacSignResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HMacSignResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HMacSignResponseMut<'_> {
  type Msg = HMacSignResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HMacSignResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HMacSignResponseView<'_> {
  type Msg = HMacSignResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HMacSignResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HMacSignResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__loquat__v1__HMacVerifyRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HMacVerifyRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HMacVerifyRequest>
}

impl ::protobuf::Message for HMacVerifyRequest {}

impl ::std::default::Default for HMacVerifyRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HMacVerifyRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HMacVerifyRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `HMacVerifyRequestMut`.
unsafe impl Sync for HMacVerifyRequest {}

// SAFETY:
// - `HMacVerifyRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for HMacVerifyRequest {}

impl ::protobuf::Proxied for HMacVerifyRequest {
  type View<'msg> = HMacVerifyRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HMacVerifyRequest {}

impl ::protobuf::MutProxied for HMacVerifyRequest {
  type Mut<'msg> = HMacVerifyRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HMacVerifyRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HMacVerifyRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HMacVerifyRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HMacVerifyRequestView<'msg> {
  type Message = HMacVerifyRequest;
}

impl ::std::fmt::Debug for HMacVerifyRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HMacVerifyRequestView<'_> {
  fn default() -> HMacVerifyRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HMacVerifyRequest>> for HMacVerifyRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HMacVerifyRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HMacVerifyRequestView<'msg> {

  pub fn to_owned(&self) -> HMacVerifyRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // hashed: optional bytes
  pub fn hashed(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // plain: optional bytes
  pub fn plain(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `HMacVerifyRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for HMacVerifyRequestView<'_> {}

// SAFETY:
// - `HMacVerifyRequestView` is `Send` because while its alive a `HMacVerifyRequestMut` cannot.
// - `HMacVerifyRequestView` does not use thread-local data.
unsafe impl Send for HMacVerifyRequestView<'_> {}

impl<'msg> ::protobuf::AsView for HMacVerifyRequestView<'msg> {
  type Proxied = HMacVerifyRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, HMacVerifyRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HMacVerifyRequestView<'msg> {
  fn into_view<'shorter>(self) -> HMacVerifyRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HMacVerifyRequest> for HMacVerifyRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HMacVerifyRequest {
    let mut dst = HMacVerifyRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HMacVerifyRequest> for HMacVerifyRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HMacVerifyRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for HMacVerifyRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for HMacVerifyRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for HMacVerifyRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HMacVerifyRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HMacVerifyRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HMacVerifyRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HMacVerifyRequestMut<'msg> {
  type Message = HMacVerifyRequest;
}

impl ::std::fmt::Debug for HMacVerifyRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HMacVerifyRequest>> for HMacVerifyRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HMacVerifyRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HMacVerifyRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HMacVerifyRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> HMacVerifyRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // hashed: optional bytes
  pub fn hashed(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_hashed(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // plain: optional bytes
  pub fn plain(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_plain(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `HMacVerifyRequestMut` does not perform any shared mutation.
unsafe impl Send for HMacVerifyRequestMut<'_> {}

// SAFETY:
// - `HMacVerifyRequestMut` does not perform any shared mutation.
unsafe impl Sync for HMacVerifyRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for HMacVerifyRequestMut<'msg> {
  type Proxied = HMacVerifyRequest;
  fn as_view(&self) -> ::protobuf::View<'_, HMacVerifyRequest> {
    HMacVerifyRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HMacVerifyRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HMacVerifyRequest>
  where
      'msg: 'shorter {
    HMacVerifyRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for HMacVerifyRequestMut<'msg> {
  type MutProxied = HMacVerifyRequest;
  fn as_mut(&mut self) -> HMacVerifyRequestMut<'msg> {
    HMacVerifyRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HMacVerifyRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> HMacVerifyRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HMacVerifyRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HMacVerifyRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HMacVerifyRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HMacVerifyRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // hashed: optional bytes
  pub fn hashed(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_hashed(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // plain: optional bytes
  pub fn plain(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_plain(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl HMacVerifyRequest

impl ::std::ops::Drop for HMacVerifyRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HMacVerifyRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HMacVerifyRequest {
  type Proxied = Self;
  fn as_view(&self) -> HMacVerifyRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HMacVerifyRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HMacVerifyRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HMacVerifyRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__loquat__v1__HMacVerifyRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$0P0P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__loquat__v1__HMacVerifyRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__loquat__v1__HMacVerifyRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HMacVerifyRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HMacVerifyRequest {
  type Msg = HMacVerifyRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HMacVerifyRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HMacVerifyRequest {
  type Msg = HMacVerifyRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HMacVerifyRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HMacVerifyRequestMut<'_> {
  type Msg = HMacVerifyRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HMacVerifyRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HMacVerifyRequestMut<'_> {
  type Msg = HMacVerifyRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HMacVerifyRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HMacVerifyRequestView<'_> {
  type Msg = HMacVerifyRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HMacVerifyRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HMacVerifyRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__loquat__v1__AesEncryptRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AesEncryptRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AesEncryptRequest>
}

impl ::protobuf::Message for AesEncryptRequest {}

impl ::std::default::Default for AesEncryptRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AesEncryptRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AesEncryptRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `AesEncryptRequestMut`.
unsafe impl Sync for AesEncryptRequest {}

// SAFETY:
// - `AesEncryptRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for AesEncryptRequest {}

impl ::protobuf::Proxied for AesEncryptRequest {
  type View<'msg> = AesEncryptRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AesEncryptRequest {}

impl ::protobuf::MutProxied for AesEncryptRequest {
  type Mut<'msg> = AesEncryptRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AesEncryptRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AesEncryptRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AesEncryptRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AesEncryptRequestView<'msg> {
  type Message = AesEncryptRequest;
}

impl ::std::fmt::Debug for AesEncryptRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AesEncryptRequestView<'_> {
  fn default() -> AesEncryptRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AesEncryptRequest>> for AesEncryptRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AesEncryptRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AesEncryptRequestView<'msg> {

  pub fn to_owned(&self) -> AesEncryptRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // plain: optional bytes
  pub fn plain(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // associated_data: optional bytes
  pub fn associated_data(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `AesEncryptRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for AesEncryptRequestView<'_> {}

// SAFETY:
// - `AesEncryptRequestView` is `Send` because while its alive a `AesEncryptRequestMut` cannot.
// - `AesEncryptRequestView` does not use thread-local data.
unsafe impl Send for AesEncryptRequestView<'_> {}

impl<'msg> ::protobuf::AsView for AesEncryptRequestView<'msg> {
  type Proxied = AesEncryptRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, AesEncryptRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AesEncryptRequestView<'msg> {
  fn into_view<'shorter>(self) -> AesEncryptRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AesEncryptRequest> for AesEncryptRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AesEncryptRequest {
    let mut dst = AesEncryptRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AesEncryptRequest> for AesEncryptRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AesEncryptRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for AesEncryptRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AesEncryptRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AesEncryptRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AesEncryptRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AesEncryptRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AesEncryptRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AesEncryptRequestMut<'msg> {
  type Message = AesEncryptRequest;
}

impl ::std::fmt::Debug for AesEncryptRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AesEncryptRequest>> for AesEncryptRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AesEncryptRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AesEncryptRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AesEncryptRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> AesEncryptRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // plain: optional bytes
  pub fn plain(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_plain(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // associated_data: optional bytes
  pub fn associated_data(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_associated_data(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `AesEncryptRequestMut` does not perform any shared mutation.
unsafe impl Send for AesEncryptRequestMut<'_> {}

// SAFETY:
// - `AesEncryptRequestMut` does not perform any shared mutation.
unsafe impl Sync for AesEncryptRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for AesEncryptRequestMut<'msg> {
  type Proxied = AesEncryptRequest;
  fn as_view(&self) -> ::protobuf::View<'_, AesEncryptRequest> {
    AesEncryptRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AesEncryptRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AesEncryptRequest>
  where
      'msg: 'shorter {
    AesEncryptRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for AesEncryptRequestMut<'msg> {
  type MutProxied = AesEncryptRequest;
  fn as_mut(&mut self) -> AesEncryptRequestMut<'msg> {
    AesEncryptRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AesEncryptRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> AesEncryptRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AesEncryptRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AesEncryptRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AesEncryptRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AesEncryptRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // plain: optional bytes
  pub fn plain(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_plain(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // associated_data: optional bytes
  pub fn associated_data(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_associated_data(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl AesEncryptRequest

impl ::std::ops::Drop for AesEncryptRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AesEncryptRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AesEncryptRequest {
  type Proxied = Self;
  fn as_view(&self) -> AesEncryptRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AesEncryptRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AesEncryptRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AesEncryptRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__loquat__v1__AesEncryptRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$0P0P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__loquat__v1__AesEncryptRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__loquat__v1__AesEncryptRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AesEncryptRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AesEncryptRequest {
  type Msg = AesEncryptRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AesEncryptRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AesEncryptRequest {
  type Msg = AesEncryptRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AesEncryptRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AesEncryptRequestMut<'_> {
  type Msg = AesEncryptRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AesEncryptRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AesEncryptRequestMut<'_> {
  type Msg = AesEncryptRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AesEncryptRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AesEncryptRequestView<'_> {
  type Msg = AesEncryptRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AesEncryptRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AesEncryptRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__loquat__v1__AesEncryptResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AesEncryptResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AesEncryptResponse>
}

impl ::protobuf::Message for AesEncryptResponse {}

impl ::std::default::Default for AesEncryptResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AesEncryptResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AesEncryptResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `AesEncryptResponseMut`.
unsafe impl Sync for AesEncryptResponse {}

// SAFETY:
// - `AesEncryptResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for AesEncryptResponse {}

impl ::protobuf::Proxied for AesEncryptResponse {
  type View<'msg> = AesEncryptResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AesEncryptResponse {}

impl ::protobuf::MutProxied for AesEncryptResponse {
  type Mut<'msg> = AesEncryptResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AesEncryptResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AesEncryptResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AesEncryptResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AesEncryptResponseView<'msg> {
  type Message = AesEncryptResponse;
}

impl ::std::fmt::Debug for AesEncryptResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AesEncryptResponseView<'_> {
  fn default() -> AesEncryptResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AesEncryptResponse>> for AesEncryptResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AesEncryptResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AesEncryptResponseView<'msg> {

  pub fn to_owned(&self) -> AesEncryptResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // cipher: optional bytes
  pub fn cipher(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `AesEncryptResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for AesEncryptResponseView<'_> {}

// SAFETY:
// - `AesEncryptResponseView` is `Send` because while its alive a `AesEncryptResponseMut` cannot.
// - `AesEncryptResponseView` does not use thread-local data.
unsafe impl Send for AesEncryptResponseView<'_> {}

impl<'msg> ::protobuf::AsView for AesEncryptResponseView<'msg> {
  type Proxied = AesEncryptResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, AesEncryptResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AesEncryptResponseView<'msg> {
  fn into_view<'shorter>(self) -> AesEncryptResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AesEncryptResponse> for AesEncryptResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AesEncryptResponse {
    let mut dst = AesEncryptResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AesEncryptResponse> for AesEncryptResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AesEncryptResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for AesEncryptResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AesEncryptResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AesEncryptResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AesEncryptResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AesEncryptResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AesEncryptResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AesEncryptResponseMut<'msg> {
  type Message = AesEncryptResponse;
}

impl ::std::fmt::Debug for AesEncryptResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AesEncryptResponse>> for AesEncryptResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AesEncryptResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AesEncryptResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AesEncryptResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> AesEncryptResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // cipher: optional bytes
  pub fn cipher(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_cipher(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `AesEncryptResponseMut` does not perform any shared mutation.
unsafe impl Send for AesEncryptResponseMut<'_> {}

// SAFETY:
// - `AesEncryptResponseMut` does not perform any shared mutation.
unsafe impl Sync for AesEncryptResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for AesEncryptResponseMut<'msg> {
  type Proxied = AesEncryptResponse;
  fn as_view(&self) -> ::protobuf::View<'_, AesEncryptResponse> {
    AesEncryptResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AesEncryptResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AesEncryptResponse>
  where
      'msg: 'shorter {
    AesEncryptResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for AesEncryptResponseMut<'msg> {
  type MutProxied = AesEncryptResponse;
  fn as_mut(&mut self) -> AesEncryptResponseMut<'msg> {
    AesEncryptResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AesEncryptResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> AesEncryptResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AesEncryptResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AesEncryptResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AesEncryptResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AesEncryptResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // cipher: optional bytes
  pub fn cipher(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_cipher(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl AesEncryptResponse

impl ::std::ops::Drop for AesEncryptResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AesEncryptResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AesEncryptResponse {
  type Proxied = Self;
  fn as_view(&self) -> AesEncryptResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AesEncryptResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AesEncryptResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AesEncryptResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__loquat__v1__AesEncryptResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$0P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__loquat__v1__AesEncryptResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__loquat__v1__AesEncryptResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AesEncryptResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AesEncryptResponse {
  type Msg = AesEncryptResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AesEncryptResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AesEncryptResponse {
  type Msg = AesEncryptResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AesEncryptResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AesEncryptResponseMut<'_> {
  type Msg = AesEncryptResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AesEncryptResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AesEncryptResponseMut<'_> {
  type Msg = AesEncryptResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AesEncryptResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AesEncryptResponseView<'_> {
  type Msg = AesEncryptResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AesEncryptResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AesEncryptResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__loquat__v1__AesDecryptRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AesDecryptRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AesDecryptRequest>
}

impl ::protobuf::Message for AesDecryptRequest {}

impl ::std::default::Default for AesDecryptRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AesDecryptRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AesDecryptRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `AesDecryptRequestMut`.
unsafe impl Sync for AesDecryptRequest {}

// SAFETY:
// - `AesDecryptRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for AesDecryptRequest {}

impl ::protobuf::Proxied for AesDecryptRequest {
  type View<'msg> = AesDecryptRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AesDecryptRequest {}

impl ::protobuf::MutProxied for AesDecryptRequest {
  type Mut<'msg> = AesDecryptRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AesDecryptRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AesDecryptRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AesDecryptRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AesDecryptRequestView<'msg> {
  type Message = AesDecryptRequest;
}

impl ::std::fmt::Debug for AesDecryptRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AesDecryptRequestView<'_> {
  fn default() -> AesDecryptRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AesDecryptRequest>> for AesDecryptRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AesDecryptRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AesDecryptRequestView<'msg> {

  pub fn to_owned(&self) -> AesDecryptRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // cipher: optional bytes
  pub fn cipher(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // associated_data: optional bytes
  pub fn associated_data(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `AesDecryptRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for AesDecryptRequestView<'_> {}

// SAFETY:
// - `AesDecryptRequestView` is `Send` because while its alive a `AesDecryptRequestMut` cannot.
// - `AesDecryptRequestView` does not use thread-local data.
unsafe impl Send for AesDecryptRequestView<'_> {}

impl<'msg> ::protobuf::AsView for AesDecryptRequestView<'msg> {
  type Proxied = AesDecryptRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, AesDecryptRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AesDecryptRequestView<'msg> {
  fn into_view<'shorter>(self) -> AesDecryptRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AesDecryptRequest> for AesDecryptRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AesDecryptRequest {
    let mut dst = AesDecryptRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AesDecryptRequest> for AesDecryptRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AesDecryptRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for AesDecryptRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AesDecryptRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AesDecryptRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AesDecryptRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AesDecryptRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AesDecryptRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AesDecryptRequestMut<'msg> {
  type Message = AesDecryptRequest;
}

impl ::std::fmt::Debug for AesDecryptRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AesDecryptRequest>> for AesDecryptRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AesDecryptRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AesDecryptRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AesDecryptRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> AesDecryptRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // cipher: optional bytes
  pub fn cipher(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_cipher(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // associated_data: optional bytes
  pub fn associated_data(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_associated_data(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `AesDecryptRequestMut` does not perform any shared mutation.
unsafe impl Send for AesDecryptRequestMut<'_> {}

// SAFETY:
// - `AesDecryptRequestMut` does not perform any shared mutation.
unsafe impl Sync for AesDecryptRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for AesDecryptRequestMut<'msg> {
  type Proxied = AesDecryptRequest;
  fn as_view(&self) -> ::protobuf::View<'_, AesDecryptRequest> {
    AesDecryptRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AesDecryptRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AesDecryptRequest>
  where
      'msg: 'shorter {
    AesDecryptRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for AesDecryptRequestMut<'msg> {
  type MutProxied = AesDecryptRequest;
  fn as_mut(&mut self) -> AesDecryptRequestMut<'msg> {
    AesDecryptRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AesDecryptRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> AesDecryptRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AesDecryptRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AesDecryptRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AesDecryptRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AesDecryptRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // cipher: optional bytes
  pub fn cipher(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_cipher(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // associated_data: optional bytes
  pub fn associated_data(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_associated_data(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl AesDecryptRequest

impl ::std::ops::Drop for AesDecryptRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AesDecryptRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AesDecryptRequest {
  type Proxied = Self;
  fn as_view(&self) -> AesDecryptRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AesDecryptRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AesDecryptRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AesDecryptRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__loquat__v1__AesDecryptRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$0P0P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__loquat__v1__AesDecryptRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__loquat__v1__AesDecryptRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AesDecryptRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AesDecryptRequest {
  type Msg = AesDecryptRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AesDecryptRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AesDecryptRequest {
  type Msg = AesDecryptRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AesDecryptRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AesDecryptRequestMut<'_> {
  type Msg = AesDecryptRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AesDecryptRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AesDecryptRequestMut<'_> {
  type Msg = AesDecryptRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AesDecryptRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AesDecryptRequestView<'_> {
  type Msg = AesDecryptRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AesDecryptRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AesDecryptRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__loquat__v1__AesDecryptResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AesDecryptResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AesDecryptResponse>
}

impl ::protobuf::Message for AesDecryptResponse {}

impl ::std::default::Default for AesDecryptResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AesDecryptResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AesDecryptResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `AesDecryptResponseMut`.
unsafe impl Sync for AesDecryptResponse {}

// SAFETY:
// - `AesDecryptResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for AesDecryptResponse {}

impl ::protobuf::Proxied for AesDecryptResponse {
  type View<'msg> = AesDecryptResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AesDecryptResponse {}

impl ::protobuf::MutProxied for AesDecryptResponse {
  type Mut<'msg> = AesDecryptResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AesDecryptResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AesDecryptResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AesDecryptResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AesDecryptResponseView<'msg> {
  type Message = AesDecryptResponse;
}

impl ::std::fmt::Debug for AesDecryptResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AesDecryptResponseView<'_> {
  fn default() -> AesDecryptResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AesDecryptResponse>> for AesDecryptResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AesDecryptResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AesDecryptResponseView<'msg> {

  pub fn to_owned(&self) -> AesDecryptResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // plain: optional bytes
  pub fn plain(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `AesDecryptResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for AesDecryptResponseView<'_> {}

// SAFETY:
// - `AesDecryptResponseView` is `Send` because while its alive a `AesDecryptResponseMut` cannot.
// - `AesDecryptResponseView` does not use thread-local data.
unsafe impl Send for AesDecryptResponseView<'_> {}

impl<'msg> ::protobuf::AsView for AesDecryptResponseView<'msg> {
  type Proxied = AesDecryptResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, AesDecryptResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AesDecryptResponseView<'msg> {
  fn into_view<'shorter>(self) -> AesDecryptResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AesDecryptResponse> for AesDecryptResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AesDecryptResponse {
    let mut dst = AesDecryptResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AesDecryptResponse> for AesDecryptResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AesDecryptResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for AesDecryptResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AesDecryptResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AesDecryptResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AesDecryptResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AesDecryptResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AesDecryptResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AesDecryptResponseMut<'msg> {
  type Message = AesDecryptResponse;
}

impl ::std::fmt::Debug for AesDecryptResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AesDecryptResponse>> for AesDecryptResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AesDecryptResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AesDecryptResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AesDecryptResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> AesDecryptResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // plain: optional bytes
  pub fn plain(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_plain(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `AesDecryptResponseMut` does not perform any shared mutation.
unsafe impl Send for AesDecryptResponseMut<'_> {}

// SAFETY:
// - `AesDecryptResponseMut` does not perform any shared mutation.
unsafe impl Sync for AesDecryptResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for AesDecryptResponseMut<'msg> {
  type Proxied = AesDecryptResponse;
  fn as_view(&self) -> ::protobuf::View<'_, AesDecryptResponse> {
    AesDecryptResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AesDecryptResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AesDecryptResponse>
  where
      'msg: 'shorter {
    AesDecryptResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for AesDecryptResponseMut<'msg> {
  type MutProxied = AesDecryptResponse;
  fn as_mut(&mut self) -> AesDecryptResponseMut<'msg> {
    AesDecryptResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AesDecryptResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> AesDecryptResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AesDecryptResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AesDecryptResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AesDecryptResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AesDecryptResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // plain: optional bytes
  pub fn plain(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_plain(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl AesDecryptResponse

impl ::std::ops::Drop for AesDecryptResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AesDecryptResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AesDecryptResponse {
  type Proxied = Self;
  fn as_view(&self) -> AesDecryptResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AesDecryptResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AesDecryptResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AesDecryptResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__loquat__v1__AesDecryptResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$0P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__loquat__v1__AesDecryptResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__loquat__v1__AesDecryptResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AesDecryptResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AesDecryptResponse {
  type Msg = AesDecryptResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AesDecryptResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AesDecryptResponse {
  type Msg = AesDecryptResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AesDecryptResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AesDecryptResponseMut<'_> {
  type Msg = AesDecryptResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AesDecryptResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AesDecryptResponseMut<'_> {
  type Msg = AesDecryptResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AesDecryptResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AesDecryptResponseView<'_> {
  type Msg = AesDecryptResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AesDecryptResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AesDecryptResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__loquat__v1__Argon2SignRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Argon2SignRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Argon2SignRequest>
}

impl ::protobuf::Message for Argon2SignRequest {}

impl ::std::default::Default for Argon2SignRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Argon2SignRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Argon2SignRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `Argon2SignRequestMut`.
unsafe impl Sync for Argon2SignRequest {}

// SAFETY:
// - `Argon2SignRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Argon2SignRequest {}

impl ::protobuf::Proxied for Argon2SignRequest {
  type View<'msg> = Argon2SignRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Argon2SignRequest {}

impl ::protobuf::MutProxied for Argon2SignRequest {
  type Mut<'msg> = Argon2SignRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Argon2SignRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Argon2SignRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Argon2SignRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for Argon2SignRequestView<'msg> {
  type Message = Argon2SignRequest;
}

impl ::std::fmt::Debug for Argon2SignRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for Argon2SignRequestView<'_> {
  fn default() -> Argon2SignRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Argon2SignRequest>> for Argon2SignRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Argon2SignRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Argon2SignRequestView<'msg> {

  pub fn to_owned(&self) -> Argon2SignRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // password: optional string
  pub fn password(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `Argon2SignRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for Argon2SignRequestView<'_> {}

// SAFETY:
// - `Argon2SignRequestView` is `Send` because while its alive a `Argon2SignRequestMut` cannot.
// - `Argon2SignRequestView` does not use thread-local data.
unsafe impl Send for Argon2SignRequestView<'_> {}

impl<'msg> ::protobuf::AsView for Argon2SignRequestView<'msg> {
  type Proxied = Argon2SignRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, Argon2SignRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Argon2SignRequestView<'msg> {
  fn into_view<'shorter>(self) -> Argon2SignRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Argon2SignRequest> for Argon2SignRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Argon2SignRequest {
    let mut dst = Argon2SignRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Argon2SignRequest> for Argon2SignRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Argon2SignRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Argon2SignRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for Argon2SignRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for Argon2SignRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct Argon2SignRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Argon2SignRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Argon2SignRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for Argon2SignRequestMut<'msg> {
  type Message = Argon2SignRequest;
}

impl ::std::fmt::Debug for Argon2SignRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Argon2SignRequest>> for Argon2SignRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Argon2SignRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Argon2SignRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Argon2SignRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> Argon2SignRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // password: optional string
  pub fn password(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_password(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `Argon2SignRequestMut` does not perform any shared mutation.
unsafe impl Send for Argon2SignRequestMut<'_> {}

// SAFETY:
// - `Argon2SignRequestMut` does not perform any shared mutation.
unsafe impl Sync for Argon2SignRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for Argon2SignRequestMut<'msg> {
  type Proxied = Argon2SignRequest;
  fn as_view(&self) -> ::protobuf::View<'_, Argon2SignRequest> {
    Argon2SignRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Argon2SignRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Argon2SignRequest>
  where
      'msg: 'shorter {
    Argon2SignRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for Argon2SignRequestMut<'msg> {
  type MutProxied = Argon2SignRequest;
  fn as_mut(&mut self) -> Argon2SignRequestMut<'msg> {
    Argon2SignRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for Argon2SignRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> Argon2SignRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Argon2SignRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Argon2SignRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> Argon2SignRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> Argon2SignRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // password: optional string
  pub fn password(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_password(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl Argon2SignRequest

impl ::std::ops::Drop for Argon2SignRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Argon2SignRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Argon2SignRequest {
  type Proxied = Self;
  fn as_view(&self) -> Argon2SignRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Argon2SignRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> Argon2SignRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Argon2SignRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__loquat__v1__Argon2SignRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__loquat__v1__Argon2SignRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__loquat__v1__Argon2SignRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Argon2SignRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Argon2SignRequest {
  type Msg = Argon2SignRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Argon2SignRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Argon2SignRequest {
  type Msg = Argon2SignRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Argon2SignRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Argon2SignRequestMut<'_> {
  type Msg = Argon2SignRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Argon2SignRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Argon2SignRequestMut<'_> {
  type Msg = Argon2SignRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Argon2SignRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Argon2SignRequestView<'_> {
  type Msg = Argon2SignRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Argon2SignRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Argon2SignRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__loquat__v1__Argon2SignResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Argon2SignResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Argon2SignResponse>
}

impl ::protobuf::Message for Argon2SignResponse {}

impl ::std::default::Default for Argon2SignResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Argon2SignResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Argon2SignResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `Argon2SignResponseMut`.
unsafe impl Sync for Argon2SignResponse {}

// SAFETY:
// - `Argon2SignResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Argon2SignResponse {}

impl ::protobuf::Proxied for Argon2SignResponse {
  type View<'msg> = Argon2SignResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Argon2SignResponse {}

impl ::protobuf::MutProxied for Argon2SignResponse {
  type Mut<'msg> = Argon2SignResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Argon2SignResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Argon2SignResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Argon2SignResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for Argon2SignResponseView<'msg> {
  type Message = Argon2SignResponse;
}

impl ::std::fmt::Debug for Argon2SignResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for Argon2SignResponseView<'_> {
  fn default() -> Argon2SignResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Argon2SignResponse>> for Argon2SignResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Argon2SignResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Argon2SignResponseView<'msg> {

  pub fn to_owned(&self) -> Argon2SignResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // hashed: optional string
  pub fn hashed(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `Argon2SignResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for Argon2SignResponseView<'_> {}

// SAFETY:
// - `Argon2SignResponseView` is `Send` because while its alive a `Argon2SignResponseMut` cannot.
// - `Argon2SignResponseView` does not use thread-local data.
unsafe impl Send for Argon2SignResponseView<'_> {}

impl<'msg> ::protobuf::AsView for Argon2SignResponseView<'msg> {
  type Proxied = Argon2SignResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, Argon2SignResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Argon2SignResponseView<'msg> {
  fn into_view<'shorter>(self) -> Argon2SignResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Argon2SignResponse> for Argon2SignResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Argon2SignResponse {
    let mut dst = Argon2SignResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Argon2SignResponse> for Argon2SignResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Argon2SignResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Argon2SignResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for Argon2SignResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for Argon2SignResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct Argon2SignResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Argon2SignResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Argon2SignResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for Argon2SignResponseMut<'msg> {
  type Message = Argon2SignResponse;
}

impl ::std::fmt::Debug for Argon2SignResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Argon2SignResponse>> for Argon2SignResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Argon2SignResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Argon2SignResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Argon2SignResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> Argon2SignResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // hashed: optional string
  pub fn hashed(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_hashed(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `Argon2SignResponseMut` does not perform any shared mutation.
unsafe impl Send for Argon2SignResponseMut<'_> {}

// SAFETY:
// - `Argon2SignResponseMut` does not perform any shared mutation.
unsafe impl Sync for Argon2SignResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for Argon2SignResponseMut<'msg> {
  type Proxied = Argon2SignResponse;
  fn as_view(&self) -> ::protobuf::View<'_, Argon2SignResponse> {
    Argon2SignResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Argon2SignResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Argon2SignResponse>
  where
      'msg: 'shorter {
    Argon2SignResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for Argon2SignResponseMut<'msg> {
  type MutProxied = Argon2SignResponse;
  fn as_mut(&mut self) -> Argon2SignResponseMut<'msg> {
    Argon2SignResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for Argon2SignResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> Argon2SignResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Argon2SignResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Argon2SignResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> Argon2SignResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> Argon2SignResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // hashed: optional string
  pub fn hashed(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_hashed(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl Argon2SignResponse

impl ::std::ops::Drop for Argon2SignResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Argon2SignResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Argon2SignResponse {
  type Proxied = Self;
  fn as_view(&self) -> Argon2SignResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Argon2SignResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> Argon2SignResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Argon2SignResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__loquat__v1__Argon2SignResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__loquat__v1__Argon2SignResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__loquat__v1__Argon2SignResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Argon2SignResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Argon2SignResponse {
  type Msg = Argon2SignResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Argon2SignResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Argon2SignResponse {
  type Msg = Argon2SignResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Argon2SignResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Argon2SignResponseMut<'_> {
  type Msg = Argon2SignResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Argon2SignResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Argon2SignResponseMut<'_> {
  type Msg = Argon2SignResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Argon2SignResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Argon2SignResponseView<'_> {
  type Msg = Argon2SignResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Argon2SignResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Argon2SignResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__loquat__v1__Argon2VerifyRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Argon2VerifyRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Argon2VerifyRequest>
}

impl ::protobuf::Message for Argon2VerifyRequest {}

impl ::std::default::Default for Argon2VerifyRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Argon2VerifyRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Argon2VerifyRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `Argon2VerifyRequestMut`.
unsafe impl Sync for Argon2VerifyRequest {}

// SAFETY:
// - `Argon2VerifyRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Argon2VerifyRequest {}

impl ::protobuf::Proxied for Argon2VerifyRequest {
  type View<'msg> = Argon2VerifyRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Argon2VerifyRequest {}

impl ::protobuf::MutProxied for Argon2VerifyRequest {
  type Mut<'msg> = Argon2VerifyRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Argon2VerifyRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Argon2VerifyRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Argon2VerifyRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for Argon2VerifyRequestView<'msg> {
  type Message = Argon2VerifyRequest;
}

impl ::std::fmt::Debug for Argon2VerifyRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for Argon2VerifyRequestView<'_> {
  fn default() -> Argon2VerifyRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Argon2VerifyRequest>> for Argon2VerifyRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Argon2VerifyRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Argon2VerifyRequestView<'msg> {

  pub fn to_owned(&self) -> Argon2VerifyRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // hashed: optional string
  pub fn hashed(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // password: optional string
  pub fn password(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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
// - `Argon2VerifyRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for Argon2VerifyRequestView<'_> {}

// SAFETY:
// - `Argon2VerifyRequestView` is `Send` because while its alive a `Argon2VerifyRequestMut` cannot.
// - `Argon2VerifyRequestView` does not use thread-local data.
unsafe impl Send for Argon2VerifyRequestView<'_> {}

impl<'msg> ::protobuf::AsView for Argon2VerifyRequestView<'msg> {
  type Proxied = Argon2VerifyRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, Argon2VerifyRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Argon2VerifyRequestView<'msg> {
  fn into_view<'shorter>(self) -> Argon2VerifyRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Argon2VerifyRequest> for Argon2VerifyRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Argon2VerifyRequest {
    let mut dst = Argon2VerifyRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Argon2VerifyRequest> for Argon2VerifyRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Argon2VerifyRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Argon2VerifyRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for Argon2VerifyRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for Argon2VerifyRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct Argon2VerifyRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Argon2VerifyRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Argon2VerifyRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for Argon2VerifyRequestMut<'msg> {
  type Message = Argon2VerifyRequest;
}

impl ::std::fmt::Debug for Argon2VerifyRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Argon2VerifyRequest>> for Argon2VerifyRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Argon2VerifyRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Argon2VerifyRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Argon2VerifyRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> Argon2VerifyRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // hashed: optional string
  pub fn hashed(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_hashed(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // password: optional string
  pub fn password(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_password(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `Argon2VerifyRequestMut` does not perform any shared mutation.
unsafe impl Send for Argon2VerifyRequestMut<'_> {}

// SAFETY:
// - `Argon2VerifyRequestMut` does not perform any shared mutation.
unsafe impl Sync for Argon2VerifyRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for Argon2VerifyRequestMut<'msg> {
  type Proxied = Argon2VerifyRequest;
  fn as_view(&self) -> ::protobuf::View<'_, Argon2VerifyRequest> {
    Argon2VerifyRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Argon2VerifyRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Argon2VerifyRequest>
  where
      'msg: 'shorter {
    Argon2VerifyRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for Argon2VerifyRequestMut<'msg> {
  type MutProxied = Argon2VerifyRequest;
  fn as_mut(&mut self) -> Argon2VerifyRequestMut<'msg> {
    Argon2VerifyRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for Argon2VerifyRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> Argon2VerifyRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Argon2VerifyRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Argon2VerifyRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> Argon2VerifyRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> Argon2VerifyRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // hashed: optional string
  pub fn hashed(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_hashed(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // password: optional string
  pub fn password(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_password(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl Argon2VerifyRequest

impl ::std::ops::Drop for Argon2VerifyRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Argon2VerifyRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Argon2VerifyRequest {
  type Proxied = Self;
  fn as_view(&self) -> Argon2VerifyRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Argon2VerifyRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> Argon2VerifyRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Argon2VerifyRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__loquat__v1__Argon2VerifyRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__loquat__v1__Argon2VerifyRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__loquat__v1__Argon2VerifyRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Argon2VerifyRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Argon2VerifyRequest {
  type Msg = Argon2VerifyRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Argon2VerifyRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Argon2VerifyRequest {
  type Msg = Argon2VerifyRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Argon2VerifyRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Argon2VerifyRequestMut<'_> {
  type Msg = Argon2VerifyRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Argon2VerifyRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Argon2VerifyRequestMut<'_> {
  type Msg = Argon2VerifyRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Argon2VerifyRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Argon2VerifyRequestView<'_> {
  type Msg = Argon2VerifyRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Argon2VerifyRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Argon2VerifyRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



