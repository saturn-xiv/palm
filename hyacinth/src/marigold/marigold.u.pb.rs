const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.34.0-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__marigold__v1__WechatPayBillPullRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct WechatPayBillPullRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<WechatPayBillPullRequest>
}

impl ::protobuf::Message for WechatPayBillPullRequest {}

impl ::std::default::Default for WechatPayBillPullRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for WechatPayBillPullRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `WechatPayBillPullRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `WechatPayBillPullRequestMut`.
unsafe impl Sync for WechatPayBillPullRequest {}

// SAFETY:
// - `WechatPayBillPullRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for WechatPayBillPullRequest {}

impl ::protobuf::Proxied for WechatPayBillPullRequest {
  type View<'msg> = WechatPayBillPullRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for WechatPayBillPullRequest {}

impl ::protobuf::MutProxied for WechatPayBillPullRequest {
  type Mut<'msg> = WechatPayBillPullRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct WechatPayBillPullRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, WechatPayBillPullRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for WechatPayBillPullRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for WechatPayBillPullRequestView<'msg> {
  type Message = WechatPayBillPullRequest;
}

impl ::std::fmt::Debug for WechatPayBillPullRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for WechatPayBillPullRequestView<'_> {
  fn default() -> WechatPayBillPullRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, WechatPayBillPullRequest>> for WechatPayBillPullRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, WechatPayBillPullRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> WechatPayBillPullRequestView<'msg> {

  pub fn to_owned(&self) -> WechatPayBillPullRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `WechatPayBillPullRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for WechatPayBillPullRequestView<'_> {}

// SAFETY:
// - `WechatPayBillPullRequestView` is `Send` because while its alive a `WechatPayBillPullRequestMut` cannot.
// - `WechatPayBillPullRequestView` does not use thread-local data.
unsafe impl Send for WechatPayBillPullRequestView<'_> {}

impl<'msg> ::protobuf::AsView for WechatPayBillPullRequestView<'msg> {
  type Proxied = WechatPayBillPullRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, WechatPayBillPullRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for WechatPayBillPullRequestView<'msg> {
  fn into_view<'shorter>(self) -> WechatPayBillPullRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<WechatPayBillPullRequest> for WechatPayBillPullRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> WechatPayBillPullRequest {
    let mut dst = WechatPayBillPullRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<WechatPayBillPullRequest> for WechatPayBillPullRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> WechatPayBillPullRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for WechatPayBillPullRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for WechatPayBillPullRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for WechatPayBillPullRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct WechatPayBillPullRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, WechatPayBillPullRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for WechatPayBillPullRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for WechatPayBillPullRequestMut<'msg> {
  type Message = WechatPayBillPullRequest;
}

impl ::std::fmt::Debug for WechatPayBillPullRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, WechatPayBillPullRequest>> for WechatPayBillPullRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, WechatPayBillPullRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> WechatPayBillPullRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, WechatPayBillPullRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> WechatPayBillPullRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `WechatPayBillPullRequestMut` does not perform any shared mutation.
unsafe impl Send for WechatPayBillPullRequestMut<'_> {}

// SAFETY:
// - `WechatPayBillPullRequestMut` does not perform any shared mutation.
unsafe impl Sync for WechatPayBillPullRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for WechatPayBillPullRequestMut<'msg> {
  type Proxied = WechatPayBillPullRequest;
  fn as_view(&self) -> ::protobuf::View<'_, WechatPayBillPullRequest> {
    WechatPayBillPullRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for WechatPayBillPullRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, WechatPayBillPullRequest>
  where
      'msg: 'shorter {
    WechatPayBillPullRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for WechatPayBillPullRequestMut<'msg> {
  type MutProxied = WechatPayBillPullRequest;
  fn as_mut(&mut self) -> WechatPayBillPullRequestMut<'msg> {
    WechatPayBillPullRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for WechatPayBillPullRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> WechatPayBillPullRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl WechatPayBillPullRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, WechatPayBillPullRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> WechatPayBillPullRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> WechatPayBillPullRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl WechatPayBillPullRequest

impl ::std::ops::Drop for WechatPayBillPullRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for WechatPayBillPullRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for WechatPayBillPullRequest {
  type Proxied = Self;
  fn as_view(&self) -> WechatPayBillPullRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for WechatPayBillPullRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> WechatPayBillPullRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for WechatPayBillPullRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__marigold__v1__WechatPayBillPullRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__marigold__v1__WechatPayBillPullRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__marigold__v1__WechatPayBillPullRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for WechatPayBillPullRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for WechatPayBillPullRequest {
  type Msg = WechatPayBillPullRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WechatPayBillPullRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WechatPayBillPullRequest {
  type Msg = WechatPayBillPullRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WechatPayBillPullRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for WechatPayBillPullRequestMut<'_> {
  type Msg = WechatPayBillPullRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WechatPayBillPullRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WechatPayBillPullRequestMut<'_> {
  type Msg = WechatPayBillPullRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WechatPayBillPullRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WechatPayBillPullRequestView<'_> {
  type Msg = WechatPayBillPullRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WechatPayBillPullRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for WechatPayBillPullRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__marigold__v1__WechatPayBillPullResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct WechatPayBillPullResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<WechatPayBillPullResponse>
}

impl ::protobuf::Message for WechatPayBillPullResponse {}

impl ::std::default::Default for WechatPayBillPullResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for WechatPayBillPullResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `WechatPayBillPullResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `WechatPayBillPullResponseMut`.
unsafe impl Sync for WechatPayBillPullResponse {}

// SAFETY:
// - `WechatPayBillPullResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for WechatPayBillPullResponse {}

impl ::protobuf::Proxied for WechatPayBillPullResponse {
  type View<'msg> = WechatPayBillPullResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for WechatPayBillPullResponse {}

impl ::protobuf::MutProxied for WechatPayBillPullResponse {
  type Mut<'msg> = WechatPayBillPullResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct WechatPayBillPullResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, WechatPayBillPullResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for WechatPayBillPullResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for WechatPayBillPullResponseView<'msg> {
  type Message = WechatPayBillPullResponse;
}

impl ::std::fmt::Debug for WechatPayBillPullResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for WechatPayBillPullResponseView<'_> {
  fn default() -> WechatPayBillPullResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, WechatPayBillPullResponse>> for WechatPayBillPullResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, WechatPayBillPullResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> WechatPayBillPullResponseView<'msg> {

