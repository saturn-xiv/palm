const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.34.0-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__oauth2__v1__GoogleSignInUrlRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GoogleSignInUrlRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GoogleSignInUrlRequest>
}

impl ::protobuf::Message for GoogleSignInUrlRequest {}

impl ::std::default::Default for GoogleSignInUrlRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GoogleSignInUrlRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GoogleSignInUrlRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GoogleSignInUrlRequestMut`.
unsafe impl Sync for GoogleSignInUrlRequest {}

// SAFETY:
// - `GoogleSignInUrlRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GoogleSignInUrlRequest {}

impl ::protobuf::Proxied for GoogleSignInUrlRequest {
  type View<'msg> = GoogleSignInUrlRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GoogleSignInUrlRequest {}

impl ::protobuf::MutProxied for GoogleSignInUrlRequest {
  type Mut<'msg> = GoogleSignInUrlRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GoogleSignInUrlRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleSignInUrlRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GoogleSignInUrlRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GoogleSignInUrlRequestView<'msg> {
  type Message = GoogleSignInUrlRequest;
}

impl ::std::fmt::Debug for GoogleSignInUrlRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GoogleSignInUrlRequestView<'_> {
  fn default() -> GoogleSignInUrlRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleSignInUrlRequest>> for GoogleSignInUrlRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleSignInUrlRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GoogleSignInUrlRequestView<'msg> {

