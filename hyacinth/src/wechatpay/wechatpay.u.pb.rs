const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.34.0-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__wechatpay__v1__BillPullRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BillPullRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BillPullRequest>
}

impl ::protobuf::Message for BillPullRequest {}

impl ::std::default::Default for BillPullRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BillPullRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BillPullRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `BillPullRequestMut`.
unsafe impl Sync for BillPullRequest {}

// SAFETY:
// - `BillPullRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for BillPullRequest {}

impl ::protobuf::Proxied for BillPullRequest {
  type View<'msg> = BillPullRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BillPullRequest {}

impl ::protobuf::MutProxied for BillPullRequest {
  type Mut<'msg> = BillPullRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BillPullRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BillPullRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BillPullRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BillPullRequestView<'msg> {
  type Message = BillPullRequest;
}

impl ::std::fmt::Debug for BillPullRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BillPullRequestView<'_> {
  fn default() -> BillPullRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BillPullRequest>> for BillPullRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BillPullRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BillPullRequestView<'msg> {

  pub fn to_owned(&self) -> BillPullRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `BillPullRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for BillPullRequestView<'_> {}

// SAFETY:
// - `BillPullRequestView` is `Send` because while its alive a `BillPullRequestMut` cannot.
// - `BillPullRequestView` does not use thread-local data.
unsafe impl Send for BillPullRequestView<'_> {}

impl<'msg> ::protobuf::AsView for BillPullRequestView<'msg> {
  type Proxied = BillPullRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, BillPullRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BillPullRequestView<'msg> {
  fn into_view<'shorter>(self) -> BillPullRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BillPullRequest> for BillPullRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BillPullRequest {
    let mut dst = BillPullRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BillPullRequest> for BillPullRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BillPullRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for BillPullRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for BillPullRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for BillPullRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BillPullRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BillPullRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BillPullRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BillPullRequestMut<'msg> {
  type Message = BillPullRequest;
}

impl ::std::fmt::Debug for BillPullRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BillPullRequest>> for BillPullRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BillPullRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BillPullRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BillPullRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> BillPullRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `BillPullRequestMut` does not perform any shared mutation.
unsafe impl Send for BillPullRequestMut<'_> {}

// SAFETY:
// - `BillPullRequestMut` does not perform any shared mutation.
unsafe impl Sync for BillPullRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for BillPullRequestMut<'msg> {
  type Proxied = BillPullRequest;
  fn as_view(&self) -> ::protobuf::View<'_, BillPullRequest> {
    BillPullRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BillPullRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BillPullRequest>
  where
      'msg: 'shorter {
    BillPullRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for BillPullRequestMut<'msg> {
  type MutProxied = BillPullRequest;
  fn as_mut(&mut self) -> BillPullRequestMut<'msg> {
    BillPullRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BillPullRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> BillPullRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BillPullRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BillPullRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BillPullRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BillPullRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl BillPullRequest

impl ::std::ops::Drop for BillPullRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BillPullRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BillPullRequest {
  type Proxied = Self;
  fn as_view(&self) -> BillPullRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BillPullRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BillPullRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BillPullRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__wechatpay__v1__BillPullRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__wechatpay__v1__BillPullRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__wechatpay__v1__BillPullRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BillPullRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BillPullRequest {
  type Msg = BillPullRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BillPullRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BillPullRequest {
  type Msg = BillPullRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BillPullRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BillPullRequestMut<'_> {
  type Msg = BillPullRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BillPullRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BillPullRequestMut<'_> {
  type Msg = BillPullRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BillPullRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BillPullRequestView<'_> {
  type Msg = BillPullRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BillPullRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BillPullRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__wechatpay__v1__BillPullResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BillPullResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BillPullResponse>
}

impl ::protobuf::Message for BillPullResponse {}

impl ::std::default::Default for BillPullResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BillPullResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BillPullResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `BillPullResponseMut`.
unsafe impl Sync for BillPullResponse {}

// SAFETY:
// - `BillPullResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for BillPullResponse {}

impl ::protobuf::Proxied for BillPullResponse {
  type View<'msg> = BillPullResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BillPullResponse {}

impl ::protobuf::MutProxied for BillPullResponse {
  type Mut<'msg> = BillPullResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BillPullResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BillPullResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BillPullResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BillPullResponseView<'msg> {
  type Message = BillPullResponse;
}

impl ::std::fmt::Debug for BillPullResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BillPullResponseView<'_> {
  fn default() -> BillPullResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BillPullResponse>> for BillPullResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BillPullResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BillPullResponseView<'msg> {

  pub fn to_owned(&self) -> BillPullResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `BillPullResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for BillPullResponseView<'_> {}

// SAFETY:
// - `BillPullResponseView` is `Send` because while its alive a `BillPullResponseMut` cannot.
// - `BillPullResponseView` does not use thread-local data.
unsafe impl Send for BillPullResponseView<'_> {}

impl<'msg> ::protobuf::AsView for BillPullResponseView<'msg> {
  type Proxied = BillPullResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, BillPullResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BillPullResponseView<'msg> {
  fn into_view<'shorter>(self) -> BillPullResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BillPullResponse> for BillPullResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BillPullResponse {
    let mut dst = BillPullResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BillPullResponse> for BillPullResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BillPullResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for BillPullResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for BillPullResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for BillPullResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BillPullResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BillPullResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BillPullResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BillPullResponseMut<'msg> {
  type Message = BillPullResponse;
}

impl ::std::fmt::Debug for BillPullResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BillPullResponse>> for BillPullResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BillPullResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BillPullResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BillPullResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> BillPullResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `BillPullResponseMut` does not perform any shared mutation.
unsafe impl Send for BillPullResponseMut<'_> {}

// SAFETY:
// - `BillPullResponseMut` does not perform any shared mutation.
unsafe impl Sync for BillPullResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for BillPullResponseMut<'msg> {
  type Proxied = BillPullResponse;
  fn as_view(&self) -> ::protobuf::View<'_, BillPullResponse> {
    BillPullResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BillPullResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BillPullResponse>
  where
      'msg: 'shorter {
    BillPullResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for BillPullResponseMut<'msg> {
  type MutProxied = BillPullResponse;
  fn as_mut(&mut self) -> BillPullResponseMut<'msg> {
    BillPullResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BillPullResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> BillPullResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BillPullResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BillPullResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BillPullResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BillPullResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl BillPullResponse

impl ::std::ops::Drop for BillPullResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BillPullResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BillPullResponse {
  type Proxied = Self;
  fn as_view(&self) -> BillPullResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BillPullResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BillPullResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BillPullResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__wechatpay__v1__BillPullResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__wechatpay__v1__BillPullResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__wechatpay__v1__BillPullResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BillPullResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BillPullResponse {
  type Msg = BillPullResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BillPullResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BillPullResponse {
  type Msg = BillPullResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BillPullResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BillPullResponseMut<'_> {
  type Msg = BillPullResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BillPullResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BillPullResponseMut<'_> {
  type Msg = BillPullResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BillPullResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BillPullResponseView<'_> {
  type Msg = BillPullResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BillPullResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BillPullResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