  pub fn to_owned(&self) -> WechatPayBillPullResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `WechatPayBillPullResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for WechatPayBillPullResponseView<'_> {}

// SAFETY:
// - `WechatPayBillPullResponseView` is `Send` because while its alive a `WechatPayBillPullResponseMut` cannot.
// - `WechatPayBillPullResponseView` does not use thread-local data.
unsafe impl Send for WechatPayBillPullResponseView<'_> {}

impl<'msg> ::protobuf::AsView for WechatPayBillPullResponseView<'msg> {
  type Proxied = WechatPayBillPullResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, WechatPayBillPullResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for WechatPayBillPullResponseView<'msg> {
  fn into_view<'shorter>(self) -> WechatPayBillPullResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<WechatPayBillPullResponse> for WechatPayBillPullResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> WechatPayBillPullResponse {
    let mut dst = WechatPayBillPullResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<WechatPayBillPullResponse> for WechatPayBillPullResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> WechatPayBillPullResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for WechatPayBillPullResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for WechatPayBillPullResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for WechatPayBillPullResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct WechatPayBillPullResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, WechatPayBillPullResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for WechatPayBillPullResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for WechatPayBillPullResponseMut<'msg> {
  type Message = WechatPayBillPullResponse;
}

impl ::std::fmt::Debug for WechatPayBillPullResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, WechatPayBillPullResponse>> for WechatPayBillPullResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, WechatPayBillPullResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> WechatPayBillPullResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, WechatPayBillPullResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> WechatPayBillPullResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `WechatPayBillPullResponseMut` does not perform any shared mutation.
unsafe impl Send for WechatPayBillPullResponseMut<'_> {}

// SAFETY:
// - `WechatPayBillPullResponseMut` does not perform any shared mutation.
unsafe impl Sync for WechatPayBillPullResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for WechatPayBillPullResponseMut<'msg> {
  type Proxied = WechatPayBillPullResponse;
  fn as_view(&self) -> ::protobuf::View<'_, WechatPayBillPullResponse> {
    WechatPayBillPullResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for WechatPayBillPullResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, WechatPayBillPullResponse>
  where
      'msg: 'shorter {
    WechatPayBillPullResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for WechatPayBillPullResponseMut<'msg> {
  type MutProxied = WechatPayBillPullResponse;
  fn as_mut(&mut self) -> WechatPayBillPullResponseMut<'msg> {
    WechatPayBillPullResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for WechatPayBillPullResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> WechatPayBillPullResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl WechatPayBillPullResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, WechatPayBillPullResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> WechatPayBillPullResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> WechatPayBillPullResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl WechatPayBillPullResponse

impl ::std::ops::Drop for WechatPayBillPullResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for WechatPayBillPullResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for WechatPayBillPullResponse {
  type Proxied = Self;
  fn as_view(&self) -> WechatPayBillPullResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for WechatPayBillPullResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> WechatPayBillPullResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for WechatPayBillPullResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__marigold__v1__WechatPayBillPullResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__marigold__v1__WechatPayBillPullResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__marigold__v1__WechatPayBillPullResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for WechatPayBillPullResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for WechatPayBillPullResponse {
  type Msg = WechatPayBillPullResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WechatPayBillPullResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WechatPayBillPullResponse {
  type Msg = WechatPayBillPullResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WechatPayBillPullResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for WechatPayBillPullResponseMut<'_> {
  type Msg = WechatPayBillPullResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WechatPayBillPullResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WechatPayBillPullResponseMut<'_> {
  type Msg = WechatPayBillPullResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WechatPayBillPullResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WechatPayBillPullResponseView<'_> {
  type Msg = WechatPayBillPullResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WechatPayBillPullResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for WechatPayBillPullResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