  pub fn to_owned(&self) -> GoogleSignInUrlRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // redirect_uri: optional string
  pub fn redirect_uri(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // scopes: repeated string
  pub fn scopes(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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
// - `GoogleSignInUrlRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GoogleSignInUrlRequestView<'_> {}

// SAFETY:
// - `GoogleSignInUrlRequestView` is `Send` because while its alive a `GoogleSignInUrlRequestMut` cannot.
// - `GoogleSignInUrlRequestView` does not use thread-local data.
unsafe impl Send for GoogleSignInUrlRequestView<'_> {}

impl<'msg> ::protobuf::AsView for GoogleSignInUrlRequestView<'msg> {
  type Proxied = GoogleSignInUrlRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, GoogleSignInUrlRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GoogleSignInUrlRequestView<'msg> {
  fn into_view<'shorter>(self) -> GoogleSignInUrlRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GoogleSignInUrlRequest> for GoogleSignInUrlRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GoogleSignInUrlRequest {
    let mut dst = GoogleSignInUrlRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GoogleSignInUrlRequest> for GoogleSignInUrlRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GoogleSignInUrlRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GoogleSignInUrlRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GoogleSignInUrlRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GoogleSignInUrlRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GoogleSignInUrlRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleSignInUrlRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GoogleSignInUrlRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GoogleSignInUrlRequestMut<'msg> {
  type Message = GoogleSignInUrlRequest;
}

impl ::std::fmt::Debug for GoogleSignInUrlRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleSignInUrlRequest>> for GoogleSignInUrlRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleSignInUrlRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GoogleSignInUrlRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleSignInUrlRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> GoogleSignInUrlRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // redirect_uri: optional string
  pub fn redirect_uri(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_redirect_uri(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // scopes: repeated string
  pub fn scopes(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn scopes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_scopes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `GoogleSignInUrlRequestMut` does not perform any shared mutation.
unsafe impl Send for GoogleSignInUrlRequestMut<'_> {}

// SAFETY:
// - `GoogleSignInUrlRequestMut` does not perform any shared mutation.
unsafe impl Sync for GoogleSignInUrlRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for GoogleSignInUrlRequestMut<'msg> {
  type Proxied = GoogleSignInUrlRequest;
  fn as_view(&self) -> ::protobuf::View<'_, GoogleSignInUrlRequest> {
    GoogleSignInUrlRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GoogleSignInUrlRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GoogleSignInUrlRequest>
  where
      'msg: 'shorter {
    GoogleSignInUrlRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GoogleSignInUrlRequestMut<'msg> {
  type MutProxied = GoogleSignInUrlRequest;
  fn as_mut(&mut self) -> GoogleSignInUrlRequestMut<'msg> {
    GoogleSignInUrlRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GoogleSignInUrlRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> GoogleSignInUrlRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GoogleSignInUrlRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GoogleSignInUrlRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GoogleSignInUrlRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GoogleSignInUrlRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // redirect_uri: optional string
  pub fn redirect_uri(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_redirect_uri(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // scopes: repeated string
  pub fn scopes(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn scopes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_scopes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl GoogleSignInUrlRequest

impl ::std::ops::Drop for GoogleSignInUrlRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GoogleSignInUrlRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GoogleSignInUrlRequest {
  type Proxied = Self;
  fn as_view(&self) -> GoogleSignInUrlRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GoogleSignInUrlRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GoogleSignInUrlRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GoogleSignInUrlRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__oauth2__v1__GoogleSignInUrlRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1PE");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__oauth2__v1__GoogleSignInUrlRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__oauth2__v1__GoogleSignInUrlRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GoogleSignInUrlRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GoogleSignInUrlRequest {
  type Msg = GoogleSignInUrlRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleSignInUrlRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleSignInUrlRequest {
  type Msg = GoogleSignInUrlRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleSignInUrlRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GoogleSignInUrlRequestMut<'_> {
  type Msg = GoogleSignInUrlRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleSignInUrlRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleSignInUrlRequestMut<'_> {
  type Msg = GoogleSignInUrlRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleSignInUrlRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleSignInUrlRequestView<'_> {
  type Msg = GoogleSignInUrlRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleSignInUrlRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GoogleSignInUrlRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__oauth2__v1__GoogleSignInUrlResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GoogleSignInUrlResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GoogleSignInUrlResponse>
}

impl ::protobuf::Message for GoogleSignInUrlResponse {}

impl ::std::default::Default for GoogleSignInUrlResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GoogleSignInUrlResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GoogleSignInUrlResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GoogleSignInUrlResponseMut`.
unsafe impl Sync for GoogleSignInUrlResponse {}

// SAFETY:
// - `GoogleSignInUrlResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GoogleSignInUrlResponse {}

impl ::protobuf::Proxied for GoogleSignInUrlResponse {
  type View<'msg> = GoogleSignInUrlResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GoogleSignInUrlResponse {}

impl ::protobuf::MutProxied for GoogleSignInUrlResponse {
  type Mut<'msg> = GoogleSignInUrlResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GoogleSignInUrlResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleSignInUrlResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GoogleSignInUrlResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GoogleSignInUrlResponseView<'msg> {
  type Message = GoogleSignInUrlResponse;
}

impl ::std::fmt::Debug for GoogleSignInUrlResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GoogleSignInUrlResponseView<'_> {
  fn default() -> GoogleSignInUrlResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleSignInUrlResponse>> for GoogleSignInUrlResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleSignInUrlResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GoogleSignInUrlResponseView<'msg> {

  pub fn to_owned(&self) -> GoogleSignInUrlResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // authorization_url: optional string
  pub fn authorization_url(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // state: optional string
  pub fn state(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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
// - `GoogleSignInUrlResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GoogleSignInUrlResponseView<'_> {}

// SAFETY:
// - `GoogleSignInUrlResponseView` is `Send` because while its alive a `GoogleSignInUrlResponseMut` cannot.
// - `GoogleSignInUrlResponseView` does not use thread-local data.
unsafe impl Send for GoogleSignInUrlResponseView<'_> {}

impl<'msg> ::protobuf::AsView for GoogleSignInUrlResponseView<'msg> {
  type Proxied = GoogleSignInUrlResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, GoogleSignInUrlResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GoogleSignInUrlResponseView<'msg> {
  fn into_view<'shorter>(self) -> GoogleSignInUrlResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GoogleSignInUrlResponse> for GoogleSignInUrlResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GoogleSignInUrlResponse {
    let mut dst = GoogleSignInUrlResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GoogleSignInUrlResponse> for GoogleSignInUrlResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GoogleSignInUrlResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GoogleSignInUrlResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GoogleSignInUrlResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GoogleSignInUrlResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GoogleSignInUrlResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleSignInUrlResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GoogleSignInUrlResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GoogleSignInUrlResponseMut<'msg> {
  type Message = GoogleSignInUrlResponse;
}

impl ::std::fmt::Debug for GoogleSignInUrlResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleSignInUrlResponse>> for GoogleSignInUrlResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleSignInUrlResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GoogleSignInUrlResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleSignInUrlResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> GoogleSignInUrlResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // authorization_url: optional string
  pub fn authorization_url(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_authorization_url(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // state: optional string
  pub fn state(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_state(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `GoogleSignInUrlResponseMut` does not perform any shared mutation.
unsafe impl Send for GoogleSignInUrlResponseMut<'_> {}

// SAFETY:
// - `GoogleSignInUrlResponseMut` does not perform any shared mutation.
unsafe impl Sync for GoogleSignInUrlResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for GoogleSignInUrlResponseMut<'msg> {
  type Proxied = GoogleSignInUrlResponse;
  fn as_view(&self) -> ::protobuf::View<'_, GoogleSignInUrlResponse> {
    GoogleSignInUrlResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GoogleSignInUrlResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GoogleSignInUrlResponse>
  where
      'msg: 'shorter {
    GoogleSignInUrlResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GoogleSignInUrlResponseMut<'msg> {
  type MutProxied = GoogleSignInUrlResponse;
  fn as_mut(&mut self) -> GoogleSignInUrlResponseMut<'msg> {
    GoogleSignInUrlResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GoogleSignInUrlResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> GoogleSignInUrlResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GoogleSignInUrlResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GoogleSignInUrlResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GoogleSignInUrlResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GoogleSignInUrlResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // authorization_url: optional string
  pub fn authorization_url(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_authorization_url(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // state: optional string
  pub fn state(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_state(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl GoogleSignInUrlResponse

impl ::std::ops::Drop for GoogleSignInUrlResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GoogleSignInUrlResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GoogleSignInUrlResponse {
  type Proxied = Self;
  fn as_view(&self) -> GoogleSignInUrlResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GoogleSignInUrlResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GoogleSignInUrlResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GoogleSignInUrlResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__oauth2__v1__GoogleSignInUrlResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__oauth2__v1__GoogleSignInUrlResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__oauth2__v1__GoogleSignInUrlResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GoogleSignInUrlResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GoogleSignInUrlResponse {
  type Msg = GoogleSignInUrlResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleSignInUrlResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleSignInUrlResponse {
  type Msg = GoogleSignInUrlResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleSignInUrlResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GoogleSignInUrlResponseMut<'_> {
  type Msg = GoogleSignInUrlResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleSignInUrlResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleSignInUrlResponseMut<'_> {
  type Msg = GoogleSignInUrlResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleSignInUrlResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleSignInUrlResponseView<'_> {
  type Msg = GoogleSignInUrlResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleSignInUrlResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GoogleSignInUrlResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__oauth2__v1__GoogleFetchTokenRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GoogleFetchTokenRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GoogleFetchTokenRequest>
}

impl ::protobuf::Message for GoogleFetchTokenRequest {}

impl ::std::default::Default for GoogleFetchTokenRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GoogleFetchTokenRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GoogleFetchTokenRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GoogleFetchTokenRequestMut`.
unsafe impl Sync for GoogleFetchTokenRequest {}

// SAFETY:
// - `GoogleFetchTokenRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GoogleFetchTokenRequest {}

impl ::protobuf::Proxied for GoogleFetchTokenRequest {
  type View<'msg> = GoogleFetchTokenRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GoogleFetchTokenRequest {}

impl ::protobuf::MutProxied for GoogleFetchTokenRequest {
  type Mut<'msg> = GoogleFetchTokenRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GoogleFetchTokenRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleFetchTokenRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GoogleFetchTokenRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GoogleFetchTokenRequestView<'msg> {
  type Message = GoogleFetchTokenRequest;
}

impl ::std::fmt::Debug for GoogleFetchTokenRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GoogleFetchTokenRequestView<'_> {
  fn default() -> GoogleFetchTokenRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleFetchTokenRequest>> for GoogleFetchTokenRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleFetchTokenRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GoogleFetchTokenRequestView<'msg> {

  pub fn to_owned(&self) -> GoogleFetchTokenRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // authorization_response: optional string
  pub fn authorization_response(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // state: optional string
  pub fn state(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // scopes: repeated string
  pub fn scopes(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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
// - `GoogleFetchTokenRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GoogleFetchTokenRequestView<'_> {}

// SAFETY:
// - `GoogleFetchTokenRequestView` is `Send` because while its alive a `GoogleFetchTokenRequestMut` cannot.
// - `GoogleFetchTokenRequestView` does not use thread-local data.
unsafe impl Send for GoogleFetchTokenRequestView<'_> {}

impl<'msg> ::protobuf::AsView for GoogleFetchTokenRequestView<'msg> {
  type Proxied = GoogleFetchTokenRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, GoogleFetchTokenRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GoogleFetchTokenRequestView<'msg> {
  fn into_view<'shorter>(self) -> GoogleFetchTokenRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GoogleFetchTokenRequest> for GoogleFetchTokenRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GoogleFetchTokenRequest {
    let mut dst = GoogleFetchTokenRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GoogleFetchTokenRequest> for GoogleFetchTokenRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GoogleFetchTokenRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GoogleFetchTokenRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GoogleFetchTokenRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GoogleFetchTokenRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GoogleFetchTokenRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleFetchTokenRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GoogleFetchTokenRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GoogleFetchTokenRequestMut<'msg> {
  type Message = GoogleFetchTokenRequest;
}

impl ::std::fmt::Debug for GoogleFetchTokenRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleFetchTokenRequest>> for GoogleFetchTokenRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleFetchTokenRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GoogleFetchTokenRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleFetchTokenRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> GoogleFetchTokenRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // authorization_response: optional string
  pub fn authorization_response(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_authorization_response(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // state: optional string
  pub fn state(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_state(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // scopes: repeated string
  pub fn scopes(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn scopes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_scopes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}

// SAFETY:
// - `GoogleFetchTokenRequestMut` does not perform any shared mutation.
unsafe impl Send for GoogleFetchTokenRequestMut<'_> {}

// SAFETY:
// - `GoogleFetchTokenRequestMut` does not perform any shared mutation.
unsafe impl Sync for GoogleFetchTokenRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for GoogleFetchTokenRequestMut<'msg> {
  type Proxied = GoogleFetchTokenRequest;
  fn as_view(&self) -> ::protobuf::View<'_, GoogleFetchTokenRequest> {
    GoogleFetchTokenRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GoogleFetchTokenRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GoogleFetchTokenRequest>
  where
      'msg: 'shorter {
    GoogleFetchTokenRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GoogleFetchTokenRequestMut<'msg> {
  type MutProxied = GoogleFetchTokenRequest;
  fn as_mut(&mut self) -> GoogleFetchTokenRequestMut<'msg> {
    GoogleFetchTokenRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GoogleFetchTokenRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> GoogleFetchTokenRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GoogleFetchTokenRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GoogleFetchTokenRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GoogleFetchTokenRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GoogleFetchTokenRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // authorization_response: optional string
  pub fn authorization_response(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_authorization_response(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // state: optional string
  pub fn state(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_state(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // scopes: repeated string
  pub fn scopes(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn scopes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_scopes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}  // impl GoogleFetchTokenRequest

impl ::std::ops::Drop for GoogleFetchTokenRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GoogleFetchTokenRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GoogleFetchTokenRequest {
  type Proxied = Self;
  fn as_view(&self) -> GoogleFetchTokenRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GoogleFetchTokenRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GoogleFetchTokenRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GoogleFetchTokenRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__oauth2__v1__GoogleFetchTokenRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1PE");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__oauth2__v1__GoogleFetchTokenRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__oauth2__v1__GoogleFetchTokenRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GoogleFetchTokenRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GoogleFetchTokenRequest {
  type Msg = GoogleFetchTokenRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleFetchTokenRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleFetchTokenRequest {
  type Msg = GoogleFetchTokenRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleFetchTokenRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GoogleFetchTokenRequestMut<'_> {
  type Msg = GoogleFetchTokenRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleFetchTokenRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleFetchTokenRequestMut<'_> {
  type Msg = GoogleFetchTokenRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleFetchTokenRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleFetchTokenRequestView<'_> {
  type Msg = GoogleFetchTokenRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleFetchTokenRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GoogleFetchTokenRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__oauth2__v1__GoogleFetchTokenResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GoogleFetchTokenResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GoogleFetchTokenResponse>
}

impl ::protobuf::Message for GoogleFetchTokenResponse {}

impl ::std::default::Default for GoogleFetchTokenResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GoogleFetchTokenResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GoogleFetchTokenResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GoogleFetchTokenResponseMut`.
unsafe impl Sync for GoogleFetchTokenResponse {}

// SAFETY:
// - `GoogleFetchTokenResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GoogleFetchTokenResponse {}

impl ::protobuf::Proxied for GoogleFetchTokenResponse {
  type View<'msg> = GoogleFetchTokenResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GoogleFetchTokenResponse {}

impl ::protobuf::MutProxied for GoogleFetchTokenResponse {
  type Mut<'msg> = GoogleFetchTokenResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GoogleFetchTokenResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleFetchTokenResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GoogleFetchTokenResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GoogleFetchTokenResponseView<'msg> {
  type Message = GoogleFetchTokenResponse;
}

impl ::std::fmt::Debug for GoogleFetchTokenResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GoogleFetchTokenResponseView<'_> {
  fn default() -> GoogleFetchTokenResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleFetchTokenResponse>> for GoogleFetchTokenResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleFetchTokenResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GoogleFetchTokenResponseView<'msg> {

  pub fn to_owned(&self) -> GoogleFetchTokenResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // credentials: optional message palm.oauth2.v1.GoogleFetchTokenResponse.Credentials
  pub fn has_credentials(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn credentials_opt(self) -> ::protobuf::Optional<super::google_fetch_token_response::CredentialsView<'msg>> {
        ::protobuf::Optional::new(self.credentials(), self.has_credentials())
  }
  pub fn credentials(self) -> super::google_fetch_token_response::CredentialsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::google_fetch_token_response::CredentialsView::default())
  }

}

// SAFETY:
// - `GoogleFetchTokenResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GoogleFetchTokenResponseView<'_> {}

// SAFETY:
// - `GoogleFetchTokenResponseView` is `Send` because while its alive a `GoogleFetchTokenResponseMut` cannot.
// - `GoogleFetchTokenResponseView` does not use thread-local data.
unsafe impl Send for GoogleFetchTokenResponseView<'_> {}

impl<'msg> ::protobuf::AsView for GoogleFetchTokenResponseView<'msg> {
  type Proxied = GoogleFetchTokenResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, GoogleFetchTokenResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GoogleFetchTokenResponseView<'msg> {
  fn into_view<'shorter>(self) -> GoogleFetchTokenResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GoogleFetchTokenResponse> for GoogleFetchTokenResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GoogleFetchTokenResponse {
    let mut dst = GoogleFetchTokenResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GoogleFetchTokenResponse> for GoogleFetchTokenResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GoogleFetchTokenResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GoogleFetchTokenResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GoogleFetchTokenResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GoogleFetchTokenResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GoogleFetchTokenResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleFetchTokenResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GoogleFetchTokenResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GoogleFetchTokenResponseMut<'msg> {
  type Message = GoogleFetchTokenResponse;
}

impl ::std::fmt::Debug for GoogleFetchTokenResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleFetchTokenResponse>> for GoogleFetchTokenResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleFetchTokenResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GoogleFetchTokenResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleFetchTokenResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> GoogleFetchTokenResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // credentials: optional message palm.oauth2.v1.GoogleFetchTokenResponse.Credentials
  pub fn has_credentials(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_credentials(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn credentials_opt(&self) -> ::protobuf::Optional<super::google_fetch_token_response::CredentialsView<'_>> {
        ::protobuf::Optional::new(self.credentials(), self.has_credentials())
  }
  pub fn credentials(&self) -> super::google_fetch_token_response::CredentialsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::google_fetch_token_response::CredentialsView::default())
  }
  pub fn credentials_mut(&mut self) -> super::google_fetch_token_response::CredentialsMut<'_> {
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
  pub fn set_credentials(&mut self,
    val: impl ::protobuf::IntoProxied<super::google_fetch_token_response::Credentials>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}

// SAFETY:
// - `GoogleFetchTokenResponseMut` does not perform any shared mutation.
unsafe impl Send for GoogleFetchTokenResponseMut<'_> {}

// SAFETY:
// - `GoogleFetchTokenResponseMut` does not perform any shared mutation.
unsafe impl Sync for GoogleFetchTokenResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for GoogleFetchTokenResponseMut<'msg> {
  type Proxied = GoogleFetchTokenResponse;
  fn as_view(&self) -> ::protobuf::View<'_, GoogleFetchTokenResponse> {
    GoogleFetchTokenResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GoogleFetchTokenResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GoogleFetchTokenResponse>
  where
      'msg: 'shorter {
    GoogleFetchTokenResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GoogleFetchTokenResponseMut<'msg> {
  type MutProxied = GoogleFetchTokenResponse;
  fn as_mut(&mut self) -> GoogleFetchTokenResponseMut<'msg> {
    GoogleFetchTokenResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GoogleFetchTokenResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> GoogleFetchTokenResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GoogleFetchTokenResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GoogleFetchTokenResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GoogleFetchTokenResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GoogleFetchTokenResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // credentials: optional message palm.oauth2.v1.GoogleFetchTokenResponse.Credentials
  pub fn has_credentials(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_credentials(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn credentials_opt(&self) -> ::protobuf::Optional<super::google_fetch_token_response::CredentialsView<'_>> {
        ::protobuf::Optional::new(self.credentials(), self.has_credentials())
  }
  pub fn credentials(&self) -> super::google_fetch_token_response::CredentialsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::google_fetch_token_response::CredentialsView::default())
  }
  pub fn credentials_mut(&mut self) -> super::google_fetch_token_response::CredentialsMut<'_> {
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
  pub fn set_credentials(&mut self,
    val: impl ::protobuf::IntoProxied<super::google_fetch_token_response::Credentials>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl GoogleFetchTokenResponse

impl ::std::ops::Drop for GoogleFetchTokenResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GoogleFetchTokenResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GoogleFetchTokenResponse {
  type Proxied = Self;
  fn as_view(&self) -> GoogleFetchTokenResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GoogleFetchTokenResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GoogleFetchTokenResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GoogleFetchTokenResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__oauth2__v1__GoogleFetchTokenResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__oauth2__v1__GoogleFetchTokenResponse_msg_init.0, &[<super::google_fetch_token_response::Credentials as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__oauth2__v1__GoogleFetchTokenResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GoogleFetchTokenResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GoogleFetchTokenResponse {
  type Msg = GoogleFetchTokenResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleFetchTokenResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleFetchTokenResponse {
  type Msg = GoogleFetchTokenResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleFetchTokenResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GoogleFetchTokenResponseMut<'_> {
  type Msg = GoogleFetchTokenResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleFetchTokenResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleFetchTokenResponseMut<'_> {
  type Msg = GoogleFetchTokenResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleFetchTokenResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleFetchTokenResponseView<'_> {
  type Msg = GoogleFetchTokenResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleFetchTokenResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GoogleFetchTokenResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod google_fetch_token_response {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__oauth2__v1__GoogleFetchTokenResponse__Credentials_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Credentials {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Credentials>
}

impl ::protobuf::Message for Credentials {}

impl ::std::default::Default for Credentials {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Credentials {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Credentials` is `Sync` because it does not implement interior mutability.
//    Neither does `CredentialsMut`.
unsafe impl Sync for Credentials {}

// SAFETY:
// - `Credentials` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Credentials {}

impl ::protobuf::Proxied for Credentials {
  type View<'msg> = CredentialsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Credentials {}

impl ::protobuf::MutProxied for Credentials {
  type Mut<'msg> = CredentialsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CredentialsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Credentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CredentialsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CredentialsView<'msg> {
  type Message = Credentials;
}

impl ::std::fmt::Debug for CredentialsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CredentialsView<'_> {
  fn default() -> CredentialsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Credentials>> for CredentialsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Credentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CredentialsView<'msg> {

  pub fn to_owned(&self) -> Credentials {
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

  // refresh_token: optional string
  pub fn refresh_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // granted_scopes: repeated string
  pub fn granted_scopes(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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
// - `CredentialsView` is `Sync` because it does not support mutation.
unsafe impl Sync for CredentialsView<'_> {}

// SAFETY:
// - `CredentialsView` is `Send` because while its alive a `CredentialsMut` cannot.
// - `CredentialsView` does not use thread-local data.
unsafe impl Send for CredentialsView<'_> {}

impl<'msg> ::protobuf::AsView for CredentialsView<'msg> {
  type Proxied = Credentials;
  fn as_view(&self) -> ::protobuf::View<'msg, Credentials> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CredentialsView<'msg> {
  fn into_view<'shorter>(self) -> CredentialsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Credentials> for CredentialsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Credentials {
    let mut dst = Credentials::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Credentials> for CredentialsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Credentials {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Credentials {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for CredentialsView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for CredentialsMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CredentialsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Credentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CredentialsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CredentialsMut<'msg> {
  type Message = Credentials;
}

impl ::std::fmt::Debug for CredentialsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Credentials>> for CredentialsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Credentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CredentialsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Credentials> {
    self.inner
  }

  pub fn to_owned(&self) -> Credentials {
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

  // refresh_token: optional string
  pub fn refresh_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_refresh_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // granted_scopes: repeated string
  pub fn granted_scopes(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn granted_scopes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_granted_scopes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}

// SAFETY:
// - `CredentialsMut` does not perform any shared mutation.
unsafe impl Send for CredentialsMut<'_> {}

// SAFETY:
// - `CredentialsMut` does not perform any shared mutation.
unsafe impl Sync for CredentialsMut<'_> {}

impl<'msg> ::protobuf::AsView for CredentialsMut<'msg> {
  type Proxied = Credentials;
  fn as_view(&self) -> ::protobuf::View<'_, Credentials> {
    CredentialsView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CredentialsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Credentials>
  where
      'msg: 'shorter {
    CredentialsView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for CredentialsMut<'msg> {
  type MutProxied = Credentials;
  fn as_mut(&mut self) -> CredentialsMut<'msg> {
    CredentialsMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CredentialsMut<'msg> {
  fn into_mut<'shorter>(self) -> CredentialsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Credentials {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Credentials> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CredentialsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CredentialsMut<'_> {
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

  // refresh_token: optional string
  pub fn refresh_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_refresh_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // granted_scopes: repeated string
  pub fn granted_scopes(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn granted_scopes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_granted_scopes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}  // impl Credentials

impl ::std::ops::Drop for Credentials {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Credentials {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Credentials {
  type Proxied = Self;
  fn as_view(&self) -> CredentialsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Credentials {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CredentialsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Credentials {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::google_fetch_token_response::palm__oauth2__v1__GoogleFetchTokenResponse__Credentials_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1PE");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::google_fetch_token_response::palm__oauth2__v1__GoogleFetchTokenResponse__Credentials_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::google_fetch_token_response::palm__oauth2__v1__GoogleFetchTokenResponse__Credentials_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Credentials {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Credentials {
  type Msg = Credentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Credentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Credentials {
  type Msg = Credentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Credentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CredentialsMut<'_> {
  type Msg = Credentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Credentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CredentialsMut<'_> {
  type Msg = Credentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Credentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CredentialsView<'_> {
  type Msg = Credentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Credentials> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CredentialsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod google_fetch_token_response


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__oauth2__v1__GoogleRevokeResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GoogleRevokeResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GoogleRevokeResponse>
}

impl ::protobuf::Message for GoogleRevokeResponse {}

impl ::std::default::Default for GoogleRevokeResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GoogleRevokeResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GoogleRevokeResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GoogleRevokeResponseMut`.
unsafe impl Sync for GoogleRevokeResponse {}

// SAFETY:
// - `GoogleRevokeResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GoogleRevokeResponse {}

impl ::protobuf::Proxied for GoogleRevokeResponse {
  type View<'msg> = GoogleRevokeResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GoogleRevokeResponse {}

impl ::protobuf::MutProxied for GoogleRevokeResponse {
  type Mut<'msg> = GoogleRevokeResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GoogleRevokeResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleRevokeResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GoogleRevokeResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GoogleRevokeResponseView<'msg> {
  type Message = GoogleRevokeResponse;
}

impl ::std::fmt::Debug for GoogleRevokeResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GoogleRevokeResponseView<'_> {
  fn default() -> GoogleRevokeResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleRevokeResponse>> for GoogleRevokeResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleRevokeResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GoogleRevokeResponseView<'msg> {

  pub fn to_owned(&self) -> GoogleRevokeResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `GoogleRevokeResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GoogleRevokeResponseView<'_> {}

// SAFETY:
// - `GoogleRevokeResponseView` is `Send` because while its alive a `GoogleRevokeResponseMut` cannot.
// - `GoogleRevokeResponseView` does not use thread-local data.
unsafe impl Send for GoogleRevokeResponseView<'_> {}

impl<'msg> ::protobuf::AsView for GoogleRevokeResponseView<'msg> {
  type Proxied = GoogleRevokeResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, GoogleRevokeResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GoogleRevokeResponseView<'msg> {
  fn into_view<'shorter>(self) -> GoogleRevokeResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GoogleRevokeResponse> for GoogleRevokeResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GoogleRevokeResponse {
    let mut dst = GoogleRevokeResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GoogleRevokeResponse> for GoogleRevokeResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GoogleRevokeResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GoogleRevokeResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GoogleRevokeResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GoogleRevokeResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GoogleRevokeResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleRevokeResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GoogleRevokeResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GoogleRevokeResponseMut<'msg> {
  type Message = GoogleRevokeResponse;
}

impl ::std::fmt::Debug for GoogleRevokeResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleRevokeResponse>> for GoogleRevokeResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleRevokeResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GoogleRevokeResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleRevokeResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> GoogleRevokeResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `GoogleRevokeResponseMut` does not perform any shared mutation.
unsafe impl Send for GoogleRevokeResponseMut<'_> {}

// SAFETY:
// - `GoogleRevokeResponseMut` does not perform any shared mutation.
unsafe impl Sync for GoogleRevokeResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for GoogleRevokeResponseMut<'msg> {
  type Proxied = GoogleRevokeResponse;
  fn as_view(&self) -> ::protobuf::View<'_, GoogleRevokeResponse> {
    GoogleRevokeResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GoogleRevokeResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GoogleRevokeResponse>
  where
      'msg: 'shorter {
    GoogleRevokeResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GoogleRevokeResponseMut<'msg> {
  type MutProxied = GoogleRevokeResponse;
  fn as_mut(&mut self) -> GoogleRevokeResponseMut<'msg> {
    GoogleRevokeResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GoogleRevokeResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> GoogleRevokeResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GoogleRevokeResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GoogleRevokeResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GoogleRevokeResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GoogleRevokeResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl GoogleRevokeResponse

impl ::std::ops::Drop for GoogleRevokeResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GoogleRevokeResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GoogleRevokeResponse {
  type Proxied = Self;
  fn as_view(&self) -> GoogleRevokeResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GoogleRevokeResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GoogleRevokeResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GoogleRevokeResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__oauth2__v1__GoogleRevokeResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__oauth2__v1__GoogleRevokeResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__oauth2__v1__GoogleRevokeResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GoogleRevokeResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GoogleRevokeResponse {
  type Msg = GoogleRevokeResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleRevokeResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleRevokeResponse {
  type Msg = GoogleRevokeResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleRevokeResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GoogleRevokeResponseMut<'_> {
  type Msg = GoogleRevokeResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleRevokeResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleRevokeResponseMut<'_> {
  type Msg = GoogleRevokeResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleRevokeResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleRevokeResponseView<'_> {
  type Msg = GoogleRevokeResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleRevokeResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GoogleRevokeResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__oauth2__v1__GoogleUserInfoResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GoogleUserInfoResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GoogleUserInfoResponse>
}

impl ::protobuf::Message for GoogleUserInfoResponse {}

impl ::std::default::Default for GoogleUserInfoResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GoogleUserInfoResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GoogleUserInfoResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GoogleUserInfoResponseMut`.
unsafe impl Sync for GoogleUserInfoResponse {}

// SAFETY:
// - `GoogleUserInfoResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GoogleUserInfoResponse {}

impl ::protobuf::Proxied for GoogleUserInfoResponse {
  type View<'msg> = GoogleUserInfoResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GoogleUserInfoResponse {}

impl ::protobuf::MutProxied for GoogleUserInfoResponse {
  type Mut<'msg> = GoogleUserInfoResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GoogleUserInfoResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleUserInfoResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GoogleUserInfoResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GoogleUserInfoResponseView<'msg> {
  type Message = GoogleUserInfoResponse;
}

impl ::std::fmt::Debug for GoogleUserInfoResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GoogleUserInfoResponseView<'_> {
  fn default() -> GoogleUserInfoResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleUserInfoResponse>> for GoogleUserInfoResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleUserInfoResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GoogleUserInfoResponseView<'msg> {

  pub fn to_owned(&self) -> GoogleUserInfoResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // sub: optional string
  pub fn sub(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // email: optional string
  pub fn email(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // verified_email: optional bool
  pub fn verified_email(self) -> bool {
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

  // name: optional string
  pub fn has_name(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn name_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.name(), self.has_name())
  }
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // given_name: optional string
  pub fn has_given_name(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn given_name_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.given_name(), self.has_given_name())
  }
  pub fn given_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // family_name: optional string
  pub fn has_family_name(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn family_name_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.family_name(), self.has_family_name())
  }
  pub fn family_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // picture: optional string
  pub fn has_picture(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn picture_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.picture(), self.has_picture())
  }
  pub fn picture(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // locale: optional string
  pub fn has_locale(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn locale_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.locale(), self.has_locale())
  }
  pub fn locale(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // hosted_domain: optional string
  pub fn has_hosted_domain(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn hosted_domain_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.hosted_domain(), self.has_hosted_domain())
  }
  pub fn hosted_domain(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // gender: optional string
  pub fn has_gender(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn gender_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.gender(), self.has_gender())
  }
  pub fn gender(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // link: optional string
  pub fn has_link(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn link_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.link(), self.has_link())
  }
  pub fn link(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        10, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `GoogleUserInfoResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GoogleUserInfoResponseView<'_> {}

// SAFETY:
// - `GoogleUserInfoResponseView` is `Send` because while its alive a `GoogleUserInfoResponseMut` cannot.
// - `GoogleUserInfoResponseView` does not use thread-local data.
unsafe impl Send for GoogleUserInfoResponseView<'_> {}

impl<'msg> ::protobuf::AsView for GoogleUserInfoResponseView<'msg> {
  type Proxied = GoogleUserInfoResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, GoogleUserInfoResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GoogleUserInfoResponseView<'msg> {
  fn into_view<'shorter>(self) -> GoogleUserInfoResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GoogleUserInfoResponse> for GoogleUserInfoResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GoogleUserInfoResponse {
    let mut dst = GoogleUserInfoResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GoogleUserInfoResponse> for GoogleUserInfoResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GoogleUserInfoResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GoogleUserInfoResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GoogleUserInfoResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GoogleUserInfoResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GoogleUserInfoResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleUserInfoResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GoogleUserInfoResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GoogleUserInfoResponseMut<'msg> {
  type Message = GoogleUserInfoResponse;
}

impl ::std::fmt::Debug for GoogleUserInfoResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleUserInfoResponse>> for GoogleUserInfoResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleUserInfoResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GoogleUserInfoResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleUserInfoResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> GoogleUserInfoResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // sub: optional string
  pub fn sub(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_sub(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // email: optional string
  pub fn email(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_email(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // verified_email: optional bool
  pub fn verified_email(&self) -> bool {
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
  pub fn set_verified_email(&mut self, val: bool) {
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

  // name: optional string
  pub fn has_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn name_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.name(), self.has_name())
  }
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // given_name: optional string
  pub fn has_given_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_given_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn given_name_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.given_name(), self.has_given_name())
  }
  pub fn given_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_given_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // family_name: optional string
  pub fn has_family_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_family_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn family_name_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.family_name(), self.has_family_name())
  }
  pub fn family_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_family_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // picture: optional string
  pub fn has_picture(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_picture(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn picture_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.picture(), self.has_picture())
  }
  pub fn picture(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_picture(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // locale: optional string
  pub fn has_locale(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_locale(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn locale_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.locale(), self.has_locale())
  }
  pub fn locale(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_locale(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val);
    }
  }

  // hosted_domain: optional string
  pub fn has_hosted_domain(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_hosted_domain(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn hosted_domain_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.hosted_domain(), self.has_hosted_domain())
  }
  pub fn hosted_domain(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_hosted_domain(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

  // gender: optional string
  pub fn has_gender(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_gender(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn gender_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.gender(), self.has_gender())
  }
  pub fn gender(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_gender(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // link: optional string
  pub fn has_link(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_link(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn link_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.link(), self.has_link())
  }
  pub fn link(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        10, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_link(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val);
    }
  }

}

// SAFETY:
// - `GoogleUserInfoResponseMut` does not perform any shared mutation.
unsafe impl Send for GoogleUserInfoResponseMut<'_> {}

// SAFETY:
// - `GoogleUserInfoResponseMut` does not perform any shared mutation.
unsafe impl Sync for GoogleUserInfoResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for GoogleUserInfoResponseMut<'msg> {
  type Proxied = GoogleUserInfoResponse;
  fn as_view(&self) -> ::protobuf::View<'_, GoogleUserInfoResponse> {
    GoogleUserInfoResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GoogleUserInfoResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GoogleUserInfoResponse>
  where
      'msg: 'shorter {
    GoogleUserInfoResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GoogleUserInfoResponseMut<'msg> {
  type MutProxied = GoogleUserInfoResponse;
  fn as_mut(&mut self) -> GoogleUserInfoResponseMut<'msg> {
    GoogleUserInfoResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GoogleUserInfoResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> GoogleUserInfoResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GoogleUserInfoResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GoogleUserInfoResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GoogleUserInfoResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GoogleUserInfoResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // sub: optional string
  pub fn sub(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_sub(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // email: optional string
  pub fn email(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_email(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // verified_email: optional bool
  pub fn verified_email(&self) -> bool {
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
  pub fn set_verified_email(&mut self, val: bool) {
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

  // name: optional string
  pub fn has_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn name_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.name(), self.has_name())
  }
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // given_name: optional string
  pub fn has_given_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_given_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn given_name_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.given_name(), self.has_given_name())
  }
  pub fn given_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_given_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // family_name: optional string
  pub fn has_family_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_family_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn family_name_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.family_name(), self.has_family_name())
  }
  pub fn family_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_family_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // picture: optional string
  pub fn has_picture(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_picture(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn picture_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.picture(), self.has_picture())
  }
  pub fn picture(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_picture(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // locale: optional string
  pub fn has_locale(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_locale(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn locale_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.locale(), self.has_locale())
  }
  pub fn locale(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_locale(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val);
    }
  }

  // hosted_domain: optional string
  pub fn has_hosted_domain(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_hosted_domain(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn hosted_domain_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.hosted_domain(), self.has_hosted_domain())
  }
  pub fn hosted_domain(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_hosted_domain(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

  // gender: optional string
  pub fn has_gender(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_gender(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn gender_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.gender(), self.has_gender())
  }
  pub fn gender(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_gender(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // link: optional string
  pub fn has_link(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_link(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn link_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.link(), self.has_link())
  }
  pub fn link(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        10, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_link(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val);
    }
  }

}  // impl GoogleUserInfoResponse

impl ::std::ops::Drop for GoogleUserInfoResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GoogleUserInfoResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GoogleUserInfoResponse {
  type Proxied = Self;
  fn as_view(&self) -> GoogleUserInfoResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GoogleUserInfoResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GoogleUserInfoResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GoogleUserInfoResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__oauth2__v1__GoogleUserInfoResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X/P1T1T1T1T1T1T1T1T");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__oauth2__v1__GoogleUserInfoResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__oauth2__v1__GoogleUserInfoResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GoogleUserInfoResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GoogleUserInfoResponse {
  type Msg = GoogleUserInfoResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleUserInfoResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleUserInfoResponse {
  type Msg = GoogleUserInfoResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleUserInfoResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GoogleUserInfoResponseMut<'_> {
  type Msg = GoogleUserInfoResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleUserInfoResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleUserInfoResponseMut<'_> {
  type Msg = GoogleUserInfoResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleUserInfoResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleUserInfoResponseView<'_> {
  type Msg = GoogleUserInfoResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleUserInfoResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GoogleUserInfoResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



