const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.34.0-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__casbin__v1__NewEnforcerRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct NewEnforcerRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<NewEnforcerRequest>
}

impl ::protobuf::Message for NewEnforcerRequest {}

impl ::std::default::Default for NewEnforcerRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for NewEnforcerRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `NewEnforcerRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `NewEnforcerRequestMut`.
unsafe impl Sync for NewEnforcerRequest {}

// SAFETY:
// - `NewEnforcerRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for NewEnforcerRequest {}

impl ::protobuf::Proxied for NewEnforcerRequest {
  type View<'msg> = NewEnforcerRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for NewEnforcerRequest {}

impl ::protobuf::MutProxied for NewEnforcerRequest {
  type Mut<'msg> = NewEnforcerRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct NewEnforcerRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NewEnforcerRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NewEnforcerRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for NewEnforcerRequestView<'msg> {
  type Message = NewEnforcerRequest;
}

impl ::std::fmt::Debug for NewEnforcerRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for NewEnforcerRequestView<'_> {
  fn default() -> NewEnforcerRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, NewEnforcerRequest>> for NewEnforcerRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NewEnforcerRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NewEnforcerRequestView<'msg> {

  pub fn to_owned(&self) -> NewEnforcerRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // modelText: optional string
  pub fn modelText(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // adapterHandle: optional int32
  pub fn adapterHandle(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `NewEnforcerRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for NewEnforcerRequestView<'_> {}

// SAFETY:
// - `NewEnforcerRequestView` is `Send` because while its alive a `NewEnforcerRequestMut` cannot.
// - `NewEnforcerRequestView` does not use thread-local data.
unsafe impl Send for NewEnforcerRequestView<'_> {}

impl<'msg> ::protobuf::AsView for NewEnforcerRequestView<'msg> {
  type Proxied = NewEnforcerRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, NewEnforcerRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NewEnforcerRequestView<'msg> {
  fn into_view<'shorter>(self) -> NewEnforcerRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<NewEnforcerRequest> for NewEnforcerRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NewEnforcerRequest {
    let mut dst = NewEnforcerRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<NewEnforcerRequest> for NewEnforcerRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NewEnforcerRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for NewEnforcerRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for NewEnforcerRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for NewEnforcerRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct NewEnforcerRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NewEnforcerRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NewEnforcerRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for NewEnforcerRequestMut<'msg> {
  type Message = NewEnforcerRequest;
}

impl ::std::fmt::Debug for NewEnforcerRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, NewEnforcerRequest>> for NewEnforcerRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NewEnforcerRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NewEnforcerRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, NewEnforcerRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> NewEnforcerRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // modelText: optional string
  pub fn modelText(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_modelText(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // adapterHandle: optional int32
  pub fn adapterHandle(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_adapterHandle(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `NewEnforcerRequestMut` does not perform any shared mutation.
unsafe impl Send for NewEnforcerRequestMut<'_> {}

// SAFETY:
// - `NewEnforcerRequestMut` does not perform any shared mutation.
unsafe impl Sync for NewEnforcerRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for NewEnforcerRequestMut<'msg> {
  type Proxied = NewEnforcerRequest;
  fn as_view(&self) -> ::protobuf::View<'_, NewEnforcerRequest> {
    NewEnforcerRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NewEnforcerRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, NewEnforcerRequest>
  where
      'msg: 'shorter {
    NewEnforcerRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for NewEnforcerRequestMut<'msg> {
  type MutProxied = NewEnforcerRequest;
  fn as_mut(&mut self) -> NewEnforcerRequestMut<'msg> {
    NewEnforcerRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for NewEnforcerRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> NewEnforcerRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl NewEnforcerRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, NewEnforcerRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> NewEnforcerRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> NewEnforcerRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // modelText: optional string
  pub fn modelText(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_modelText(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // adapterHandle: optional int32
  pub fn adapterHandle(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_adapterHandle(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

}  // impl NewEnforcerRequest

impl ::std::ops::Drop for NewEnforcerRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for NewEnforcerRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for NewEnforcerRequest {
  type Proxied = Self;
  fn as_view(&self) -> NewEnforcerRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for NewEnforcerRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> NewEnforcerRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for NewEnforcerRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__casbin__v1__NewEnforcerRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X(P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__casbin__v1__NewEnforcerRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__casbin__v1__NewEnforcerRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NewEnforcerRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NewEnforcerRequest {
  type Msg = NewEnforcerRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NewEnforcerRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NewEnforcerRequest {
  type Msg = NewEnforcerRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NewEnforcerRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NewEnforcerRequestMut<'_> {
  type Msg = NewEnforcerRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NewEnforcerRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NewEnforcerRequestMut<'_> {
  type Msg = NewEnforcerRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NewEnforcerRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NewEnforcerRequestView<'_> {
  type Msg = NewEnforcerRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NewEnforcerRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NewEnforcerRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__casbin__v1__NewEnforcerReply_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct NewEnforcerReply {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<NewEnforcerReply>
}

impl ::protobuf::Message for NewEnforcerReply {}

impl ::std::default::Default for NewEnforcerReply {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for NewEnforcerReply {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `NewEnforcerReply` is `Sync` because it does not implement interior mutability.
//    Neither does `NewEnforcerReplyMut`.
unsafe impl Sync for NewEnforcerReply {}

// SAFETY:
// - `NewEnforcerReply` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for NewEnforcerReply {}

impl ::protobuf::Proxied for NewEnforcerReply {
  type View<'msg> = NewEnforcerReplyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for NewEnforcerReply {}

impl ::protobuf::MutProxied for NewEnforcerReply {
  type Mut<'msg> = NewEnforcerReplyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct NewEnforcerReplyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NewEnforcerReply>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NewEnforcerReplyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for NewEnforcerReplyView<'msg> {
  type Message = NewEnforcerReply;
}

impl ::std::fmt::Debug for NewEnforcerReplyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for NewEnforcerReplyView<'_> {
  fn default() -> NewEnforcerReplyView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, NewEnforcerReply>> for NewEnforcerReplyView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NewEnforcerReply>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NewEnforcerReplyView<'msg> {

  pub fn to_owned(&self) -> NewEnforcerReply {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // handler: optional int32
  pub fn handler(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `NewEnforcerReplyView` is `Sync` because it does not support mutation.
unsafe impl Sync for NewEnforcerReplyView<'_> {}

// SAFETY:
// - `NewEnforcerReplyView` is `Send` because while its alive a `NewEnforcerReplyMut` cannot.
// - `NewEnforcerReplyView` does not use thread-local data.
unsafe impl Send for NewEnforcerReplyView<'_> {}

impl<'msg> ::protobuf::AsView for NewEnforcerReplyView<'msg> {
  type Proxied = NewEnforcerReply;
  fn as_view(&self) -> ::protobuf::View<'msg, NewEnforcerReply> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NewEnforcerReplyView<'msg> {
  fn into_view<'shorter>(self) -> NewEnforcerReplyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<NewEnforcerReply> for NewEnforcerReplyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NewEnforcerReply {
    let mut dst = NewEnforcerReply::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<NewEnforcerReply> for NewEnforcerReplyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NewEnforcerReply {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for NewEnforcerReply {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for NewEnforcerReplyView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for NewEnforcerReplyMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct NewEnforcerReplyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NewEnforcerReply>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NewEnforcerReplyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for NewEnforcerReplyMut<'msg> {
  type Message = NewEnforcerReply;
}

impl ::std::fmt::Debug for NewEnforcerReplyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, NewEnforcerReply>> for NewEnforcerReplyMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NewEnforcerReply>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NewEnforcerReplyMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, NewEnforcerReply> {
    self.inner
  }

  pub fn to_owned(&self) -> NewEnforcerReply {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // handler: optional int32
  pub fn handler(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_handler(&mut self, val: i32) {
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

}

// SAFETY:
// - `NewEnforcerReplyMut` does not perform any shared mutation.
unsafe impl Send for NewEnforcerReplyMut<'_> {}

// SAFETY:
// - `NewEnforcerReplyMut` does not perform any shared mutation.
unsafe impl Sync for NewEnforcerReplyMut<'_> {}

impl<'msg> ::protobuf::AsView for NewEnforcerReplyMut<'msg> {
  type Proxied = NewEnforcerReply;
  fn as_view(&self) -> ::protobuf::View<'_, NewEnforcerReply> {
    NewEnforcerReplyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NewEnforcerReplyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, NewEnforcerReply>
  where
      'msg: 'shorter {
    NewEnforcerReplyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for NewEnforcerReplyMut<'msg> {
  type MutProxied = NewEnforcerReply;
  fn as_mut(&mut self) -> NewEnforcerReplyMut<'msg> {
    NewEnforcerReplyMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for NewEnforcerReplyMut<'msg> {
  fn into_mut<'shorter>(self) -> NewEnforcerReplyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl NewEnforcerReply {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, NewEnforcerReply> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> NewEnforcerReplyView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> NewEnforcerReplyMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // handler: optional int32
  pub fn handler(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_handler(&mut self, val: i32) {
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

}  // impl NewEnforcerReply

impl ::std::ops::Drop for NewEnforcerReply {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for NewEnforcerReply {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for NewEnforcerReply {
  type Proxied = Self;
  fn as_view(&self) -> NewEnforcerReplyView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for NewEnforcerReply {
  type MutProxied = Self;
  fn as_mut(&mut self) -> NewEnforcerReplyMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for NewEnforcerReply {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__casbin__v1__NewEnforcerReply_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$(P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__casbin__v1__NewEnforcerReply_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__casbin__v1__NewEnforcerReply_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NewEnforcerReply {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NewEnforcerReply {
  type Msg = NewEnforcerReply;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NewEnforcerReply> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NewEnforcerReply {
  type Msg = NewEnforcerReply;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NewEnforcerReply> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NewEnforcerReplyMut<'_> {
  type Msg = NewEnforcerReply;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NewEnforcerReply> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NewEnforcerReplyMut<'_> {
  type Msg = NewEnforcerReply;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NewEnforcerReply> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NewEnforcerReplyView<'_> {
  type Msg = NewEnforcerReply;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NewEnforcerReply> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NewEnforcerReplyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__casbin__v1__NewAdapterRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct NewAdapterRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<NewAdapterRequest>
}

impl ::protobuf::Message for NewAdapterRequest {}

impl ::std::default::Default for NewAdapterRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for NewAdapterRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `NewAdapterRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `NewAdapterRequestMut`.
unsafe impl Sync for NewAdapterRequest {}

// SAFETY:
// - `NewAdapterRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for NewAdapterRequest {}

impl ::protobuf::Proxied for NewAdapterRequest {
  type View<'msg> = NewAdapterRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for NewAdapterRequest {}

impl ::protobuf::MutProxied for NewAdapterRequest {
  type Mut<'msg> = NewAdapterRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct NewAdapterRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NewAdapterRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NewAdapterRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for NewAdapterRequestView<'msg> {
  type Message = NewAdapterRequest;
}

impl ::std::fmt::Debug for NewAdapterRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for NewAdapterRequestView<'_> {
  fn default() -> NewAdapterRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, NewAdapterRequest>> for NewAdapterRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NewAdapterRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NewAdapterRequestView<'msg> {

  pub fn to_owned(&self) -> NewAdapterRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // adapterName: optional string
  pub fn adapterName(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // driverName: optional string
  pub fn driverName(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // connectString: optional string
  pub fn connectString(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // dbSpecified: optional bool
  pub fn dbSpecified(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `NewAdapterRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for NewAdapterRequestView<'_> {}

// SAFETY:
// - `NewAdapterRequestView` is `Send` because while its alive a `NewAdapterRequestMut` cannot.
// - `NewAdapterRequestView` does not use thread-local data.
unsafe impl Send for NewAdapterRequestView<'_> {}

impl<'msg> ::protobuf::AsView for NewAdapterRequestView<'msg> {
  type Proxied = NewAdapterRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, NewAdapterRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NewAdapterRequestView<'msg> {
  fn into_view<'shorter>(self) -> NewAdapterRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<NewAdapterRequest> for NewAdapterRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NewAdapterRequest {
    let mut dst = NewAdapterRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<NewAdapterRequest> for NewAdapterRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NewAdapterRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for NewAdapterRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for NewAdapterRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for NewAdapterRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct NewAdapterRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NewAdapterRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NewAdapterRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for NewAdapterRequestMut<'msg> {
  type Message = NewAdapterRequest;
}

impl ::std::fmt::Debug for NewAdapterRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, NewAdapterRequest>> for NewAdapterRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NewAdapterRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NewAdapterRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, NewAdapterRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> NewAdapterRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // adapterName: optional string
  pub fn adapterName(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_adapterName(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // driverName: optional string
  pub fn driverName(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_driverName(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // connectString: optional string
  pub fn connectString(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_connectString(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // dbSpecified: optional bool
  pub fn dbSpecified(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_dbSpecified(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

}

// SAFETY:
// - `NewAdapterRequestMut` does not perform any shared mutation.
unsafe impl Send for NewAdapterRequestMut<'_> {}

// SAFETY:
// - `NewAdapterRequestMut` does not perform any shared mutation.
unsafe impl Sync for NewAdapterRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for NewAdapterRequestMut<'msg> {
  type Proxied = NewAdapterRequest;
  fn as_view(&self) -> ::protobuf::View<'_, NewAdapterRequest> {
    NewAdapterRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NewAdapterRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, NewAdapterRequest>
  where
      'msg: 'shorter {
    NewAdapterRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for NewAdapterRequestMut<'msg> {
  type MutProxied = NewAdapterRequest;
  fn as_mut(&mut self) -> NewAdapterRequestMut<'msg> {
    NewAdapterRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for NewAdapterRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> NewAdapterRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl NewAdapterRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, NewAdapterRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> NewAdapterRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> NewAdapterRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // adapterName: optional string
  pub fn adapterName(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_adapterName(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // driverName: optional string
  pub fn driverName(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_driverName(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // connectString: optional string
  pub fn connectString(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_connectString(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // dbSpecified: optional bool
  pub fn dbSpecified(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_dbSpecified(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

}  // impl NewAdapterRequest

impl ::std::ops::Drop for NewAdapterRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for NewAdapterRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for NewAdapterRequest {
  type Proxied = Self;
  fn as_view(&self) -> NewAdapterRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for NewAdapterRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> NewAdapterRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for NewAdapterRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__casbin__v1__NewAdapterRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X1X/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__casbin__v1__NewAdapterRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__casbin__v1__NewAdapterRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NewAdapterRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NewAdapterRequest {
  type Msg = NewAdapterRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NewAdapterRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NewAdapterRequest {
  type Msg = NewAdapterRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NewAdapterRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NewAdapterRequestMut<'_> {
  type Msg = NewAdapterRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NewAdapterRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NewAdapterRequestMut<'_> {
  type Msg = NewAdapterRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NewAdapterRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NewAdapterRequestView<'_> {
  type Msg = NewAdapterRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NewAdapterRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NewAdapterRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__casbin__v1__NewAdapterReply_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct NewAdapterReply {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<NewAdapterReply>
}

impl ::protobuf::Message for NewAdapterReply {}

impl ::std::default::Default for NewAdapterReply {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for NewAdapterReply {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `NewAdapterReply` is `Sync` because it does not implement interior mutability.
//    Neither does `NewAdapterReplyMut`.
unsafe impl Sync for NewAdapterReply {}

// SAFETY:
// - `NewAdapterReply` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for NewAdapterReply {}

impl ::protobuf::Proxied for NewAdapterReply {
  type View<'msg> = NewAdapterReplyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for NewAdapterReply {}

impl ::protobuf::MutProxied for NewAdapterReply {
  type Mut<'msg> = NewAdapterReplyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct NewAdapterReplyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NewAdapterReply>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NewAdapterReplyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for NewAdapterReplyView<'msg> {
  type Message = NewAdapterReply;
}

impl ::std::fmt::Debug for NewAdapterReplyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for NewAdapterReplyView<'_> {
  fn default() -> NewAdapterReplyView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, NewAdapterReply>> for NewAdapterReplyView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NewAdapterReply>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NewAdapterReplyView<'msg> {

  pub fn to_owned(&self) -> NewAdapterReply {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // handler: optional int32
  pub fn handler(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `NewAdapterReplyView` is `Sync` because it does not support mutation.
unsafe impl Sync for NewAdapterReplyView<'_> {}

// SAFETY:
// - `NewAdapterReplyView` is `Send` because while its alive a `NewAdapterReplyMut` cannot.
// - `NewAdapterReplyView` does not use thread-local data.
unsafe impl Send for NewAdapterReplyView<'_> {}

impl<'msg> ::protobuf::AsView for NewAdapterReplyView<'msg> {
  type Proxied = NewAdapterReply;
  fn as_view(&self) -> ::protobuf::View<'msg, NewAdapterReply> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NewAdapterReplyView<'msg> {
  fn into_view<'shorter>(self) -> NewAdapterReplyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<NewAdapterReply> for NewAdapterReplyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NewAdapterReply {
    let mut dst = NewAdapterReply::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<NewAdapterReply> for NewAdapterReplyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NewAdapterReply {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for NewAdapterReply {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for NewAdapterReplyView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for NewAdapterReplyMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct NewAdapterReplyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NewAdapterReply>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NewAdapterReplyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for NewAdapterReplyMut<'msg> {
  type Message = NewAdapterReply;
}

impl ::std::fmt::Debug for NewAdapterReplyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, NewAdapterReply>> for NewAdapterReplyMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NewAdapterReply>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NewAdapterReplyMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, NewAdapterReply> {
    self.inner
  }

  pub fn to_owned(&self) -> NewAdapterReply {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // handler: optional int32
  pub fn handler(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_handler(&mut self, val: i32) {
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

}

// SAFETY:
// - `NewAdapterReplyMut` does not perform any shared mutation.
unsafe impl Send for NewAdapterReplyMut<'_> {}

// SAFETY:
// - `NewAdapterReplyMut` does not perform any shared mutation.
unsafe impl Sync for NewAdapterReplyMut<'_> {}

impl<'msg> ::protobuf::AsView for NewAdapterReplyMut<'msg> {
  type Proxied = NewAdapterReply;
  fn as_view(&self) -> ::protobuf::View<'_, NewAdapterReply> {
    NewAdapterReplyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NewAdapterReplyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, NewAdapterReply>
  where
      'msg: 'shorter {
    NewAdapterReplyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for NewAdapterReplyMut<'msg> {
  type MutProxied = NewAdapterReply;
  fn as_mut(&mut self) -> NewAdapterReplyMut<'msg> {
    NewAdapterReplyMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for NewAdapterReplyMut<'msg> {
  fn into_mut<'shorter>(self) -> NewAdapterReplyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl NewAdapterReply {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, NewAdapterReply> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> NewAdapterReplyView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> NewAdapterReplyMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // handler: optional int32
  pub fn handler(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_handler(&mut self, val: i32) {
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

}  // impl NewAdapterReply

impl ::std::ops::Drop for NewAdapterReply {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for NewAdapterReply {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for NewAdapterReply {
  type Proxied = Self;
  fn as_view(&self) -> NewAdapterReplyView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for NewAdapterReply {
  type MutProxied = Self;
  fn as_mut(&mut self) -> NewAdapterReplyMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for NewAdapterReply {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__casbin__v1__NewAdapterReply_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$(P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__casbin__v1__NewAdapterReply_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__casbin__v1__NewAdapterReply_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NewAdapterReply {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NewAdapterReply {
  type Msg = NewAdapterReply;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NewAdapterReply> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NewAdapterReply {
  type Msg = NewAdapterReply;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NewAdapterReply> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NewAdapterReplyMut<'_> {
  type Msg = NewAdapterReply;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NewAdapterReply> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NewAdapterReplyMut<'_> {
  type Msg = NewAdapterReply;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NewAdapterReply> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NewAdapterReplyView<'_> {
  type Msg = NewAdapterReply;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NewAdapterReply> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NewAdapterReplyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__casbin__v1__EnforceRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct EnforceRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<EnforceRequest>
}

impl ::protobuf::Message for EnforceRequest {}

impl ::std::default::Default for EnforceRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for EnforceRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `EnforceRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `EnforceRequestMut`.
unsafe impl Sync for EnforceRequest {}

// SAFETY:
// - `EnforceRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for EnforceRequest {}

impl ::protobuf::Proxied for EnforceRequest {
  type View<'msg> = EnforceRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for EnforceRequest {}

impl ::protobuf::MutProxied for EnforceRequest {
  type Mut<'msg> = EnforceRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct EnforceRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EnforceRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EnforceRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for EnforceRequestView<'msg> {
  type Message = EnforceRequest;
}

impl ::std::fmt::Debug for EnforceRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for EnforceRequestView<'_> {
  fn default() -> EnforceRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, EnforceRequest>> for EnforceRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EnforceRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EnforceRequestView<'msg> {

  pub fn to_owned(&self) -> EnforceRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // enforcerHandler: optional int32
  pub fn enforcerHandler(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // params: repeated string
  pub fn params(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `EnforceRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for EnforceRequestView<'_> {}

// SAFETY:
// - `EnforceRequestView` is `Send` because while its alive a `EnforceRequestMut` cannot.
// - `EnforceRequestView` does not use thread-local data.
unsafe impl Send for EnforceRequestView<'_> {}

impl<'msg> ::protobuf::AsView for EnforceRequestView<'msg> {
  type Proxied = EnforceRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, EnforceRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EnforceRequestView<'msg> {
  fn into_view<'shorter>(self) -> EnforceRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<EnforceRequest> for EnforceRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EnforceRequest {
    let mut dst = EnforceRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<EnforceRequest> for EnforceRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EnforceRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for EnforceRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for EnforceRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for EnforceRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct EnforceRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EnforceRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EnforceRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for EnforceRequestMut<'msg> {
  type Message = EnforceRequest;
}

impl ::std::fmt::Debug for EnforceRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, EnforceRequest>> for EnforceRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EnforceRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EnforceRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, EnforceRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> EnforceRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // enforcerHandler: optional int32
  pub fn enforcerHandler(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enforcerHandler(&mut self, val: i32) {
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

  // params: repeated string
  pub fn params(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn params_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
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
  pub fn set_params(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `EnforceRequestMut` does not perform any shared mutation.
unsafe impl Send for EnforceRequestMut<'_> {}

// SAFETY:
// - `EnforceRequestMut` does not perform any shared mutation.
unsafe impl Sync for EnforceRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for EnforceRequestMut<'msg> {
  type Proxied = EnforceRequest;
  fn as_view(&self) -> ::protobuf::View<'_, EnforceRequest> {
    EnforceRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EnforceRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, EnforceRequest>
  where
      'msg: 'shorter {
    EnforceRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for EnforceRequestMut<'msg> {
  type MutProxied = EnforceRequest;
  fn as_mut(&mut self) -> EnforceRequestMut<'msg> {
    EnforceRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for EnforceRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> EnforceRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl EnforceRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, EnforceRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> EnforceRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> EnforceRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // enforcerHandler: optional int32
  pub fn enforcerHandler(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enforcerHandler(&mut self, val: i32) {
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

  // params: repeated string
  pub fn params(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn params_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
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
  pub fn set_params(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl EnforceRequest

impl ::std::ops::Drop for EnforceRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for EnforceRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for EnforceRequest {
  type Proxied = Self;
  fn as_view(&self) -> EnforceRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for EnforceRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> EnforceRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for EnforceRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__casbin__v1__EnforceRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$(PET");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__casbin__v1__EnforceRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__casbin__v1__EnforceRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EnforceRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EnforceRequest {
  type Msg = EnforceRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnforceRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EnforceRequest {
  type Msg = EnforceRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnforceRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EnforceRequestMut<'_> {
  type Msg = EnforceRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnforceRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EnforceRequestMut<'_> {
  type Msg = EnforceRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnforceRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EnforceRequestView<'_> {
  type Msg = EnforceRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnforceRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EnforceRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__casbin__v1__BoolReply_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BoolReply {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BoolReply>
}

impl ::protobuf::Message for BoolReply {}

impl ::std::default::Default for BoolReply {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BoolReply {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BoolReply` is `Sync` because it does not implement interior mutability.
//    Neither does `BoolReplyMut`.
unsafe impl Sync for BoolReply {}

// SAFETY:
// - `BoolReply` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for BoolReply {}

impl ::protobuf::Proxied for BoolReply {
  type View<'msg> = BoolReplyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BoolReply {}

impl ::protobuf::MutProxied for BoolReply {
  type Mut<'msg> = BoolReplyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BoolReplyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BoolReply>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BoolReplyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BoolReplyView<'msg> {
  type Message = BoolReply;
}

impl ::std::fmt::Debug for BoolReplyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BoolReplyView<'_> {
  fn default() -> BoolReplyView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BoolReply>> for BoolReplyView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BoolReply>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BoolReplyView<'msg> {

  pub fn to_owned(&self) -> BoolReply {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // res: optional bool
  pub fn res(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `BoolReplyView` is `Sync` because it does not support mutation.
unsafe impl Sync for BoolReplyView<'_> {}

// SAFETY:
// - `BoolReplyView` is `Send` because while its alive a `BoolReplyMut` cannot.
// - `BoolReplyView` does not use thread-local data.
unsafe impl Send for BoolReplyView<'_> {}

impl<'msg> ::protobuf::AsView for BoolReplyView<'msg> {
  type Proxied = BoolReply;
  fn as_view(&self) -> ::protobuf::View<'msg, BoolReply> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BoolReplyView<'msg> {
  fn into_view<'shorter>(self) -> BoolReplyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BoolReply> for BoolReplyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BoolReply {
    let mut dst = BoolReply::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BoolReply> for BoolReplyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BoolReply {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for BoolReply {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for BoolReplyView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for BoolReplyMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BoolReplyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BoolReply>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BoolReplyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BoolReplyMut<'msg> {
  type Message = BoolReply;
}

impl ::std::fmt::Debug for BoolReplyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BoolReply>> for BoolReplyMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BoolReply>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BoolReplyMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BoolReply> {
    self.inner
  }

  pub fn to_owned(&self) -> BoolReply {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // res: optional bool
  pub fn res(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_res(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        0, val.into()
      )
    }
  }

}

// SAFETY:
// - `BoolReplyMut` does not perform any shared mutation.
unsafe impl Send for BoolReplyMut<'_> {}

// SAFETY:
// - `BoolReplyMut` does not perform any shared mutation.
unsafe impl Sync for BoolReplyMut<'_> {}

impl<'msg> ::protobuf::AsView for BoolReplyMut<'msg> {
  type Proxied = BoolReply;
  fn as_view(&self) -> ::protobuf::View<'_, BoolReply> {
    BoolReplyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BoolReplyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BoolReply>
  where
      'msg: 'shorter {
    BoolReplyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for BoolReplyMut<'msg> {
  type MutProxied = BoolReply;
  fn as_mut(&mut self) -> BoolReplyMut<'msg> {
    BoolReplyMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BoolReplyMut<'msg> {
  fn into_mut<'shorter>(self) -> BoolReplyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BoolReply {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BoolReply> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BoolReplyView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BoolReplyMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // res: optional bool
  pub fn res(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_res(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        0, val.into()
      )
    }
  }

}  // impl BoolReply

impl ::std::ops::Drop for BoolReply {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BoolReply {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BoolReply {
  type Proxied = Self;
  fn as_view(&self) -> BoolReplyView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BoolReply {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BoolReplyMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BoolReply {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__casbin__v1__BoolReply_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__casbin__v1__BoolReply_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__casbin__v1__BoolReply_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BoolReply {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BoolReply {
  type Msg = BoolReply;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BoolReply> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BoolReply {
  type Msg = BoolReply;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BoolReply> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BoolReplyMut<'_> {
  type Msg = BoolReply;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BoolReply> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BoolReplyMut<'_> {
  type Msg = BoolReply;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BoolReply> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BoolReplyView<'_> {
  type Msg = BoolReply;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BoolReply> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BoolReplyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__casbin__v1__EmptyRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct EmptyRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<EmptyRequest>
}

impl ::protobuf::Message for EmptyRequest {}

impl ::std::default::Default for EmptyRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for EmptyRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `EmptyRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `EmptyRequestMut`.
unsafe impl Sync for EmptyRequest {}

// SAFETY:
// - `EmptyRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for EmptyRequest {}

impl ::protobuf::Proxied for EmptyRequest {
  type View<'msg> = EmptyRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for EmptyRequest {}

impl ::protobuf::MutProxied for EmptyRequest {
  type Mut<'msg> = EmptyRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct EmptyRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EmptyRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EmptyRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for EmptyRequestView<'msg> {
  type Message = EmptyRequest;
}

impl ::std::fmt::Debug for EmptyRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for EmptyRequestView<'_> {
  fn default() -> EmptyRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, EmptyRequest>> for EmptyRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EmptyRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EmptyRequestView<'msg> {

  pub fn to_owned(&self) -> EmptyRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // handler: optional int32
  pub fn handler(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `EmptyRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for EmptyRequestView<'_> {}

// SAFETY:
// - `EmptyRequestView` is `Send` because while its alive a `EmptyRequestMut` cannot.
// - `EmptyRequestView` does not use thread-local data.
unsafe impl Send for EmptyRequestView<'_> {}

impl<'msg> ::protobuf::AsView for EmptyRequestView<'msg> {
  type Proxied = EmptyRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, EmptyRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EmptyRequestView<'msg> {
  fn into_view<'shorter>(self) -> EmptyRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<EmptyRequest> for EmptyRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EmptyRequest {
    let mut dst = EmptyRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<EmptyRequest> for EmptyRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EmptyRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for EmptyRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for EmptyRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for EmptyRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct EmptyRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EmptyRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EmptyRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for EmptyRequestMut<'msg> {
  type Message = EmptyRequest;
}

impl ::std::fmt::Debug for EmptyRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, EmptyRequest>> for EmptyRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EmptyRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EmptyRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, EmptyRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> EmptyRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // handler: optional int32
  pub fn handler(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_handler(&mut self, val: i32) {
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

}

// SAFETY:
// - `EmptyRequestMut` does not perform any shared mutation.
unsafe impl Send for EmptyRequestMut<'_> {}

// SAFETY:
// - `EmptyRequestMut` does not perform any shared mutation.
unsafe impl Sync for EmptyRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for EmptyRequestMut<'msg> {
  type Proxied = EmptyRequest;
  fn as_view(&self) -> ::protobuf::View<'_, EmptyRequest> {
    EmptyRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EmptyRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, EmptyRequest>
  where
      'msg: 'shorter {
    EmptyRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for EmptyRequestMut<'msg> {
  type MutProxied = EmptyRequest;
  fn as_mut(&mut self) -> EmptyRequestMut<'msg> {
    EmptyRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for EmptyRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> EmptyRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl EmptyRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, EmptyRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> EmptyRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> EmptyRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // handler: optional int32
  pub fn handler(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_handler(&mut self, val: i32) {
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

}  // impl EmptyRequest

impl ::std::ops::Drop for EmptyRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for EmptyRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for EmptyRequest {
  type Proxied = Self;
  fn as_view(&self) -> EmptyRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for EmptyRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> EmptyRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for EmptyRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__casbin__v1__EmptyRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$(P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__casbin__v1__EmptyRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__casbin__v1__EmptyRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EmptyRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EmptyRequest {
  type Msg = EmptyRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EmptyRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EmptyRequest {
  type Msg = EmptyRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EmptyRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EmptyRequestMut<'_> {
  type Msg = EmptyRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EmptyRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EmptyRequestMut<'_> {
  type Msg = EmptyRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EmptyRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EmptyRequestView<'_> {
  type Msg = EmptyRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EmptyRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EmptyRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__casbin__v1__EmptyReply_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct EmptyReply {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<EmptyReply>
}

impl ::protobuf::Message for EmptyReply {}

impl ::std::default::Default for EmptyReply {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for EmptyReply {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `EmptyReply` is `Sync` because it does not implement interior mutability.
//    Neither does `EmptyReplyMut`.
unsafe impl Sync for EmptyReply {}

// SAFETY:
// - `EmptyReply` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for EmptyReply {}

impl ::protobuf::Proxied for EmptyReply {
  type View<'msg> = EmptyReplyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for EmptyReply {}

impl ::protobuf::MutProxied for EmptyReply {
  type Mut<'msg> = EmptyReplyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct EmptyReplyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EmptyReply>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EmptyReplyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for EmptyReplyView<'msg> {
  type Message = EmptyReply;
}

impl ::std::fmt::Debug for EmptyReplyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for EmptyReplyView<'_> {
  fn default() -> EmptyReplyView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, EmptyReply>> for EmptyReplyView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EmptyReply>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EmptyReplyView<'msg> {

  pub fn to_owned(&self) -> EmptyReply {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `EmptyReplyView` is `Sync` because it does not support mutation.
unsafe impl Sync for EmptyReplyView<'_> {}

// SAFETY:
// - `EmptyReplyView` is `Send` because while its alive a `EmptyReplyMut` cannot.
// - `EmptyReplyView` does not use thread-local data.
unsafe impl Send for EmptyReplyView<'_> {}

impl<'msg> ::protobuf::AsView for EmptyReplyView<'msg> {
  type Proxied = EmptyReply;
  fn as_view(&self) -> ::protobuf::View<'msg, EmptyReply> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EmptyReplyView<'msg> {
  fn into_view<'shorter>(self) -> EmptyReplyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<EmptyReply> for EmptyReplyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EmptyReply {
    let mut dst = EmptyReply::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<EmptyReply> for EmptyReplyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EmptyReply {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for EmptyReply {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for EmptyReplyView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for EmptyReplyMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct EmptyReplyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EmptyReply>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EmptyReplyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for EmptyReplyMut<'msg> {
  type Message = EmptyReply;
}

impl ::std::fmt::Debug for EmptyReplyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, EmptyReply>> for EmptyReplyMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EmptyReply>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EmptyReplyMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, EmptyReply> {
    self.inner
  }

  pub fn to_owned(&self) -> EmptyReply {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `EmptyReplyMut` does not perform any shared mutation.
unsafe impl Send for EmptyReplyMut<'_> {}

// SAFETY:
// - `EmptyReplyMut` does not perform any shared mutation.
unsafe impl Sync for EmptyReplyMut<'_> {}

impl<'msg> ::protobuf::AsView for EmptyReplyMut<'msg> {
  type Proxied = EmptyReply;
  fn as_view(&self) -> ::protobuf::View<'_, EmptyReply> {
    EmptyReplyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EmptyReplyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, EmptyReply>
  where
      'msg: 'shorter {
    EmptyReplyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for EmptyReplyMut<'msg> {
  type MutProxied = EmptyReply;
  fn as_mut(&mut self) -> EmptyReplyMut<'msg> {
    EmptyReplyMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for EmptyReplyMut<'msg> {
  fn into_mut<'shorter>(self) -> EmptyReplyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl EmptyReply {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, EmptyReply> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> EmptyReplyView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> EmptyReplyMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl EmptyReply

impl ::std::ops::Drop for EmptyReply {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for EmptyReply {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for EmptyReply {
  type Proxied = Self;
  fn as_view(&self) -> EmptyReplyView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for EmptyReply {
  type MutProxied = Self;
  fn as_mut(&mut self) -> EmptyReplyMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for EmptyReply {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__casbin__v1__EmptyReply_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__casbin__v1__EmptyReply_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__casbin__v1__EmptyReply_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EmptyReply {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EmptyReply {
  type Msg = EmptyReply;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EmptyReply> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EmptyReply {
  type Msg = EmptyReply;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EmptyReply> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EmptyReplyMut<'_> {
  type Msg = EmptyReply;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EmptyReply> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EmptyReplyMut<'_> {
  type Msg = EmptyReply;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EmptyReply> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EmptyReplyView<'_> {
  type Msg = EmptyReply;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EmptyReply> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EmptyReplyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__casbin__v1__PolicyRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PolicyRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PolicyRequest>
}

impl ::protobuf::Message for PolicyRequest {}

impl ::std::default::Default for PolicyRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PolicyRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PolicyRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `PolicyRequestMut`.
unsafe impl Sync for PolicyRequest {}

// SAFETY:
// - `PolicyRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PolicyRequest {}

impl ::protobuf::Proxied for PolicyRequest {
  type View<'msg> = PolicyRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PolicyRequest {}

impl ::protobuf::MutProxied for PolicyRequest {
  type Mut<'msg> = PolicyRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PolicyRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PolicyRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PolicyRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PolicyRequestView<'msg> {
  type Message = PolicyRequest;
}

impl ::std::fmt::Debug for PolicyRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PolicyRequestView<'_> {
  fn default() -> PolicyRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PolicyRequest>> for PolicyRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PolicyRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PolicyRequestView<'msg> {

  pub fn to_owned(&self) -> PolicyRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // enforcerHandler: optional int32
  pub fn enforcerHandler(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // pType: optional string
  pub fn pType(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // params: repeated string
  pub fn params(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `PolicyRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for PolicyRequestView<'_> {}

// SAFETY:
// - `PolicyRequestView` is `Send` because while its alive a `PolicyRequestMut` cannot.
// - `PolicyRequestView` does not use thread-local data.
unsafe impl Send for PolicyRequestView<'_> {}

impl<'msg> ::protobuf::AsView for PolicyRequestView<'msg> {
  type Proxied = PolicyRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, PolicyRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PolicyRequestView<'msg> {
  fn into_view<'shorter>(self) -> PolicyRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PolicyRequest> for PolicyRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PolicyRequest {
    let mut dst = PolicyRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PolicyRequest> for PolicyRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PolicyRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for PolicyRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PolicyRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PolicyRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PolicyRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PolicyRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PolicyRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PolicyRequestMut<'msg> {
  type Message = PolicyRequest;
}

impl ::std::fmt::Debug for PolicyRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PolicyRequest>> for PolicyRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PolicyRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PolicyRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PolicyRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> PolicyRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // enforcerHandler: optional int32
  pub fn enforcerHandler(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enforcerHandler(&mut self, val: i32) {
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

  // pType: optional string
  pub fn pType(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_pType(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // params: repeated string
  pub fn params(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn params_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_params(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}

// SAFETY:
// - `PolicyRequestMut` does not perform any shared mutation.
unsafe impl Send for PolicyRequestMut<'_> {}

// SAFETY:
// - `PolicyRequestMut` does not perform any shared mutation.
unsafe impl Sync for PolicyRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for PolicyRequestMut<'msg> {
  type Proxied = PolicyRequest;
  fn as_view(&self) -> ::protobuf::View<'_, PolicyRequest> {
    PolicyRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PolicyRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PolicyRequest>
  where
      'msg: 'shorter {
    PolicyRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for PolicyRequestMut<'msg> {
  type MutProxied = PolicyRequest;
  fn as_mut(&mut self) -> PolicyRequestMut<'msg> {
    PolicyRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PolicyRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> PolicyRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PolicyRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PolicyRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PolicyRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PolicyRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // enforcerHandler: optional int32
  pub fn enforcerHandler(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enforcerHandler(&mut self, val: i32) {
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

  // pType: optional string
  pub fn pType(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_pType(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // params: repeated string
  pub fn params(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn params_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_params(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}  // impl PolicyRequest

impl ::std::ops::Drop for PolicyRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PolicyRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PolicyRequest {
  type Proxied = Self;
  fn as_view(&self) -> PolicyRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PolicyRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PolicyRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PolicyRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__casbin__v1__PolicyRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$(P1XET");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__casbin__v1__PolicyRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__casbin__v1__PolicyRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PolicyRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PolicyRequest {
  type Msg = PolicyRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PolicyRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PolicyRequest {
  type Msg = PolicyRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PolicyRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PolicyRequestMut<'_> {
  type Msg = PolicyRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PolicyRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PolicyRequestMut<'_> {
  type Msg = PolicyRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PolicyRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PolicyRequestView<'_> {
  type Msg = PolicyRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PolicyRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PolicyRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__casbin__v1__SimpleGetRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SimpleGetRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SimpleGetRequest>
}

impl ::protobuf::Message for SimpleGetRequest {}

impl ::std::default::Default for SimpleGetRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SimpleGetRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SimpleGetRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `SimpleGetRequestMut`.
unsafe impl Sync for SimpleGetRequest {}

// SAFETY:
// - `SimpleGetRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SimpleGetRequest {}

impl ::protobuf::Proxied for SimpleGetRequest {
  type View<'msg> = SimpleGetRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SimpleGetRequest {}

impl ::protobuf::MutProxied for SimpleGetRequest {
  type Mut<'msg> = SimpleGetRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SimpleGetRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SimpleGetRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SimpleGetRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SimpleGetRequestView<'msg> {
  type Message = SimpleGetRequest;
}

impl ::std::fmt::Debug for SimpleGetRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SimpleGetRequestView<'_> {
  fn default() -> SimpleGetRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SimpleGetRequest>> for SimpleGetRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SimpleGetRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SimpleGetRequestView<'msg> {

  pub fn to_owned(&self) -> SimpleGetRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // enforcerHandler: optional int32
  pub fn enforcerHandler(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // pType: optional string
  pub fn pType(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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
// - `SimpleGetRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for SimpleGetRequestView<'_> {}

// SAFETY:
// - `SimpleGetRequestView` is `Send` because while its alive a `SimpleGetRequestMut` cannot.
// - `SimpleGetRequestView` does not use thread-local data.
unsafe impl Send for SimpleGetRequestView<'_> {}

impl<'msg> ::protobuf::AsView for SimpleGetRequestView<'msg> {
  type Proxied = SimpleGetRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, SimpleGetRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SimpleGetRequestView<'msg> {
  fn into_view<'shorter>(self) -> SimpleGetRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SimpleGetRequest> for SimpleGetRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SimpleGetRequest {
    let mut dst = SimpleGetRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SimpleGetRequest> for SimpleGetRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SimpleGetRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for SimpleGetRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SimpleGetRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SimpleGetRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SimpleGetRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SimpleGetRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SimpleGetRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SimpleGetRequestMut<'msg> {
  type Message = SimpleGetRequest;
}

impl ::std::fmt::Debug for SimpleGetRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SimpleGetRequest>> for SimpleGetRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SimpleGetRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SimpleGetRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SimpleGetRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> SimpleGetRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // enforcerHandler: optional int32
  pub fn enforcerHandler(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enforcerHandler(&mut self, val: i32) {
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

  // pType: optional string
  pub fn pType(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_pType(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `SimpleGetRequestMut` does not perform any shared mutation.
unsafe impl Send for SimpleGetRequestMut<'_> {}

// SAFETY:
// - `SimpleGetRequestMut` does not perform any shared mutation.
unsafe impl Sync for SimpleGetRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for SimpleGetRequestMut<'msg> {
  type Proxied = SimpleGetRequest;
  fn as_view(&self) -> ::protobuf::View<'_, SimpleGetRequest> {
    SimpleGetRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SimpleGetRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SimpleGetRequest>
  where
      'msg: 'shorter {
    SimpleGetRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for SimpleGetRequestMut<'msg> {
  type MutProxied = SimpleGetRequest;
  fn as_mut(&mut self) -> SimpleGetRequestMut<'msg> {
    SimpleGetRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SimpleGetRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> SimpleGetRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SimpleGetRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SimpleGetRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SimpleGetRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SimpleGetRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // enforcerHandler: optional int32
  pub fn enforcerHandler(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enforcerHandler(&mut self, val: i32) {
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

  // pType: optional string
  pub fn pType(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_pType(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl SimpleGetRequest

impl ::std::ops::Drop for SimpleGetRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SimpleGetRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SimpleGetRequest {
  type Proxied = Self;
  fn as_view(&self) -> SimpleGetRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SimpleGetRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SimpleGetRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SimpleGetRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__casbin__v1__SimpleGetRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$(P1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__casbin__v1__SimpleGetRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__casbin__v1__SimpleGetRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SimpleGetRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SimpleGetRequest {
  type Msg = SimpleGetRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SimpleGetRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SimpleGetRequest {
  type Msg = SimpleGetRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SimpleGetRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SimpleGetRequestMut<'_> {
  type Msg = SimpleGetRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SimpleGetRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SimpleGetRequestMut<'_> {
  type Msg = SimpleGetRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SimpleGetRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SimpleGetRequestView<'_> {
  type Msg = SimpleGetRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SimpleGetRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SimpleGetRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__casbin__v1__ArrayReply_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ArrayReply {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ArrayReply>
}

impl ::protobuf::Message for ArrayReply {}

impl ::std::default::Default for ArrayReply {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ArrayReply {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ArrayReply` is `Sync` because it does not implement interior mutability.
//    Neither does `ArrayReplyMut`.
unsafe impl Sync for ArrayReply {}

// SAFETY:
// - `ArrayReply` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ArrayReply {}

impl ::protobuf::Proxied for ArrayReply {
  type View<'msg> = ArrayReplyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ArrayReply {}

impl ::protobuf::MutProxied for ArrayReply {
  type Mut<'msg> = ArrayReplyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ArrayReplyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ArrayReply>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ArrayReplyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ArrayReplyView<'msg> {
  type Message = ArrayReply;
}

impl ::std::fmt::Debug for ArrayReplyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ArrayReplyView<'_> {
  fn default() -> ArrayReplyView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ArrayReply>> for ArrayReplyView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ArrayReply>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ArrayReplyView<'msg> {

  pub fn to_owned(&self) -> ArrayReply {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // array: repeated string
  pub fn array(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ArrayReplyView` is `Sync` because it does not support mutation.
unsafe impl Sync for ArrayReplyView<'_> {}

// SAFETY:
// - `ArrayReplyView` is `Send` because while its alive a `ArrayReplyMut` cannot.
// - `ArrayReplyView` does not use thread-local data.
unsafe impl Send for ArrayReplyView<'_> {}

impl<'msg> ::protobuf::AsView for ArrayReplyView<'msg> {
  type Proxied = ArrayReply;
  fn as_view(&self) -> ::protobuf::View<'msg, ArrayReply> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ArrayReplyView<'msg> {
  fn into_view<'shorter>(self) -> ArrayReplyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ArrayReply> for ArrayReplyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ArrayReply {
    let mut dst = ArrayReply::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ArrayReply> for ArrayReplyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ArrayReply {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ArrayReply {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ArrayReplyView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ArrayReplyMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ArrayReplyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ArrayReply>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ArrayReplyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ArrayReplyMut<'msg> {
  type Message = ArrayReply;
}

impl ::std::fmt::Debug for ArrayReplyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ArrayReply>> for ArrayReplyMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ArrayReply>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ArrayReplyMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ArrayReply> {
    self.inner
  }

  pub fn to_owned(&self) -> ArrayReply {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // array: repeated string
  pub fn array(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn array_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
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
  pub fn set_array(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `ArrayReplyMut` does not perform any shared mutation.
unsafe impl Send for ArrayReplyMut<'_> {}

// SAFETY:
// - `ArrayReplyMut` does not perform any shared mutation.
unsafe impl Sync for ArrayReplyMut<'_> {}

impl<'msg> ::protobuf::AsView for ArrayReplyMut<'msg> {
  type Proxied = ArrayReply;
  fn as_view(&self) -> ::protobuf::View<'_, ArrayReply> {
    ArrayReplyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ArrayReplyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ArrayReply>
  where
      'msg: 'shorter {
    ArrayReplyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ArrayReplyMut<'msg> {
  type MutProxied = ArrayReply;
  fn as_mut(&mut self) -> ArrayReplyMut<'msg> {
    ArrayReplyMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ArrayReplyMut<'msg> {
  fn into_mut<'shorter>(self) -> ArrayReplyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ArrayReply {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ArrayReply> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ArrayReplyView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ArrayReplyMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // array: repeated string
  pub fn array(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn array_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
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
  pub fn set_array(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl ArrayReply

impl ::std::ops::Drop for ArrayReply {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ArrayReply {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ArrayReply {
  type Proxied = Self;
  fn as_view(&self) -> ArrayReplyView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ArrayReply {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ArrayReplyMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ArrayReply {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__casbin__v1__ArrayReply_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$ME");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__casbin__v1__ArrayReply_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__casbin__v1__ArrayReply_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ArrayReply {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ArrayReply {
  type Msg = ArrayReply;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArrayReply> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ArrayReply {
  type Msg = ArrayReply;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArrayReply> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ArrayReplyMut<'_> {
  type Msg = ArrayReply;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArrayReply> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ArrayReplyMut<'_> {
  type Msg = ArrayReply;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArrayReply> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ArrayReplyView<'_> {
  type Msg = ArrayReply;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArrayReply> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ArrayReplyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__casbin__v1__FilteredPolicyRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FilteredPolicyRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FilteredPolicyRequest>
}

impl ::protobuf::Message for FilteredPolicyRequest {}

impl ::std::default::Default for FilteredPolicyRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FilteredPolicyRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FilteredPolicyRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `FilteredPolicyRequestMut`.
unsafe impl Sync for FilteredPolicyRequest {}

// SAFETY:
// - `FilteredPolicyRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for FilteredPolicyRequest {}

impl ::protobuf::Proxied for FilteredPolicyRequest {
  type View<'msg> = FilteredPolicyRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FilteredPolicyRequest {}

impl ::protobuf::MutProxied for FilteredPolicyRequest {
  type Mut<'msg> = FilteredPolicyRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FilteredPolicyRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FilteredPolicyRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FilteredPolicyRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FilteredPolicyRequestView<'msg> {
  type Message = FilteredPolicyRequest;
}

impl ::std::fmt::Debug for FilteredPolicyRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FilteredPolicyRequestView<'_> {
  fn default() -> FilteredPolicyRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FilteredPolicyRequest>> for FilteredPolicyRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FilteredPolicyRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FilteredPolicyRequestView<'msg> {

  pub fn to_owned(&self) -> FilteredPolicyRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // enforcerHandler: optional int32
  pub fn enforcerHandler(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // pType: optional string
  pub fn pType(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // fieldIndex: optional int32
  pub fn fieldIndex(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // fieldValues: repeated string
  pub fn fieldValues(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `FilteredPolicyRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for FilteredPolicyRequestView<'_> {}

// SAFETY:
// - `FilteredPolicyRequestView` is `Send` because while its alive a `FilteredPolicyRequestMut` cannot.
// - `FilteredPolicyRequestView` does not use thread-local data.
unsafe impl Send for FilteredPolicyRequestView<'_> {}

impl<'msg> ::protobuf::AsView for FilteredPolicyRequestView<'msg> {
  type Proxied = FilteredPolicyRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, FilteredPolicyRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FilteredPolicyRequestView<'msg> {
  fn into_view<'shorter>(self) -> FilteredPolicyRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FilteredPolicyRequest> for FilteredPolicyRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FilteredPolicyRequest {
    let mut dst = FilteredPolicyRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FilteredPolicyRequest> for FilteredPolicyRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FilteredPolicyRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for FilteredPolicyRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for FilteredPolicyRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for FilteredPolicyRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FilteredPolicyRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FilteredPolicyRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FilteredPolicyRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FilteredPolicyRequestMut<'msg> {
  type Message = FilteredPolicyRequest;
}

impl ::std::fmt::Debug for FilteredPolicyRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FilteredPolicyRequest>> for FilteredPolicyRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FilteredPolicyRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FilteredPolicyRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FilteredPolicyRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> FilteredPolicyRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // enforcerHandler: optional int32
  pub fn enforcerHandler(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enforcerHandler(&mut self, val: i32) {
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

  // pType: optional string
  pub fn pType(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_pType(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // fieldIndex: optional int32
  pub fn fieldIndex(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_fieldIndex(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // fieldValues: repeated string
  pub fn fieldValues(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn fieldValues_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        3,
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
  pub fn set_fieldValues(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}

// SAFETY:
// - `FilteredPolicyRequestMut` does not perform any shared mutation.
unsafe impl Send for FilteredPolicyRequestMut<'_> {}

// SAFETY:
// - `FilteredPolicyRequestMut` does not perform any shared mutation.
unsafe impl Sync for FilteredPolicyRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for FilteredPolicyRequestMut<'msg> {
  type Proxied = FilteredPolicyRequest;
  fn as_view(&self) -> ::protobuf::View<'_, FilteredPolicyRequest> {
    FilteredPolicyRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FilteredPolicyRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FilteredPolicyRequest>
  where
      'msg: 'shorter {
    FilteredPolicyRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for FilteredPolicyRequestMut<'msg> {
  type MutProxied = FilteredPolicyRequest;
  fn as_mut(&mut self) -> FilteredPolicyRequestMut<'msg> {
    FilteredPolicyRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FilteredPolicyRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> FilteredPolicyRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FilteredPolicyRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FilteredPolicyRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FilteredPolicyRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FilteredPolicyRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // enforcerHandler: optional int32
  pub fn enforcerHandler(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enforcerHandler(&mut self, val: i32) {
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

  // pType: optional string
  pub fn pType(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_pType(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // fieldIndex: optional int32
  pub fn fieldIndex(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_fieldIndex(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // fieldValues: repeated string
  pub fn fieldValues(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn fieldValues_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        3,
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
  pub fn set_fieldValues(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}  // impl FilteredPolicyRequest

impl ::std::ops::Drop for FilteredPolicyRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FilteredPolicyRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FilteredPolicyRequest {
  type Proxied = Self;
  fn as_view(&self) -> FilteredPolicyRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FilteredPolicyRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FilteredPolicyRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FilteredPolicyRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__casbin__v1__FilteredPolicyRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$(P1X(PET");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__casbin__v1__FilteredPolicyRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__casbin__v1__FilteredPolicyRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FilteredPolicyRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FilteredPolicyRequest {
  type Msg = FilteredPolicyRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilteredPolicyRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FilteredPolicyRequest {
  type Msg = FilteredPolicyRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilteredPolicyRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FilteredPolicyRequestMut<'_> {
  type Msg = FilteredPolicyRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilteredPolicyRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FilteredPolicyRequestMut<'_> {
  type Msg = FilteredPolicyRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilteredPolicyRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FilteredPolicyRequestView<'_> {
  type Msg = FilteredPolicyRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilteredPolicyRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FilteredPolicyRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__casbin__v1__UserRoleRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct UserRoleRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<UserRoleRequest>
}

impl ::protobuf::Message for UserRoleRequest {}

impl ::std::default::Default for UserRoleRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for UserRoleRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `UserRoleRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `UserRoleRequestMut`.
unsafe impl Sync for UserRoleRequest {}

// SAFETY:
// - `UserRoleRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for UserRoleRequest {}

impl ::protobuf::Proxied for UserRoleRequest {
  type View<'msg> = UserRoleRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for UserRoleRequest {}

impl ::protobuf::MutProxied for UserRoleRequest {
  type Mut<'msg> = UserRoleRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UserRoleRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UserRoleRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UserRoleRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UserRoleRequestView<'msg> {
  type Message = UserRoleRequest;
}

impl ::std::fmt::Debug for UserRoleRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UserRoleRequestView<'_> {
  fn default() -> UserRoleRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, UserRoleRequest>> for UserRoleRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UserRoleRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UserRoleRequestView<'msg> {

  pub fn to_owned(&self) -> UserRoleRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // enforcerHandler: optional int32
  pub fn enforcerHandler(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // user: optional string
  pub fn user(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // role: optional string
  pub fn role(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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
// - `UserRoleRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for UserRoleRequestView<'_> {}

// SAFETY:
// - `UserRoleRequestView` is `Send` because while its alive a `UserRoleRequestMut` cannot.
// - `UserRoleRequestView` does not use thread-local data.
unsafe impl Send for UserRoleRequestView<'_> {}

impl<'msg> ::protobuf::AsView for UserRoleRequestView<'msg> {
  type Proxied = UserRoleRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, UserRoleRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UserRoleRequestView<'msg> {
  fn into_view<'shorter>(self) -> UserRoleRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<UserRoleRequest> for UserRoleRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UserRoleRequest {
    let mut dst = UserRoleRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<UserRoleRequest> for UserRoleRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UserRoleRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for UserRoleRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for UserRoleRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for UserRoleRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UserRoleRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UserRoleRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UserRoleRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UserRoleRequestMut<'msg> {
  type Message = UserRoleRequest;
}

impl ::std::fmt::Debug for UserRoleRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, UserRoleRequest>> for UserRoleRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UserRoleRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UserRoleRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, UserRoleRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> UserRoleRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // enforcerHandler: optional int32
  pub fn enforcerHandler(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enforcerHandler(&mut self, val: i32) {
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

  // user: optional string
  pub fn user(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_user(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // role: optional string
  pub fn role(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_role(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `UserRoleRequestMut` does not perform any shared mutation.
unsafe impl Send for UserRoleRequestMut<'_> {}

// SAFETY:
// - `UserRoleRequestMut` does not perform any shared mutation.
unsafe impl Sync for UserRoleRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for UserRoleRequestMut<'msg> {
  type Proxied = UserRoleRequest;
  fn as_view(&self) -> ::protobuf::View<'_, UserRoleRequest> {
    UserRoleRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UserRoleRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, UserRoleRequest>
  where
      'msg: 'shorter {
    UserRoleRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for UserRoleRequestMut<'msg> {
  type MutProxied = UserRoleRequest;
  fn as_mut(&mut self) -> UserRoleRequestMut<'msg> {
    UserRoleRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UserRoleRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> UserRoleRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl UserRoleRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, UserRoleRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UserRoleRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UserRoleRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // enforcerHandler: optional int32
  pub fn enforcerHandler(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enforcerHandler(&mut self, val: i32) {
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

  // user: optional string
  pub fn user(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_user(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // role: optional string
  pub fn role(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_role(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl UserRoleRequest

impl ::std::ops::Drop for UserRoleRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for UserRoleRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for UserRoleRequest {
  type Proxied = Self;
  fn as_view(&self) -> UserRoleRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for UserRoleRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UserRoleRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for UserRoleRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__casbin__v1__UserRoleRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$(P1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__casbin__v1__UserRoleRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__casbin__v1__UserRoleRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UserRoleRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UserRoleRequest {
  type Msg = UserRoleRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UserRoleRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UserRoleRequest {
  type Msg = UserRoleRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UserRoleRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UserRoleRequestMut<'_> {
  type Msg = UserRoleRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UserRoleRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UserRoleRequestMut<'_> {
  type Msg = UserRoleRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UserRoleRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UserRoleRequestView<'_> {
  type Msg = UserRoleRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UserRoleRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UserRoleRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__casbin__v1__PermissionRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PermissionRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PermissionRequest>
}

impl ::protobuf::Message for PermissionRequest {}

impl ::std::default::Default for PermissionRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PermissionRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PermissionRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `PermissionRequestMut`.
unsafe impl Sync for PermissionRequest {}

// SAFETY:
// - `PermissionRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PermissionRequest {}

impl ::protobuf::Proxied for PermissionRequest {
  type View<'msg> = PermissionRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PermissionRequest {}

impl ::protobuf::MutProxied for PermissionRequest {
  type Mut<'msg> = PermissionRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PermissionRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PermissionRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PermissionRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PermissionRequestView<'msg> {
  type Message = PermissionRequest;
}

impl ::std::fmt::Debug for PermissionRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PermissionRequestView<'_> {
  fn default() -> PermissionRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PermissionRequest>> for PermissionRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PermissionRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PermissionRequestView<'msg> {

  pub fn to_owned(&self) -> PermissionRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // enforcerHandler: optional int32
  pub fn enforcerHandler(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // user: optional string
  pub fn user(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // permissions: repeated string
  pub fn permissions(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `PermissionRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for PermissionRequestView<'_> {}

// SAFETY:
// - `PermissionRequestView` is `Send` because while its alive a `PermissionRequestMut` cannot.
// - `PermissionRequestView` does not use thread-local data.
unsafe impl Send for PermissionRequestView<'_> {}

impl<'msg> ::protobuf::AsView for PermissionRequestView<'msg> {
  type Proxied = PermissionRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, PermissionRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PermissionRequestView<'msg> {
  fn into_view<'shorter>(self) -> PermissionRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PermissionRequest> for PermissionRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PermissionRequest {
    let mut dst = PermissionRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PermissionRequest> for PermissionRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PermissionRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for PermissionRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PermissionRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PermissionRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PermissionRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PermissionRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PermissionRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PermissionRequestMut<'msg> {
  type Message = PermissionRequest;
}

impl ::std::fmt::Debug for PermissionRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PermissionRequest>> for PermissionRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PermissionRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PermissionRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PermissionRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> PermissionRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // enforcerHandler: optional int32
  pub fn enforcerHandler(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enforcerHandler(&mut self, val: i32) {
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

  // user: optional string
  pub fn user(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_user(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // permissions: repeated string
  pub fn permissions(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn permissions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_permissions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}

// SAFETY:
// - `PermissionRequestMut` does not perform any shared mutation.
unsafe impl Send for PermissionRequestMut<'_> {}

// SAFETY:
// - `PermissionRequestMut` does not perform any shared mutation.
unsafe impl Sync for PermissionRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for PermissionRequestMut<'msg> {
  type Proxied = PermissionRequest;
  fn as_view(&self) -> ::protobuf::View<'_, PermissionRequest> {
    PermissionRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PermissionRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PermissionRequest>
  where
      'msg: 'shorter {
    PermissionRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for PermissionRequestMut<'msg> {
  type MutProxied = PermissionRequest;
  fn as_mut(&mut self) -> PermissionRequestMut<'msg> {
    PermissionRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PermissionRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> PermissionRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PermissionRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PermissionRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PermissionRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PermissionRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // enforcerHandler: optional int32
  pub fn enforcerHandler(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enforcerHandler(&mut self, val: i32) {
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

  // user: optional string
  pub fn user(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_user(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // permissions: repeated string
  pub fn permissions(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn permissions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_permissions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}  // impl PermissionRequest

impl ::std::ops::Drop for PermissionRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PermissionRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PermissionRequest {
  type Proxied = Self;
  fn as_view(&self) -> PermissionRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PermissionRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PermissionRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PermissionRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__casbin__v1__PermissionRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$(P1XET");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__casbin__v1__PermissionRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__casbin__v1__PermissionRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PermissionRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PermissionRequest {
  type Msg = PermissionRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PermissionRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PermissionRequest {
  type Msg = PermissionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PermissionRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PermissionRequestMut<'_> {
  type Msg = PermissionRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PermissionRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PermissionRequestMut<'_> {
  type Msg = PermissionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PermissionRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PermissionRequestView<'_> {
  type Msg = PermissionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PermissionRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PermissionRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__casbin__v1__Array2DReply_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Array2DReply {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Array2DReply>
}

impl ::protobuf::Message for Array2DReply {}

impl ::std::default::Default for Array2DReply {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Array2DReply {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Array2DReply` is `Sync` because it does not implement interior mutability.
//    Neither does `Array2DReplyMut`.
unsafe impl Sync for Array2DReply {}

// SAFETY:
// - `Array2DReply` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Array2DReply {}

impl ::protobuf::Proxied for Array2DReply {
  type View<'msg> = Array2DReplyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Array2DReply {}

impl ::protobuf::MutProxied for Array2DReply {
  type Mut<'msg> = Array2DReplyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Array2DReplyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Array2DReply>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Array2DReplyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for Array2DReplyView<'msg> {
  type Message = Array2DReply;
}

impl ::std::fmt::Debug for Array2DReplyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for Array2DReplyView<'_> {
  fn default() -> Array2DReplyView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Array2DReply>> for Array2DReplyView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Array2DReply>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Array2DReplyView<'msg> {

  pub fn to_owned(&self) -> Array2DReply {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // d2: repeated message palm.casbin.v1.Array2DReply.d
  pub fn d2(self) -> ::protobuf::RepeatedView<'msg, super::array2_d_reply::d> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::array2_d_reply::d>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `Array2DReplyView` is `Sync` because it does not support mutation.
unsafe impl Sync for Array2DReplyView<'_> {}

// SAFETY:
// - `Array2DReplyView` is `Send` because while its alive a `Array2DReplyMut` cannot.
// - `Array2DReplyView` does not use thread-local data.
unsafe impl Send for Array2DReplyView<'_> {}

impl<'msg> ::protobuf::AsView for Array2DReplyView<'msg> {
  type Proxied = Array2DReply;
  fn as_view(&self) -> ::protobuf::View<'msg, Array2DReply> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Array2DReplyView<'msg> {
  fn into_view<'shorter>(self) -> Array2DReplyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Array2DReply> for Array2DReplyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Array2DReply {
    let mut dst = Array2DReply::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Array2DReply> for Array2DReplyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Array2DReply {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Array2DReply {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for Array2DReplyView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for Array2DReplyMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct Array2DReplyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Array2DReply>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Array2DReplyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for Array2DReplyMut<'msg> {
  type Message = Array2DReply;
}

impl ::std::fmt::Debug for Array2DReplyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Array2DReply>> for Array2DReplyMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Array2DReply>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Array2DReplyMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Array2DReply> {
    self.inner
  }

  pub fn to_owned(&self) -> Array2DReply {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // d2: repeated message palm.casbin.v1.Array2DReply.d
  pub fn d2(&self) -> ::protobuf::RepeatedView<'_, super::array2_d_reply::d> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::array2_d_reply::d>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn d2_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::array2_d_reply::d> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
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
  pub fn set_d2(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::array2_d_reply::d>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `Array2DReplyMut` does not perform any shared mutation.
unsafe impl Send for Array2DReplyMut<'_> {}

// SAFETY:
// - `Array2DReplyMut` does not perform any shared mutation.
unsafe impl Sync for Array2DReplyMut<'_> {}

impl<'msg> ::protobuf::AsView for Array2DReplyMut<'msg> {
  type Proxied = Array2DReply;
  fn as_view(&self) -> ::protobuf::View<'_, Array2DReply> {
    Array2DReplyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Array2DReplyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Array2DReply>
  where
      'msg: 'shorter {
    Array2DReplyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for Array2DReplyMut<'msg> {
  type MutProxied = Array2DReply;
  fn as_mut(&mut self) -> Array2DReplyMut<'msg> {
    Array2DReplyMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for Array2DReplyMut<'msg> {
  fn into_mut<'shorter>(self) -> Array2DReplyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Array2DReply {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Array2DReply> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> Array2DReplyView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> Array2DReplyMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // d2: repeated message palm.casbin.v1.Array2DReply.d
  pub fn d2(&self) -> ::protobuf::RepeatedView<'_, super::array2_d_reply::d> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::array2_d_reply::d>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn d2_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::array2_d_reply::d> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
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
  pub fn set_d2(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::array2_d_reply::d>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl Array2DReply

impl ::std::ops::Drop for Array2DReply {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Array2DReply {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Array2DReply {
  type Proxied = Self;
  fn as_view(&self) -> Array2DReplyView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Array2DReply {
  type MutProxied = Self;
  fn as_mut(&mut self) -> Array2DReplyMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Array2DReply {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__casbin__v1__Array2DReply_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__casbin__v1__Array2DReply_msg_init.0, &[<super::array2_d_reply::d as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__casbin__v1__Array2DReply_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Array2DReply {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Array2DReply {
  type Msg = Array2DReply;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Array2DReply> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Array2DReply {
  type Msg = Array2DReply;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Array2DReply> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Array2DReplyMut<'_> {
  type Msg = Array2DReply;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Array2DReply> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Array2DReplyMut<'_> {
  type Msg = Array2DReply;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Array2DReply> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Array2DReplyView<'_> {
  type Msg = Array2DReply;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Array2DReply> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Array2DReplyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod array2_d_reply {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__casbin__v1__Array2DReply__d_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct d {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<d>
}

impl ::protobuf::Message for d {}

impl ::std::default::Default for d {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for d {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `d` is `Sync` because it does not implement interior mutability.
//    Neither does `dMut`.
unsafe impl Sync for d {}

// SAFETY:
// - `d` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for d {}

impl ::protobuf::Proxied for d {
  type View<'msg> = dView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for d {}

impl ::protobuf::MutProxied for d {
  type Mut<'msg> = dMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct dView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, d>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for dView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for dView<'msg> {
  type Message = d;
}

impl ::std::fmt::Debug for dView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for dView<'_> {
  fn default() -> dView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, d>> for dView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, d>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> dView<'msg> {

  pub fn to_owned(&self) -> d {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // d1: repeated string
  pub fn d1(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `dView` is `Sync` because it does not support mutation.
unsafe impl Sync for dView<'_> {}

// SAFETY:
// - `dView` is `Send` because while its alive a `dMut` cannot.
// - `dView` does not use thread-local data.
unsafe impl Send for dView<'_> {}

impl<'msg> ::protobuf::AsView for dView<'msg> {
  type Proxied = d;
  fn as_view(&self) -> ::protobuf::View<'msg, d> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for dView<'msg> {
  fn into_view<'shorter>(self) -> dView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<d> for dView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> d {
    let mut dst = d::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<d> for dMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> d {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for d {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for dView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for dMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct dMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, d>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for dMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for dMut<'msg> {
  type Message = d;
}

impl ::std::fmt::Debug for dMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, d>> for dMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, d>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> dMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, d> {
    self.inner
  }

  pub fn to_owned(&self) -> d {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // d1: repeated string
  pub fn d1(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn d1_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
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
  pub fn set_d1(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `dMut` does not perform any shared mutation.
unsafe impl Send for dMut<'_> {}

// SAFETY:
// - `dMut` does not perform any shared mutation.
unsafe impl Sync for dMut<'_> {}

impl<'msg> ::protobuf::AsView for dMut<'msg> {
  type Proxied = d;
  fn as_view(&self) -> ::protobuf::View<'_, d> {
    dView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for dMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, d>
  where
      'msg: 'shorter {
    dView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for dMut<'msg> {
  type MutProxied = d;
  fn as_mut(&mut self) -> dMut<'msg> {
    dMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for dMut<'msg> {
  fn into_mut<'shorter>(self) -> dMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl d {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, d> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> dView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> dMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // d1: repeated string
  pub fn d1(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn d1_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
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
  pub fn set_d1(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl d

impl ::std::ops::Drop for d {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for d {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for d {
  type Proxied = Self;
  fn as_view(&self) -> dView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for d {
  type MutProxied = Self;
  fn as_mut(&mut self) -> dMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for d {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::array2_d_reply::palm__casbin__v1__Array2DReply__d_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$ME");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::array2_d_reply::palm__casbin__v1__Array2DReply__d_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::array2_d_reply::palm__casbin__v1__Array2DReply__d_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for d {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for d {
  type Msg = d;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<d> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for d {
  type Msg = d;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<d> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for dMut<'_> {
  type Msg = d;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<d> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for dMut<'_> {
  type Msg = d;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<d> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for dView<'_> {
  type Msg = d;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<d> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for dMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod array2_d_reply


