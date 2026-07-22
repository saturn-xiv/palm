const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.34.0-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__protobuf__Duration_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Duration {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Duration>
}

impl ::protobuf::Message for Duration {}

impl ::std::default::Default for Duration {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Duration {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Duration` is `Sync` because it does not implement interior mutability.
//    Neither does `DurationMut`.
unsafe impl Sync for Duration {}

// SAFETY:
// - `Duration` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Duration {}

impl ::protobuf::Proxied for Duration {
  type View<'msg> = DurationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Duration {}

impl ::protobuf::MutProxied for Duration {
  type Mut<'msg> = DurationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DurationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Duration>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DurationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DurationView<'msg> {
  type Message = Duration;
}

impl ::std::fmt::Debug for DurationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DurationView<'_> {
  fn default() -> DurationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Duration>> for DurationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Duration>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DurationView<'msg> {

  pub fn to_owned(&self) -> Duration {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // seconds: optional int64
  pub fn seconds(self) -> i64 {
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

  // nanos: optional int32
  pub fn nanos(self) -> i32 {
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
// - `DurationView` is `Sync` because it does not support mutation.
unsafe impl Sync for DurationView<'_> {}

// SAFETY:
// - `DurationView` is `Send` because while its alive a `DurationMut` cannot.
// - `DurationView` does not use thread-local data.
unsafe impl Send for DurationView<'_> {}

impl<'msg> ::protobuf::AsView for DurationView<'msg> {
  type Proxied = Duration;
  fn as_view(&self) -> ::protobuf::View<'msg, Duration> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DurationView<'msg> {
  fn into_view<'shorter>(self) -> DurationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Duration> for DurationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Duration {
    let mut dst = Duration::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Duration> for DurationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Duration {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Duration {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DurationView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DurationMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DurationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Duration>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DurationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DurationMut<'msg> {
  type Message = Duration;
}

impl ::std::fmt::Debug for DurationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Duration>> for DurationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Duration>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DurationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Duration> {
    self.inner
  }

  pub fn to_owned(&self) -> Duration {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // seconds: optional int64
  pub fn seconds(&self) -> i64 {
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
  pub fn set_seconds(&mut self, val: i64) {
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

  // nanos: optional int32
  pub fn nanos(&self) -> i32 {
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
  pub fn set_nanos(&mut self, val: i32) {
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
// - `DurationMut` does not perform any shared mutation.
unsafe impl Send for DurationMut<'_> {}

// SAFETY:
// - `DurationMut` does not perform any shared mutation.
unsafe impl Sync for DurationMut<'_> {}

impl<'msg> ::protobuf::AsView for DurationMut<'msg> {
  type Proxied = Duration;
  fn as_view(&self) -> ::protobuf::View<'_, Duration> {
    DurationView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DurationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Duration>
  where
      'msg: 'shorter {
    DurationView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for DurationMut<'msg> {
  type MutProxied = Duration;
  fn as_mut(&mut self) -> DurationMut<'msg> {
    DurationMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DurationMut<'msg> {
  fn into_mut<'shorter>(self) -> DurationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Duration {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Duration> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DurationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DurationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // seconds: optional int64
  pub fn seconds(&self) -> i64 {
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
  pub fn set_seconds(&mut self, val: i64) {
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

  // nanos: optional int32
  pub fn nanos(&self) -> i32 {
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
  pub fn set_nanos(&mut self, val: i32) {
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

}  // impl Duration

impl ::std::ops::Drop for Duration {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Duration {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Duration {
  type Proxied = Self;
  fn as_view(&self) -> DurationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Duration {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DurationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Duration {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__protobuf__Duration_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$+P(P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__protobuf__Duration_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__protobuf__Duration_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Duration {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Duration {
  type Msg = Duration;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Duration> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Duration {
  type Msg = Duration;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Duration> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DurationMut<'_> {
  type Msg = Duration;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Duration> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DurationMut<'_> {
  type Msg = Duration;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Duration> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DurationView<'_> {
  type Msg = Duration;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Duration> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DurationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



