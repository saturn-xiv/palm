const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.34.0-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__lavender__v1__Empty_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
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
        super::palm__lavender__v1__Empty_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__lavender__v1__Empty_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__lavender__v1__Empty_msg_init.0)
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
pub(crate) static mut palm__lavender__v1__Timestamp_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Timestamp {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Timestamp>
}

impl ::protobuf::Message for Timestamp {}

impl ::std::default::Default for Timestamp {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Timestamp {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Timestamp` is `Sync` because it does not implement interior mutability.
//    Neither does `TimestampMut`.
unsafe impl Sync for Timestamp {}

// SAFETY:
// - `Timestamp` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Timestamp {}

impl ::protobuf::Proxied for Timestamp {
  type View<'msg> = TimestampView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Timestamp {}

impl ::protobuf::MutProxied for Timestamp {
  type Mut<'msg> = TimestampMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TimestampView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Timestamp>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TimestampView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TimestampView<'msg> {
  type Message = Timestamp;
}

impl ::std::fmt::Debug for TimestampView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TimestampView<'_> {
  fn default() -> TimestampView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Timestamp>> for TimestampView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Timestamp>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TimestampView<'msg> {

  pub fn to_owned(&self) -> Timestamp {
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
// - `TimestampView` is `Sync` because it does not support mutation.
unsafe impl Sync for TimestampView<'_> {}

// SAFETY:
// - `TimestampView` is `Send` because while its alive a `TimestampMut` cannot.
// - `TimestampView` does not use thread-local data.
unsafe impl Send for TimestampView<'_> {}

impl<'msg> ::protobuf::AsView for TimestampView<'msg> {
  type Proxied = Timestamp;
  fn as_view(&self) -> ::protobuf::View<'msg, Timestamp> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TimestampView<'msg> {
  fn into_view<'shorter>(self) -> TimestampView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Timestamp> for TimestampView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Timestamp {
    let mut dst = Timestamp::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Timestamp> for TimestampMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Timestamp {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Timestamp {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for TimestampView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for TimestampMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TimestampMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Timestamp>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TimestampMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TimestampMut<'msg> {
  type Message = Timestamp;
}

impl ::std::fmt::Debug for TimestampMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Timestamp>> for TimestampMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Timestamp>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TimestampMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Timestamp> {
    self.inner
  }

  pub fn to_owned(&self) -> Timestamp {
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
// - `TimestampMut` does not perform any shared mutation.
unsafe impl Send for TimestampMut<'_> {}

// SAFETY:
// - `TimestampMut` does not perform any shared mutation.
unsafe impl Sync for TimestampMut<'_> {}

impl<'msg> ::protobuf::AsView for TimestampMut<'msg> {
  type Proxied = Timestamp;
  fn as_view(&self) -> ::protobuf::View<'_, Timestamp> {
    TimestampView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TimestampMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Timestamp>
  where
      'msg: 'shorter {
    TimestampView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for TimestampMut<'msg> {
  type MutProxied = Timestamp;
  fn as_mut(&mut self) -> TimestampMut<'msg> {
    TimestampMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TimestampMut<'msg> {
  fn into_mut<'shorter>(self) -> TimestampMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Timestamp {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Timestamp> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TimestampView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TimestampMut<'_> {
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

}  // impl Timestamp

impl ::std::ops::Drop for Timestamp {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Timestamp {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Timestamp {
  type Proxied = Self;
  fn as_view(&self) -> TimestampView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Timestamp {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TimestampMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Timestamp {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__lavender__v1__Timestamp_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$+P(P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__lavender__v1__Timestamp_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__lavender__v1__Timestamp_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Timestamp {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Timestamp {
  type Msg = Timestamp;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Timestamp> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Timestamp {
  type Msg = Timestamp;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Timestamp> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TimestampMut<'_> {
  type Msg = Timestamp;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Timestamp> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TimestampMut<'_> {
  type Msg = Timestamp;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Timestamp> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TimestampView<'_> {
  type Msg = Timestamp;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Timestamp> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TimestampMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__lavender__v1__Duration_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
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
        super::palm__lavender__v1__Duration_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$+P(P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__lavender__v1__Duration_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__lavender__v1__Duration_msg_init.0)
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



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__lavender__v1__SystemdRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SystemdRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SystemdRequest>
}

impl ::protobuf::Message for SystemdRequest {}

impl ::std::default::Default for SystemdRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SystemdRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SystemdRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `SystemdRequestMut`.
unsafe impl Sync for SystemdRequest {}

// SAFETY:
// - `SystemdRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SystemdRequest {}

impl ::protobuf::Proxied for SystemdRequest {
  type View<'msg> = SystemdRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SystemdRequest {}

impl ::protobuf::MutProxied for SystemdRequest {
  type Mut<'msg> = SystemdRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SystemdRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SystemdRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SystemdRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SystemdRequestView<'msg> {
  type Message = SystemdRequest;
}

impl ::std::fmt::Debug for SystemdRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SystemdRequestView<'_> {
  fn default() -> SystemdRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SystemdRequest>> for SystemdRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SystemdRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SystemdRequestView<'msg> {

  pub fn to_owned(&self) -> SystemdRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // host: optional string
  pub fn host(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // unit: optional string
  pub fn unit(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // priority: optional uint32
  pub fn priority(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // message: optional string
  pub fn message(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // created_at: optional message palm.lavender.v1.Timestamp
  pub fn has_created_at(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn created_at_opt(self) -> ::protobuf::Optional<super::TimestampView<'msg>> {
        ::protobuf::Optional::new(self.created_at(), self.has_created_at())
  }
  pub fn created_at(self) -> super::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TimestampView::default())
  }

}

// SAFETY:
// - `SystemdRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for SystemdRequestView<'_> {}

// SAFETY:
// - `SystemdRequestView` is `Send` because while its alive a `SystemdRequestMut` cannot.
// - `SystemdRequestView` does not use thread-local data.
unsafe impl Send for SystemdRequestView<'_> {}

impl<'msg> ::protobuf::AsView for SystemdRequestView<'msg> {
  type Proxied = SystemdRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, SystemdRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SystemdRequestView<'msg> {
  fn into_view<'shorter>(self) -> SystemdRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SystemdRequest> for SystemdRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SystemdRequest {
    let mut dst = SystemdRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SystemdRequest> for SystemdRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SystemdRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for SystemdRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SystemdRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SystemdRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SystemdRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SystemdRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SystemdRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SystemdRequestMut<'msg> {
  type Message = SystemdRequest;
}

impl ::std::fmt::Debug for SystemdRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SystemdRequest>> for SystemdRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SystemdRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SystemdRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SystemdRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> SystemdRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // host: optional string
  pub fn host(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_host(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // unit: optional string
  pub fn unit(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_unit(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // priority: optional uint32
  pub fn priority(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_priority(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        2, val.into()
      )
    }
  }

  // message: optional string
  pub fn message(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_message(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // created_at: optional message palm.lavender.v1.Timestamp
  pub fn has_created_at(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_created_at(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn created_at_opt(&self) -> ::protobuf::Optional<super::TimestampView<'_>> {
        ::protobuf::Optional::new(self.created_at(), self.has_created_at())
  }
  pub fn created_at(&self) -> super::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TimestampView::default())
  }
  pub fn created_at_mut(&mut self) -> super::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
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
        4,
        val
      );
    }
  }

}

// SAFETY:
// - `SystemdRequestMut` does not perform any shared mutation.
unsafe impl Send for SystemdRequestMut<'_> {}

// SAFETY:
// - `SystemdRequestMut` does not perform any shared mutation.
unsafe impl Sync for SystemdRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for SystemdRequestMut<'msg> {
  type Proxied = SystemdRequest;
  fn as_view(&self) -> ::protobuf::View<'_, SystemdRequest> {
    SystemdRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SystemdRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SystemdRequest>
  where
      'msg: 'shorter {
    SystemdRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for SystemdRequestMut<'msg> {
  type MutProxied = SystemdRequest;
  fn as_mut(&mut self) -> SystemdRequestMut<'msg> {
    SystemdRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SystemdRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> SystemdRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SystemdRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SystemdRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SystemdRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SystemdRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // host: optional string
  pub fn host(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_host(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // unit: optional string
  pub fn unit(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_unit(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // priority: optional uint32
  pub fn priority(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_priority(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        2, val.into()
      )
    }
  }

  // message: optional string
  pub fn message(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_message(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // created_at: optional message palm.lavender.v1.Timestamp
  pub fn has_created_at(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_created_at(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn created_at_opt(&self) -> ::protobuf::Optional<super::TimestampView<'_>> {
        ::protobuf::Optional::new(self.created_at(), self.has_created_at())
  }
  pub fn created_at(&self) -> super::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TimestampView::default())
  }
  pub fn created_at_mut(&mut self) -> super::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
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
        4,
        val
      );
    }
  }

}  // impl SystemdRequest

impl ::std::ops::Drop for SystemdRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SystemdRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SystemdRequest {
  type Proxied = Self;
  fn as_view(&self) -> SystemdRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SystemdRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SystemdRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SystemdRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__lavender__v1__SystemdRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X)P1Xd3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__lavender__v1__SystemdRequest_msg_init.0, &[<super::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__lavender__v1__SystemdRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SystemdRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SystemdRequest {
  type Msg = SystemdRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SystemdRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SystemdRequest {
  type Msg = SystemdRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SystemdRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SystemdRequestMut<'_> {
  type Msg = SystemdRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SystemdRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SystemdRequestMut<'_> {
  type Msg = SystemdRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SystemdRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SystemdRequestView<'_> {
  type Msg = SystemdRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SystemdRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SystemdRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__lavender__v1__SystemdResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SystemdResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SystemdResponse>
}

impl ::protobuf::Message for SystemdResponse {}

impl ::std::default::Default for SystemdResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SystemdResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SystemdResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `SystemdResponseMut`.
unsafe impl Sync for SystemdResponse {}

// SAFETY:
// - `SystemdResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SystemdResponse {}

impl ::protobuf::Proxied for SystemdResponse {
  type View<'msg> = SystemdResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SystemdResponse {}

impl ::protobuf::MutProxied for SystemdResponse {
  type Mut<'msg> = SystemdResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SystemdResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SystemdResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SystemdResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SystemdResponseView<'msg> {
  type Message = SystemdResponse;
}

impl ::std::fmt::Debug for SystemdResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SystemdResponseView<'_> {
  fn default() -> SystemdResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SystemdResponse>> for SystemdResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SystemdResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SystemdResponseView<'msg> {

  pub fn to_owned(&self) -> SystemdResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // chunk_count: optional uint32
  pub fn chunk_count(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `SystemdResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for SystemdResponseView<'_> {}

// SAFETY:
// - `SystemdResponseView` is `Send` because while its alive a `SystemdResponseMut` cannot.
// - `SystemdResponseView` does not use thread-local data.
unsafe impl Send for SystemdResponseView<'_> {}

impl<'msg> ::protobuf::AsView for SystemdResponseView<'msg> {
  type Proxied = SystemdResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, SystemdResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SystemdResponseView<'msg> {
  fn into_view<'shorter>(self) -> SystemdResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SystemdResponse> for SystemdResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SystemdResponse {
    let mut dst = SystemdResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SystemdResponse> for SystemdResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SystemdResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for SystemdResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SystemdResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SystemdResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SystemdResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SystemdResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SystemdResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SystemdResponseMut<'msg> {
  type Message = SystemdResponse;
}

impl ::std::fmt::Debug for SystemdResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SystemdResponse>> for SystemdResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SystemdResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SystemdResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SystemdResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> SystemdResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // chunk_count: optional uint32
  pub fn chunk_count(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_chunk_count(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

}

// SAFETY:
// - `SystemdResponseMut` does not perform any shared mutation.
unsafe impl Send for SystemdResponseMut<'_> {}

// SAFETY:
// - `SystemdResponseMut` does not perform any shared mutation.
unsafe impl Sync for SystemdResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for SystemdResponseMut<'msg> {
  type Proxied = SystemdResponse;
  fn as_view(&self) -> ::protobuf::View<'_, SystemdResponse> {
    SystemdResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SystemdResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SystemdResponse>
  where
      'msg: 'shorter {
    SystemdResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for SystemdResponseMut<'msg> {
  type MutProxied = SystemdResponse;
  fn as_mut(&mut self) -> SystemdResponseMut<'msg> {
    SystemdResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SystemdResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> SystemdResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SystemdResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SystemdResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SystemdResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SystemdResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // chunk_count: optional uint32
  pub fn chunk_count(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_chunk_count(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

}  // impl SystemdResponse

impl ::std::ops::Drop for SystemdResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SystemdResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SystemdResponse {
  type Proxied = Self;
  fn as_view(&self) -> SystemdResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SystemdResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SystemdResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SystemdResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__lavender__v1__SystemdResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$)P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__lavender__v1__SystemdResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__lavender__v1__SystemdResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SystemdResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SystemdResponse {
  type Msg = SystemdResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SystemdResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SystemdResponse {
  type Msg = SystemdResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SystemdResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SystemdResponseMut<'_> {
  type Msg = SystemdResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SystemdResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SystemdResponseMut<'_> {
  type Msg = SystemdResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SystemdResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SystemdResponseView<'_> {
  type Msg = SystemdResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SystemdResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SystemdResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__lavender__v1__KubernetesRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct KubernetesRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<KubernetesRequest>
}

impl ::protobuf::Message for KubernetesRequest {}

impl ::std::default::Default for KubernetesRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for KubernetesRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `KubernetesRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `KubernetesRequestMut`.
unsafe impl Sync for KubernetesRequest {}

// SAFETY:
// - `KubernetesRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for KubernetesRequest {}

impl ::protobuf::Proxied for KubernetesRequest {
  type View<'msg> = KubernetesRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for KubernetesRequest {}

impl ::protobuf::MutProxied for KubernetesRequest {
  type Mut<'msg> = KubernetesRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct KubernetesRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, KubernetesRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for KubernetesRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for KubernetesRequestView<'msg> {
  type Message = KubernetesRequest;
}

impl ::std::fmt::Debug for KubernetesRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for KubernetesRequestView<'_> {
  fn default() -> KubernetesRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, KubernetesRequest>> for KubernetesRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, KubernetesRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> KubernetesRequestView<'msg> {

  pub fn to_owned(&self) -> KubernetesRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // node: optional string
  pub fn node(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // pod: optional string
  pub fn pod(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // container: optional string
  pub fn container(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // message: optional string
  pub fn message(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // created_at: optional message palm.lavender.v1.Timestamp
  pub fn has_created_at(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn created_at_opt(self) -> ::protobuf::Optional<super::TimestampView<'msg>> {
        ::protobuf::Optional::new(self.created_at(), self.has_created_at())
  }
  pub fn created_at(self) -> super::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TimestampView::default())
  }

}

// SAFETY:
// - `KubernetesRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for KubernetesRequestView<'_> {}

// SAFETY:
// - `KubernetesRequestView` is `Send` because while its alive a `KubernetesRequestMut` cannot.
// - `KubernetesRequestView` does not use thread-local data.
unsafe impl Send for KubernetesRequestView<'_> {}

impl<'msg> ::protobuf::AsView for KubernetesRequestView<'msg> {
  type Proxied = KubernetesRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, KubernetesRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for KubernetesRequestView<'msg> {
  fn into_view<'shorter>(self) -> KubernetesRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<KubernetesRequest> for KubernetesRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> KubernetesRequest {
    let mut dst = KubernetesRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<KubernetesRequest> for KubernetesRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> KubernetesRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for KubernetesRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for KubernetesRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for KubernetesRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct KubernetesRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, KubernetesRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for KubernetesRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for KubernetesRequestMut<'msg> {
  type Message = KubernetesRequest;
}

impl ::std::fmt::Debug for KubernetesRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, KubernetesRequest>> for KubernetesRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, KubernetesRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> KubernetesRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, KubernetesRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> KubernetesRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // node: optional string
  pub fn node(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_node(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // pod: optional string
  pub fn pod(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_pod(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // container: optional string
  pub fn container(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_container(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // message: optional string
  pub fn message(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_message(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // created_at: optional message palm.lavender.v1.Timestamp
  pub fn has_created_at(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_created_at(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn created_at_opt(&self) -> ::protobuf::Optional<super::TimestampView<'_>> {
        ::protobuf::Optional::new(self.created_at(), self.has_created_at())
  }
  pub fn created_at(&self) -> super::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TimestampView::default())
  }
  pub fn created_at_mut(&mut self) -> super::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
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
        4,
        val
      );
    }
  }

}

// SAFETY:
// - `KubernetesRequestMut` does not perform any shared mutation.
unsafe impl Send for KubernetesRequestMut<'_> {}

// SAFETY:
// - `KubernetesRequestMut` does not perform any shared mutation.
unsafe impl Sync for KubernetesRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for KubernetesRequestMut<'msg> {
  type Proxied = KubernetesRequest;
  fn as_view(&self) -> ::protobuf::View<'_, KubernetesRequest> {
    KubernetesRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for KubernetesRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, KubernetesRequest>
  where
      'msg: 'shorter {
    KubernetesRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for KubernetesRequestMut<'msg> {
  type MutProxied = KubernetesRequest;
  fn as_mut(&mut self) -> KubernetesRequestMut<'msg> {
    KubernetesRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for KubernetesRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> KubernetesRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl KubernetesRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, KubernetesRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> KubernetesRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> KubernetesRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // node: optional string
  pub fn node(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_node(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // pod: optional string
  pub fn pod(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_pod(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // container: optional string
  pub fn container(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_container(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // message: optional string
  pub fn message(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_message(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // created_at: optional message palm.lavender.v1.Timestamp
  pub fn has_created_at(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_created_at(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn created_at_opt(&self) -> ::protobuf::Optional<super::TimestampView<'_>> {
        ::protobuf::Optional::new(self.created_at(), self.has_created_at())
  }
  pub fn created_at(&self) -> super::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TimestampView::default())
  }
  pub fn created_at_mut(&mut self) -> super::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
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
        4,
        val
      );
    }
  }

}  // impl KubernetesRequest

impl ::std::ops::Drop for KubernetesRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for KubernetesRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for KubernetesRequest {
  type Proxied = Self;
  fn as_view(&self) -> KubernetesRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for KubernetesRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> KubernetesRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for KubernetesRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__lavender__v1__KubernetesRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X1X1Xd3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__lavender__v1__KubernetesRequest_msg_init.0, &[<super::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__lavender__v1__KubernetesRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for KubernetesRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for KubernetesRequest {
  type Msg = KubernetesRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KubernetesRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KubernetesRequest {
  type Msg = KubernetesRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KubernetesRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for KubernetesRequestMut<'_> {
  type Msg = KubernetesRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KubernetesRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KubernetesRequestMut<'_> {
  type Msg = KubernetesRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KubernetesRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KubernetesRequestView<'_> {
  type Msg = KubernetesRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KubernetesRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for KubernetesRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__lavender__v1__KubernetesResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct KubernetesResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<KubernetesResponse>
}

impl ::protobuf::Message for KubernetesResponse {}

impl ::std::default::Default for KubernetesResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for KubernetesResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `KubernetesResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `KubernetesResponseMut`.
unsafe impl Sync for KubernetesResponse {}

// SAFETY:
// - `KubernetesResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for KubernetesResponse {}

impl ::protobuf::Proxied for KubernetesResponse {
  type View<'msg> = KubernetesResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for KubernetesResponse {}

impl ::protobuf::MutProxied for KubernetesResponse {
  type Mut<'msg> = KubernetesResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct KubernetesResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, KubernetesResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for KubernetesResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for KubernetesResponseView<'msg> {
  type Message = KubernetesResponse;
}

impl ::std::fmt::Debug for KubernetesResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for KubernetesResponseView<'_> {
  fn default() -> KubernetesResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, KubernetesResponse>> for KubernetesResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, KubernetesResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> KubernetesResponseView<'msg> {

  pub fn to_owned(&self) -> KubernetesResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // chunk_count: optional uint32
  pub fn chunk_count(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `KubernetesResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for KubernetesResponseView<'_> {}

// SAFETY:
// - `KubernetesResponseView` is `Send` because while its alive a `KubernetesResponseMut` cannot.
// - `KubernetesResponseView` does not use thread-local data.
unsafe impl Send for KubernetesResponseView<'_> {}

impl<'msg> ::protobuf::AsView for KubernetesResponseView<'msg> {
  type Proxied = KubernetesResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, KubernetesResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for KubernetesResponseView<'msg> {
  fn into_view<'shorter>(self) -> KubernetesResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<KubernetesResponse> for KubernetesResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> KubernetesResponse {
    let mut dst = KubernetesResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<KubernetesResponse> for KubernetesResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> KubernetesResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for KubernetesResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for KubernetesResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for KubernetesResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct KubernetesResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, KubernetesResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for KubernetesResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for KubernetesResponseMut<'msg> {
  type Message = KubernetesResponse;
}

impl ::std::fmt::Debug for KubernetesResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, KubernetesResponse>> for KubernetesResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, KubernetesResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> KubernetesResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, KubernetesResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> KubernetesResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // chunk_count: optional uint32
  pub fn chunk_count(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_chunk_count(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

}

// SAFETY:
// - `KubernetesResponseMut` does not perform any shared mutation.
unsafe impl Send for KubernetesResponseMut<'_> {}

// SAFETY:
// - `KubernetesResponseMut` does not perform any shared mutation.
unsafe impl Sync for KubernetesResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for KubernetesResponseMut<'msg> {
  type Proxied = KubernetesResponse;
  fn as_view(&self) -> ::protobuf::View<'_, KubernetesResponse> {
    KubernetesResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for KubernetesResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, KubernetesResponse>
  where
      'msg: 'shorter {
    KubernetesResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for KubernetesResponseMut<'msg> {
  type MutProxied = KubernetesResponse;
  fn as_mut(&mut self) -> KubernetesResponseMut<'msg> {
    KubernetesResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for KubernetesResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> KubernetesResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl KubernetesResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, KubernetesResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> KubernetesResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> KubernetesResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // chunk_count: optional uint32
  pub fn chunk_count(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_chunk_count(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

}  // impl KubernetesResponse

impl ::std::ops::Drop for KubernetesResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for KubernetesResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for KubernetesResponse {
  type Proxied = Self;
  fn as_view(&self) -> KubernetesResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for KubernetesResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> KubernetesResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for KubernetesResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__lavender__v1__KubernetesResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$)P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__lavender__v1__KubernetesResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__lavender__v1__KubernetesResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for KubernetesResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for KubernetesResponse {
  type Msg = KubernetesResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KubernetesResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KubernetesResponse {
  type Msg = KubernetesResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KubernetesResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for KubernetesResponseMut<'_> {
  type Msg = KubernetesResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KubernetesResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KubernetesResponseMut<'_> {
  type Msg = KubernetesResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KubernetesResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KubernetesResponseView<'_> {
  type Msg = KubernetesResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KubernetesResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for KubernetesResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__lavender__v1__HttpRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HttpRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HttpRequest>
}

impl ::protobuf::Message for HttpRequest {}

impl ::std::default::Default for HttpRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HttpRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HttpRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpRequestMut`.
unsafe impl Sync for HttpRequest {}

// SAFETY:
// - `HttpRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for HttpRequest {}

impl ::protobuf::Proxied for HttpRequest {
  type View<'msg> = HttpRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HttpRequest {}

impl ::protobuf::MutProxied for HttpRequest {
  type Mut<'msg> = HttpRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpRequestView<'msg> {
  type Message = HttpRequest;
}

impl ::std::fmt::Debug for HttpRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpRequestView<'_> {
  fn default() -> HttpRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HttpRequest>> for HttpRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpRequestView<'msg> {

  pub fn to_owned(&self) -> HttpRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // host: optional string
  pub fn host(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // items: repeated message palm.lavender.v1.HttpRequest.Item
  pub fn items(self) -> ::protobuf::RepeatedView<'msg, super::http_request::Item> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::http_request::Item>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `HttpRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for HttpRequestView<'_> {}

// SAFETY:
// - `HttpRequestView` is `Send` because while its alive a `HttpRequestMut` cannot.
// - `HttpRequestView` does not use thread-local data.
unsafe impl Send for HttpRequestView<'_> {}

impl<'msg> ::protobuf::AsView for HttpRequestView<'msg> {
  type Proxied = HttpRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, HttpRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpRequestView<'msg> {
  fn into_view<'shorter>(self) -> HttpRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpRequest> for HttpRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpRequest {
    let mut dst = HttpRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpRequest> for HttpRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for HttpRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for HttpRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for HttpRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpRequestMut<'msg> {
  type Message = HttpRequest;
}

impl ::std::fmt::Debug for HttpRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRequest>> for HttpRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> HttpRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // host: optional string
  pub fn host(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_host(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // items: repeated message palm.lavender.v1.HttpRequest.Item
  pub fn items(&self) -> ::protobuf::RepeatedView<'_, super::http_request::Item> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::http_request::Item>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::http_request::Item> {
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
  pub fn set_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::http_request::Item>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `HttpRequestMut` does not perform any shared mutation.
unsafe impl Send for HttpRequestMut<'_> {}

// SAFETY:
// - `HttpRequestMut` does not perform any shared mutation.
unsafe impl Sync for HttpRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpRequestMut<'msg> {
  type Proxied = HttpRequest;
  fn as_view(&self) -> ::protobuf::View<'_, HttpRequest> {
    HttpRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HttpRequest>
  where
      'msg: 'shorter {
    HttpRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for HttpRequestMut<'msg> {
  type MutProxied = HttpRequest;
  fn as_mut(&mut self) -> HttpRequestMut<'msg> {
    HttpRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HttpRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HttpRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // host: optional string
  pub fn host(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_host(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // items: repeated message palm.lavender.v1.HttpRequest.Item
  pub fn items(&self) -> ::protobuf::RepeatedView<'_, super::http_request::Item> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::http_request::Item>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::http_request::Item> {
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
  pub fn set_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::http_request::Item>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl HttpRequest

impl ::std::ops::Drop for HttpRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HttpRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HttpRequest {
  type Proxied = Self;
  fn as_view(&self) -> HttpRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HttpRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HttpRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__lavender__v1__HttpRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XgG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__lavender__v1__HttpRequest_msg_init.0, &[<super::http_request::Item as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__lavender__v1__HttpRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpRequest {
  type Msg = HttpRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpRequest {
  type Msg = HttpRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpRequestMut<'_> {
  type Msg = HttpRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpRequestMut<'_> {
  type Msg = HttpRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpRequestView<'_> {
  type Msg = HttpRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod http_request {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__lavender__v1__HttpRequest__Item_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Item {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Item>
}

impl ::protobuf::Message for Item {}

impl ::std::default::Default for Item {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Item {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Item` is `Sync` because it does not implement interior mutability.
//    Neither does `ItemMut`.
unsafe impl Sync for Item {}

// SAFETY:
// - `Item` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Item {}

impl ::protobuf::Proxied for Item {
  type View<'msg> = ItemView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Item {}

impl ::protobuf::MutProxied for Item {
  type Mut<'msg> = ItemMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ItemView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Item>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ItemView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ItemView<'msg> {
  type Message = Item;
}

impl ::std::fmt::Debug for ItemView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ItemView<'_> {
  fn default() -> ItemView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Item>> for ItemView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Item>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ItemView<'msg> {

  pub fn to_owned(&self) -> Item {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // status_code: optional uint32
  pub fn status_code(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // content_type: optional string
  pub fn content_type(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // body: optional string
  pub fn body(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // elapsed: optional message palm.lavender.v1.Duration
  pub fn has_elapsed(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn elapsed_opt(self) -> ::protobuf::Optional<super::super::DurationView<'msg>> {
        ::protobuf::Optional::new(self.elapsed(), self.has_elapsed())
  }
  pub fn elapsed(self) -> super::super::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::DurationView::default())
  }

  // created_at: optional message palm.lavender.v1.Timestamp
  pub fn has_created_at(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn created_at_opt(self) -> ::protobuf::Optional<super::super::TimestampView<'msg>> {
        ::protobuf::Optional::new(self.created_at(), self.has_created_at())
  }
  pub fn created_at(self) -> super::super::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::TimestampView::default())
  }

}

// SAFETY:
// - `ItemView` is `Sync` because it does not support mutation.
unsafe impl Sync for ItemView<'_> {}

// SAFETY:
// - `ItemView` is `Send` because while its alive a `ItemMut` cannot.
// - `ItemView` does not use thread-local data.
unsafe impl Send for ItemView<'_> {}

impl<'msg> ::protobuf::AsView for ItemView<'msg> {
  type Proxied = Item;
  fn as_view(&self) -> ::protobuf::View<'msg, Item> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ItemView<'msg> {
  fn into_view<'shorter>(self) -> ItemView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Item> for ItemView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Item {
    let mut dst = Item::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Item> for ItemMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Item {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Item {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ItemView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ItemMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ItemMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Item>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ItemMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ItemMut<'msg> {
  type Message = Item;
}

impl ::std::fmt::Debug for ItemMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Item>> for ItemMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Item>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ItemMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Item> {
    self.inner
  }

  pub fn to_owned(&self) -> Item {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // status_code: optional uint32
  pub fn status_code(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_status_code(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

  // content_type: optional string
  pub fn content_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_content_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // body: optional string
  pub fn body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // elapsed: optional message palm.lavender.v1.Duration
  pub fn has_elapsed(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_elapsed(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn elapsed_opt(&self) -> ::protobuf::Optional<super::super::DurationView<'_>> {
        ::protobuf::Optional::new(self.elapsed(), self.has_elapsed())
  }
  pub fn elapsed(&self) -> super::super::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::DurationView::default())
  }
  pub fn elapsed_mut(&mut self) -> super::super::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_elapsed(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // created_at: optional message palm.lavender.v1.Timestamp
  pub fn has_created_at(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_created_at(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn created_at_opt(&self) -> ::protobuf::Optional<super::super::TimestampView<'_>> {
        ::protobuf::Optional::new(self.created_at(), self.has_created_at())
  }
  pub fn created_at(&self) -> super::super::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::TimestampView::default())
  }
  pub fn created_at_mut(&mut self) -> super::super::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_created_at(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

}

// SAFETY:
// - `ItemMut` does not perform any shared mutation.
unsafe impl Send for ItemMut<'_> {}

// SAFETY:
// - `ItemMut` does not perform any shared mutation.
unsafe impl Sync for ItemMut<'_> {}

impl<'msg> ::protobuf::AsView for ItemMut<'msg> {
  type Proxied = Item;
  fn as_view(&self) -> ::protobuf::View<'_, Item> {
    ItemView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ItemMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Item>
  where
      'msg: 'shorter {
    ItemView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ItemMut<'msg> {
  type MutProxied = Item;
  fn as_mut(&mut self) -> ItemMut<'msg> {
    ItemMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ItemMut<'msg> {
  fn into_mut<'shorter>(self) -> ItemMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Item {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Item> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ItemView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ItemMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // status_code: optional uint32
  pub fn status_code(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_status_code(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

  // content_type: optional string
  pub fn content_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_content_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // body: optional string
  pub fn body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // elapsed: optional message palm.lavender.v1.Duration
  pub fn has_elapsed(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_elapsed(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn elapsed_opt(&self) -> ::protobuf::Optional<super::super::DurationView<'_>> {
        ::protobuf::Optional::new(self.elapsed(), self.has_elapsed())
  }
  pub fn elapsed(&self) -> super::super::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::DurationView::default())
  }
  pub fn elapsed_mut(&mut self) -> super::super::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_elapsed(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // created_at: optional message palm.lavender.v1.Timestamp
  pub fn has_created_at(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_created_at(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn created_at_opt(&self) -> ::protobuf::Optional<super::super::TimestampView<'_>> {
        ::protobuf::Optional::new(self.created_at(), self.has_created_at())
  }
  pub fn created_at(&self) -> super::super::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::TimestampView::default())
  }
  pub fn created_at_mut(&mut self) -> super::super::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_created_at(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

}  // impl Item

impl ::std::ops::Drop for Item {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Item {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Item {
  type Proxied = Self;
  fn as_view(&self) -> ItemView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Item {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ItemMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Item {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::http_request::palm__lavender__v1__HttpRequest__Item_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$)P1X1Xd33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::http_request::palm__lavender__v1__HttpRequest__Item_msg_init.0, &[<super::super::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::http_request::palm__lavender__v1__HttpRequest__Item_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Item {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Item {
  type Msg = Item;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Item> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Item {
  type Msg = Item;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Item> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ItemMut<'_> {
  type Msg = Item;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Item> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ItemMut<'_> {
  type Msg = Item;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Item> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ItemView<'_> {
  type Msg = Item;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Item> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ItemMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod http_request


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__lavender__v1__PostgreSql_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PostgreSql {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PostgreSql>
}

impl ::protobuf::Message for PostgreSql {}

impl ::std::default::Default for PostgreSql {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PostgreSql {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PostgreSql` is `Sync` because it does not implement interior mutability.
//    Neither does `PostgreSqlMut`.
unsafe impl Sync for PostgreSql {}

// SAFETY:
// - `PostgreSql` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PostgreSql {}

impl ::protobuf::Proxied for PostgreSql {
  type View<'msg> = PostgreSqlView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PostgreSql {}

impl ::protobuf::MutProxied for PostgreSql {
  type Mut<'msg> = PostgreSqlMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PostgreSqlView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PostgreSql>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PostgreSqlView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PostgreSqlView<'msg> {
  type Message = PostgreSql;
}

impl ::std::fmt::Debug for PostgreSqlView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PostgreSqlView<'_> {
  fn default() -> PostgreSqlView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PostgreSql>> for PostgreSqlView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PostgreSql>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PostgreSqlView<'msg> {

  pub fn to_owned(&self) -> PostgreSql {
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

}

// SAFETY:
// - `PostgreSqlView` is `Sync` because it does not support mutation.
unsafe impl Sync for PostgreSqlView<'_> {}

// SAFETY:
// - `PostgreSqlView` is `Send` because while its alive a `PostgreSqlMut` cannot.
// - `PostgreSqlView` does not use thread-local data.
unsafe impl Send for PostgreSqlView<'_> {}

impl<'msg> ::protobuf::AsView for PostgreSqlView<'msg> {
  type Proxied = PostgreSql;
  fn as_view(&self) -> ::protobuf::View<'msg, PostgreSql> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PostgreSqlView<'msg> {
  fn into_view<'shorter>(self) -> PostgreSqlView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PostgreSql> for PostgreSqlView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PostgreSql {
    let mut dst = PostgreSql::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PostgreSql> for PostgreSqlMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PostgreSql {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for PostgreSql {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PostgreSqlView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PostgreSqlMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PostgreSqlMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PostgreSql>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PostgreSqlMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PostgreSqlMut<'msg> {
  type Message = PostgreSql;
}

impl ::std::fmt::Debug for PostgreSqlMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PostgreSql>> for PostgreSqlMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PostgreSql>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PostgreSqlMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PostgreSql> {
    self.inner
  }

  pub fn to_owned(&self) -> PostgreSql {
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

}

// SAFETY:
// - `PostgreSqlMut` does not perform any shared mutation.
unsafe impl Send for PostgreSqlMut<'_> {}

// SAFETY:
// - `PostgreSqlMut` does not perform any shared mutation.
unsafe impl Sync for PostgreSqlMut<'_> {}

impl<'msg> ::protobuf::AsView for PostgreSqlMut<'msg> {
  type Proxied = PostgreSql;
  fn as_view(&self) -> ::protobuf::View<'_, PostgreSql> {
    PostgreSqlView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PostgreSqlMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PostgreSql>
  where
      'msg: 'shorter {
    PostgreSqlView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for PostgreSqlMut<'msg> {
  type MutProxied = PostgreSql;
  fn as_mut(&mut self) -> PostgreSqlMut<'msg> {
    PostgreSqlMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PostgreSqlMut<'msg> {
  fn into_mut<'shorter>(self) -> PostgreSqlMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PostgreSql {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PostgreSql> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PostgreSqlView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PostgreSqlMut<'_> {
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

}  // impl PostgreSql

impl ::std::ops::Drop for PostgreSql {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PostgreSql {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PostgreSql {
  type Proxied = Self;
  fn as_view(&self) -> PostgreSqlView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PostgreSql {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PostgreSqlMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PostgreSql {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__lavender__v1__PostgreSql_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__lavender__v1__PostgreSql_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__lavender__v1__PostgreSql_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PostgreSql {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PostgreSql {
  type Msg = PostgreSql;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PostgreSql> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PostgreSql {
  type Msg = PostgreSql;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PostgreSql> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PostgreSqlMut<'_> {
  type Msg = PostgreSql;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PostgreSql> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PostgreSqlMut<'_> {
  type Msg = PostgreSql;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PostgreSql> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PostgreSqlView<'_> {
  type Msg = PostgreSql;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PostgreSql> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PostgreSqlMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__lavender__v1__MySql_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MySql {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MySql>
}

impl ::protobuf::Message for MySql {}

impl ::std::default::Default for MySql {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MySql {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MySql` is `Sync` because it does not implement interior mutability.
//    Neither does `MySqlMut`.
unsafe impl Sync for MySql {}

// SAFETY:
// - `MySql` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for MySql {}

impl ::protobuf::Proxied for MySql {
  type View<'msg> = MySqlView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MySql {}

impl ::protobuf::MutProxied for MySql {
  type Mut<'msg> = MySqlMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MySqlView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MySql>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MySqlView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MySqlView<'msg> {
  type Message = MySql;
}

impl ::std::fmt::Debug for MySqlView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MySqlView<'_> {
  fn default() -> MySqlView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MySql>> for MySqlView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MySql>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MySqlView<'msg> {

  pub fn to_owned(&self) -> MySql {
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

}

// SAFETY:
// - `MySqlView` is `Sync` because it does not support mutation.
unsafe impl Sync for MySqlView<'_> {}

// SAFETY:
// - `MySqlView` is `Send` because while its alive a `MySqlMut` cannot.
// - `MySqlView` does not use thread-local data.
unsafe impl Send for MySqlView<'_> {}

impl<'msg> ::protobuf::AsView for MySqlView<'msg> {
  type Proxied = MySql;
  fn as_view(&self) -> ::protobuf::View<'msg, MySql> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MySqlView<'msg> {
  fn into_view<'shorter>(self) -> MySqlView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MySql> for MySqlView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MySql {
    let mut dst = MySql::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MySql> for MySqlMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MySql {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for MySql {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for MySqlView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for MySqlMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MySqlMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MySql>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MySqlMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MySqlMut<'msg> {
  type Message = MySql;
}

impl ::std::fmt::Debug for MySqlMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MySql>> for MySqlMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MySql>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MySqlMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MySql> {
    self.inner
  }

  pub fn to_owned(&self) -> MySql {
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

}

// SAFETY:
// - `MySqlMut` does not perform any shared mutation.
unsafe impl Send for MySqlMut<'_> {}

// SAFETY:
// - `MySqlMut` does not perform any shared mutation.
unsafe impl Sync for MySqlMut<'_> {}

impl<'msg> ::protobuf::AsView for MySqlMut<'msg> {
  type Proxied = MySql;
  fn as_view(&self) -> ::protobuf::View<'_, MySql> {
    MySqlView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MySqlMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MySql>
  where
      'msg: 'shorter {
    MySqlView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for MySqlMut<'msg> {
  type MutProxied = MySql;
  fn as_mut(&mut self) -> MySqlMut<'msg> {
    MySqlMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MySqlMut<'msg> {
  fn into_mut<'shorter>(self) -> MySqlMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MySql {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MySql> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MySqlView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MySqlMut<'_> {
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

}  // impl MySql

impl ::std::ops::Drop for MySql {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MySql {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MySql {
  type Proxied = Self;
  fn as_view(&self) -> MySqlView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MySql {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MySqlMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MySql {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__lavender__v1__MySql_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__lavender__v1__MySql_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__lavender__v1__MySql_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MySql {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MySql {
  type Msg = MySql;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MySql> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MySql {
  type Msg = MySql;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MySql> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MySqlMut<'_> {
  type Msg = MySql;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MySql> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MySqlMut<'_> {
  type Msg = MySql;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MySql> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MySqlView<'_> {
  type Msg = MySql;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MySql> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MySqlMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__lavender__v1__Redis_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Redis {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Redis>
}

impl ::protobuf::Message for Redis {}

impl ::std::default::Default for Redis {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Redis {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Redis` is `Sync` because it does not implement interior mutability.
//    Neither does `RedisMut`.
unsafe impl Sync for Redis {}

// SAFETY:
// - `Redis` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Redis {}

impl ::protobuf::Proxied for Redis {
  type View<'msg> = RedisView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Redis {}

impl ::protobuf::MutProxied for Redis {
  type Mut<'msg> = RedisMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RedisView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Redis>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RedisView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RedisView<'msg> {
  type Message = Redis;
}

impl ::std::fmt::Debug for RedisView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RedisView<'_> {
  fn default() -> RedisView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Redis>> for RedisView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Redis>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RedisView<'msg> {

  pub fn to_owned(&self) -> Redis {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // info: optional string
  pub fn info(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // cluster: optional string
  pub fn has_cluster(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn cluster_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.cluster(), self.has_cluster())
  }
  pub fn cluster(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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
// - `RedisView` is `Sync` because it does not support mutation.
unsafe impl Sync for RedisView<'_> {}

// SAFETY:
// - `RedisView` is `Send` because while its alive a `RedisMut` cannot.
// - `RedisView` does not use thread-local data.
unsafe impl Send for RedisView<'_> {}

impl<'msg> ::protobuf::AsView for RedisView<'msg> {
  type Proxied = Redis;
  fn as_view(&self) -> ::protobuf::View<'msg, Redis> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RedisView<'msg> {
  fn into_view<'shorter>(self) -> RedisView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Redis> for RedisView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Redis {
    let mut dst = Redis::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Redis> for RedisMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Redis {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Redis {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for RedisView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for RedisMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RedisMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Redis>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RedisMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RedisMut<'msg> {
  type Message = Redis;
}

impl ::std::fmt::Debug for RedisMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Redis>> for RedisMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Redis>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RedisMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Redis> {
    self.inner
  }

  pub fn to_owned(&self) -> Redis {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // info: optional string
  pub fn info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // cluster: optional string
  pub fn has_cluster(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_cluster(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn cluster_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.cluster(), self.has_cluster())
  }
  pub fn cluster(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_cluster(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `RedisMut` does not perform any shared mutation.
unsafe impl Send for RedisMut<'_> {}

// SAFETY:
// - `RedisMut` does not perform any shared mutation.
unsafe impl Sync for RedisMut<'_> {}

impl<'msg> ::protobuf::AsView for RedisMut<'msg> {
  type Proxied = Redis;
  fn as_view(&self) -> ::protobuf::View<'_, Redis> {
    RedisView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RedisMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Redis>
  where
      'msg: 'shorter {
    RedisView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for RedisMut<'msg> {
  type MutProxied = Redis;
  fn as_mut(&mut self) -> RedisMut<'msg> {
    RedisMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RedisMut<'msg> {
  fn into_mut<'shorter>(self) -> RedisMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Redis {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Redis> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RedisView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RedisMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // info: optional string
  pub fn info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // cluster: optional string
  pub fn has_cluster(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_cluster(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn cluster_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.cluster(), self.has_cluster())
  }
  pub fn cluster(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_cluster(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl Redis

impl ::std::ops::Drop for Redis {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Redis {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Redis {
  type Proxied = Self;
  fn as_view(&self) -> RedisView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Redis {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RedisMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Redis {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__lavender__v1__Redis_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__lavender__v1__Redis_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__lavender__v1__Redis_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Redis {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Redis {
  type Msg = Redis;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Redis> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Redis {
  type Msg = Redis;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Redis> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RedisMut<'_> {
  type Msg = Redis;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Redis> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RedisMut<'_> {
  type Msg = Redis;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Redis> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RedisView<'_> {
  type Msg = Redis;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Redis> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RedisMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__lavender__v1__Snmp_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Snmp {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Snmp>
}

impl ::protobuf::Message for Snmp {}

impl ::std::default::Default for Snmp {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Snmp {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Snmp` is `Sync` because it does not implement interior mutability.
//    Neither does `SnmpMut`.
unsafe impl Sync for Snmp {}

// SAFETY:
// - `Snmp` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Snmp {}

impl ::protobuf::Proxied for Snmp {
  type View<'msg> = SnmpView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Snmp {}

impl ::protobuf::MutProxied for Snmp {
  type Mut<'msg> = SnmpMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SnmpView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Snmp>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SnmpView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SnmpView<'msg> {
  type Message = Snmp;
}

impl ::std::fmt::Debug for SnmpView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SnmpView<'_> {
  fn default() -> SnmpView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Snmp>> for SnmpView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Snmp>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SnmpView<'msg> {

  pub fn to_owned(&self) -> Snmp {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // items: repeated message palm.lavender.v1.Snmp.Item
  pub fn items(self) -> ::protobuf::RepeatedView<'msg, super::snmp::Item> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::snmp::Item>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `SnmpView` is `Sync` because it does not support mutation.
unsafe impl Sync for SnmpView<'_> {}

// SAFETY:
// - `SnmpView` is `Send` because while its alive a `SnmpMut` cannot.
// - `SnmpView` does not use thread-local data.
unsafe impl Send for SnmpView<'_> {}

impl<'msg> ::protobuf::AsView for SnmpView<'msg> {
  type Proxied = Snmp;
  fn as_view(&self) -> ::protobuf::View<'msg, Snmp> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SnmpView<'msg> {
  fn into_view<'shorter>(self) -> SnmpView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Snmp> for SnmpView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Snmp {
    let mut dst = Snmp::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Snmp> for SnmpMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Snmp {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Snmp {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SnmpView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SnmpMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SnmpMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Snmp>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SnmpMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SnmpMut<'msg> {
  type Message = Snmp;
}

impl ::std::fmt::Debug for SnmpMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Snmp>> for SnmpMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Snmp>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SnmpMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Snmp> {
    self.inner
  }

  pub fn to_owned(&self) -> Snmp {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // items: repeated message palm.lavender.v1.Snmp.Item
  pub fn items(&self) -> ::protobuf::RepeatedView<'_, super::snmp::Item> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::snmp::Item>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::snmp::Item> {
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
  pub fn set_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::snmp::Item>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `SnmpMut` does not perform any shared mutation.
unsafe impl Send for SnmpMut<'_> {}

// SAFETY:
// - `SnmpMut` does not perform any shared mutation.
unsafe impl Sync for SnmpMut<'_> {}

impl<'msg> ::protobuf::AsView for SnmpMut<'msg> {
  type Proxied = Snmp;
  fn as_view(&self) -> ::protobuf::View<'_, Snmp> {
    SnmpView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SnmpMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Snmp>
  where
      'msg: 'shorter {
    SnmpView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for SnmpMut<'msg> {
  type MutProxied = Snmp;
  fn as_mut(&mut self) -> SnmpMut<'msg> {
    SnmpMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SnmpMut<'msg> {
  fn into_mut<'shorter>(self) -> SnmpMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Snmp {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Snmp> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SnmpView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SnmpMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // items: repeated message palm.lavender.v1.Snmp.Item
  pub fn items(&self) -> ::protobuf::RepeatedView<'_, super::snmp::Item> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::snmp::Item>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::snmp::Item> {
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
  pub fn set_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::snmp::Item>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl Snmp

impl ::std::ops::Drop for Snmp {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Snmp {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Snmp {
  type Proxied = Self;
  fn as_view(&self) -> SnmpView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Snmp {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SnmpMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Snmp {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__lavender__v1__Snmp_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__lavender__v1__Snmp_msg_init.0, &[<super::snmp::Item as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__lavender__v1__Snmp_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Snmp {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Snmp {
  type Msg = Snmp;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Snmp> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Snmp {
  type Msg = Snmp;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Snmp> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SnmpMut<'_> {
  type Msg = Snmp;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Snmp> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SnmpMut<'_> {
  type Msg = Snmp;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Snmp> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SnmpView<'_> {
  type Msg = Snmp;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Snmp> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SnmpMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod snmp {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__lavender__v1__Snmp__Item_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Item {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Item>
}

impl ::protobuf::Message for Item {}

impl ::std::default::Default for Item {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Item {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Item` is `Sync` because it does not implement interior mutability.
//    Neither does `ItemMut`.
unsafe impl Sync for Item {}

// SAFETY:
// - `Item` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Item {}

impl ::protobuf::Proxied for Item {
  type View<'msg> = ItemView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Item {}

impl ::protobuf::MutProxied for Item {
  type Mut<'msg> = ItemMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ItemView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Item>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ItemView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ItemView<'msg> {
  type Message = Item;
}

impl ::std::fmt::Debug for ItemView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ItemView<'_> {
  fn default() -> ItemView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Item>> for ItemView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Item>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ItemView<'msg> {

  pub fn to_owned(&self) -> Item {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // oid: optional string
  pub fn oid(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // i: optional int64
  pub fn has_i(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn i_opt(self) -> ::protobuf::Optional<i64> {
        ::protobuf::Optional::new(self.i(), self.has_i())
  }
  pub fn i(self) -> i64 {
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

  // s: optional string
  pub fn has_s(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn s_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.s(), self.has_s())
  }
  pub fn s(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // d: optional double
  pub fn has_d(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn d_opt(self) -> ::protobuf::Optional<f64> {
        ::protobuf::Optional::new(self.d(), self.has_d())
  }
  pub fn d(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        3, (0f64).into()
      ).try_into().unwrap()
    }
  }

  pub fn value(self) -> super::super::snmp::item::ValueOneof<'msg> {
    match self.value_case() {
      super::super::snmp::item::ValueCase::I =>
          super::super::snmp::item::ValueOneof::I(self.i()),
      super::super::snmp::item::ValueCase::S =>
          super::super::snmp::item::ValueOneof::S(self.s()),
      super::super::snmp::item::ValueCase::D =>
          super::super::snmp::item::ValueOneof::D(self.d()),
      _ => super::super::snmp::item::ValueOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn value_case(self) -> super::super::snmp::item::ValueCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::snmp::item::ValueCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ItemView` is `Sync` because it does not support mutation.
unsafe impl Sync for ItemView<'_> {}

// SAFETY:
// - `ItemView` is `Send` because while its alive a `ItemMut` cannot.
// - `ItemView` does not use thread-local data.
unsafe impl Send for ItemView<'_> {}

impl<'msg> ::protobuf::AsView for ItemView<'msg> {
  type Proxied = Item;
  fn as_view(&self) -> ::protobuf::View<'msg, Item> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ItemView<'msg> {
  fn into_view<'shorter>(self) -> ItemView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Item> for ItemView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Item {
    let mut dst = Item::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Item> for ItemMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Item {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Item {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ItemView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ItemMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ItemMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Item>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ItemMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ItemMut<'msg> {
  type Message = Item;
}

impl ::std::fmt::Debug for ItemMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Item>> for ItemMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Item>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ItemMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Item> {
    self.inner
  }

  pub fn to_owned(&self) -> Item {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // oid: optional string
  pub fn oid(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_oid(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // i: optional int64
  pub fn has_i(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_i(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn i_opt(&self) -> ::protobuf::Optional<i64> {
        ::protobuf::Optional::new(self.i(), self.has_i())
  }
  pub fn i(&self) -> i64 {
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
  pub fn set_i(&mut self, val: i64) {
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

  // s: optional string
  pub fn has_s(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_s(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn s_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.s(), self.has_s())
  }
  pub fn s(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_s(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // d: optional double
  pub fn has_d(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_d(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn d_opt(&self) -> ::protobuf::Optional<f64> {
        ::protobuf::Optional::new(self.d(), self.has_d())
  }
  pub fn d(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        3, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_d(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        3, val.into()
      )
    }
  }

  pub fn value(&self) -> super::super::snmp::item::ValueOneof<'_> {
    match &self.value_case() {
      super::super::snmp::item::ValueCase::I =>
          super::super::snmp::item::ValueOneof::I(self.i()),
      super::super::snmp::item::ValueCase::S =>
          super::super::snmp::item::ValueOneof::S(self.s()),
      super::super::snmp::item::ValueCase::D =>
          super::super::snmp::item::ValueOneof::D(self.d()),
      _ => super::super::snmp::item::ValueOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn value_case(&self) -> super::super::snmp::item::ValueCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::snmp::item::ValueCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ItemMut` does not perform any shared mutation.
unsafe impl Send for ItemMut<'_> {}

// SAFETY:
// - `ItemMut` does not perform any shared mutation.
unsafe impl Sync for ItemMut<'_> {}

impl<'msg> ::protobuf::AsView for ItemMut<'msg> {
  type Proxied = Item;
  fn as_view(&self) -> ::protobuf::View<'_, Item> {
    ItemView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ItemMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Item>
  where
      'msg: 'shorter {
    ItemView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ItemMut<'msg> {
  type MutProxied = Item;
  fn as_mut(&mut self) -> ItemMut<'msg> {
    ItemMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ItemMut<'msg> {
  fn into_mut<'shorter>(self) -> ItemMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Item {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Item> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ItemView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ItemMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // oid: optional string
  pub fn oid(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_oid(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // i: optional int64
  pub fn has_i(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_i(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn i_opt(&self) -> ::protobuf::Optional<i64> {
        ::protobuf::Optional::new(self.i(), self.has_i())
  }
  pub fn i(&self) -> i64 {
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
  pub fn set_i(&mut self, val: i64) {
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

  // s: optional string
  pub fn has_s(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_s(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn s_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.s(), self.has_s())
  }
  pub fn s(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_s(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // d: optional double
  pub fn has_d(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_d(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn d_opt(&self) -> ::protobuf::Optional<f64> {
        ::protobuf::Optional::new(self.d(), self.has_d())
  }
  pub fn d(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        3, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_d(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        3, val.into()
      )
    }
  }

  pub fn value(&self) -> super::super::snmp::item::ValueOneof<'_> {
    match &self.value_case() {
      super::super::snmp::item::ValueCase::I =>
          super::super::snmp::item::ValueOneof::I(self.i()),
      super::super::snmp::item::ValueCase::S =>
          super::super::snmp::item::ValueOneof::S(self.s()),
      super::super::snmp::item::ValueCase::D =>
          super::super::snmp::item::ValueOneof::D(self.d()),
      _ => super::super::snmp::item::ValueOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn value_case(&self) -> super::super::snmp::item::ValueCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::snmp::item::ValueCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Item

impl ::std::ops::Drop for Item {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Item {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Item {
  type Proxied = Self;
  fn as_view(&self) -> ItemView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Item {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ItemMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Item {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::snmp::palm__lavender__v1__Snmp__Item_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1Xi+1T ^-|.|/");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::snmp::palm__lavender__v1__Snmp__Item_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::snmp::palm__lavender__v1__Snmp__Item_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Item {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Item {
  type Msg = Item;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Item> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Item {
  type Msg = Item;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Item> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ItemMut<'_> {
  type Msg = Item;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Item> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ItemMut<'_> {
  type Msg = Item;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Item> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ItemView<'_> {
  type Msg = Item;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Item> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ItemMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod item {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ValueOneof<'msg> {
  I(i64) = 11,
  S(&'msg ::protobuf::ProtoStr) = 12,
  D(f64) = 13,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ValueCase {
  I = 11,
  S = 12,
  D = 13,

  not_set = 0
}

impl ValueCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ValueCase> {
    match v {
      0 => Some(ValueCase::not_set),
      11 => Some(ValueCase::I),
      12 => Some(ValueCase::S),
      13 => Some(ValueCase::D),
      _ => None
    }
  }
}
}  // pub mod item


}  // pub mod snmp


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__lavender__v1__OpenSearch_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct OpenSearch {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<OpenSearch>
}

impl ::protobuf::Message for OpenSearch {}

impl ::std::default::Default for OpenSearch {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for OpenSearch {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `OpenSearch` is `Sync` because it does not implement interior mutability.
//    Neither does `OpenSearchMut`.
unsafe impl Sync for OpenSearch {}

// SAFETY:
// - `OpenSearch` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for OpenSearch {}

impl ::protobuf::Proxied for OpenSearch {
  type View<'msg> = OpenSearchView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for OpenSearch {}

impl ::protobuf::MutProxied for OpenSearch {
  type Mut<'msg> = OpenSearchMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct OpenSearchView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OpenSearch>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OpenSearchView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for OpenSearchView<'msg> {
  type Message = OpenSearch;
}

impl ::std::fmt::Debug for OpenSearchView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for OpenSearchView<'_> {
  fn default() -> OpenSearchView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, OpenSearch>> for OpenSearchView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OpenSearch>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OpenSearchView<'msg> {

  pub fn to_owned(&self) -> OpenSearch {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // cluster_name: optional string
  pub fn cluster_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // status: optional string
  pub fn status(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // timed_out: optional bool
  pub fn timed_out(self) -> bool {
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

  // number_of_nodes: optional uint32
  pub fn number_of_nodes(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        3, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // number_of_data_nodes: optional uint32
  pub fn number_of_data_nodes(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        4, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // discovered_master: optional bool
  pub fn discovered_master(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }

  // discovered_cluster_manager: optional bool
  pub fn discovered_cluster_manager(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }

  // active_primary_shards: optional uint32
  pub fn active_primary_shards(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        7, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // active_shards: optional uint32
  pub fn active_shards(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        8, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // relocating_shards: optional uint32
  pub fn relocating_shards(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        9, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // initializing_shards: optional uint32
  pub fn initializing_shards(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        10, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // unassigned_shards: optional uint32
  pub fn unassigned_shards(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        11, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // delayed_unassigned_shards: optional uint32
  pub fn delayed_unassigned_shards(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        12, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // number_of_pending_tasks: optional uint32
  pub fn number_of_pending_tasks(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        13, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // number_of_in_flight_fetch: optional uint32
  pub fn number_of_in_flight_fetch(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        14, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // task_max_waiting_in_queue_millis: optional uint32
  pub fn task_max_waiting_in_queue_millis(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        15, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // active_shards_percent_as_number: optional float
  pub fn active_shards_percent_as_number(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        16, (0f32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `OpenSearchView` is `Sync` because it does not support mutation.
unsafe impl Sync for OpenSearchView<'_> {}

// SAFETY:
// - `OpenSearchView` is `Send` because while its alive a `OpenSearchMut` cannot.
// - `OpenSearchView` does not use thread-local data.
unsafe impl Send for OpenSearchView<'_> {}

impl<'msg> ::protobuf::AsView for OpenSearchView<'msg> {
  type Proxied = OpenSearch;
  fn as_view(&self) -> ::protobuf::View<'msg, OpenSearch> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OpenSearchView<'msg> {
  fn into_view<'shorter>(self) -> OpenSearchView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<OpenSearch> for OpenSearchView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OpenSearch {
    let mut dst = OpenSearch::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<OpenSearch> for OpenSearchMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OpenSearch {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for OpenSearch {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for OpenSearchView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for OpenSearchMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct OpenSearchMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OpenSearch>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OpenSearchMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for OpenSearchMut<'msg> {
  type Message = OpenSearch;
}

impl ::std::fmt::Debug for OpenSearchMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, OpenSearch>> for OpenSearchMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OpenSearch>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OpenSearchMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, OpenSearch> {
    self.inner
  }

  pub fn to_owned(&self) -> OpenSearch {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // cluster_name: optional string
  pub fn cluster_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_cluster_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // status: optional string
  pub fn status(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_status(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // timed_out: optional bool
  pub fn timed_out(&self) -> bool {
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
  pub fn set_timed_out(&mut self, val: bool) {
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

  // number_of_nodes: optional uint32
  pub fn number_of_nodes(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        3, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_number_of_nodes(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        3, val.into()
      )
    }
  }

  // number_of_data_nodes: optional uint32
  pub fn number_of_data_nodes(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        4, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_number_of_data_nodes(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        4, val.into()
      )
    }
  }

  // discovered_master: optional bool
  pub fn discovered_master(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_discovered_master(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        5, val.into()
      )
    }
  }

  // discovered_cluster_manager: optional bool
  pub fn discovered_cluster_manager(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_discovered_cluster_manager(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        6, val.into()
      )
    }
  }

  // active_primary_shards: optional uint32
  pub fn active_primary_shards(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        7, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_active_primary_shards(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        7, val.into()
      )
    }
  }

  // active_shards: optional uint32
  pub fn active_shards(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        8, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_active_shards(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        8, val.into()
      )
    }
  }

  // relocating_shards: optional uint32
  pub fn relocating_shards(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        9, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_relocating_shards(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        9, val.into()
      )
    }
  }

  // initializing_shards: optional uint32
  pub fn initializing_shards(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        10, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_initializing_shards(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        10, val.into()
      )
    }
  }

  // unassigned_shards: optional uint32
  pub fn unassigned_shards(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        11, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_unassigned_shards(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        11, val.into()
      )
    }
  }

  // delayed_unassigned_shards: optional uint32
  pub fn delayed_unassigned_shards(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        12, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_delayed_unassigned_shards(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        12, val.into()
      )
    }
  }

  // number_of_pending_tasks: optional uint32
  pub fn number_of_pending_tasks(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        13, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_number_of_pending_tasks(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        13, val.into()
      )
    }
  }

  // number_of_in_flight_fetch: optional uint32
  pub fn number_of_in_flight_fetch(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        14, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_number_of_in_flight_fetch(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        14, val.into()
      )
    }
  }

  // task_max_waiting_in_queue_millis: optional uint32
  pub fn task_max_waiting_in_queue_millis(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        15, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_task_max_waiting_in_queue_millis(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        15, val.into()
      )
    }
  }

  // active_shards_percent_as_number: optional float
  pub fn active_shards_percent_as_number(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        16, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_active_shards_percent_as_number(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        16, val.into()
      )
    }
  }

}

// SAFETY:
// - `OpenSearchMut` does not perform any shared mutation.
unsafe impl Send for OpenSearchMut<'_> {}

// SAFETY:
// - `OpenSearchMut` does not perform any shared mutation.
unsafe impl Sync for OpenSearchMut<'_> {}

impl<'msg> ::protobuf::AsView for OpenSearchMut<'msg> {
  type Proxied = OpenSearch;
  fn as_view(&self) -> ::protobuf::View<'_, OpenSearch> {
    OpenSearchView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OpenSearchMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, OpenSearch>
  where
      'msg: 'shorter {
    OpenSearchView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for OpenSearchMut<'msg> {
  type MutProxied = OpenSearch;
  fn as_mut(&mut self) -> OpenSearchMut<'msg> {
    OpenSearchMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for OpenSearchMut<'msg> {
  fn into_mut<'shorter>(self) -> OpenSearchMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl OpenSearch {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, OpenSearch> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> OpenSearchView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> OpenSearchMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // cluster_name: optional string
  pub fn cluster_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_cluster_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // status: optional string
  pub fn status(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_status(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // timed_out: optional bool
  pub fn timed_out(&self) -> bool {
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
  pub fn set_timed_out(&mut self, val: bool) {
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

  // number_of_nodes: optional uint32
  pub fn number_of_nodes(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        3, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_number_of_nodes(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        3, val.into()
      )
    }
  }

  // number_of_data_nodes: optional uint32
  pub fn number_of_data_nodes(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        4, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_number_of_data_nodes(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        4, val.into()
      )
    }
  }

  // discovered_master: optional bool
  pub fn discovered_master(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_discovered_master(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        5, val.into()
      )
    }
  }

  // discovered_cluster_manager: optional bool
  pub fn discovered_cluster_manager(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_discovered_cluster_manager(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        6, val.into()
      )
    }
  }

  // active_primary_shards: optional uint32
  pub fn active_primary_shards(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        7, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_active_primary_shards(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        7, val.into()
      )
    }
  }

  // active_shards: optional uint32
  pub fn active_shards(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        8, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_active_shards(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        8, val.into()
      )
    }
  }

  // relocating_shards: optional uint32
  pub fn relocating_shards(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        9, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_relocating_shards(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        9, val.into()
      )
    }
  }

  // initializing_shards: optional uint32
  pub fn initializing_shards(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        10, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_initializing_shards(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        10, val.into()
      )
    }
  }

  // unassigned_shards: optional uint32
  pub fn unassigned_shards(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        11, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_unassigned_shards(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        11, val.into()
      )
    }
  }

  // delayed_unassigned_shards: optional uint32
  pub fn delayed_unassigned_shards(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        12, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_delayed_unassigned_shards(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        12, val.into()
      )
    }
  }

  // number_of_pending_tasks: optional uint32
  pub fn number_of_pending_tasks(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        13, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_number_of_pending_tasks(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        13, val.into()
      )
    }
  }

  // number_of_in_flight_fetch: optional uint32
  pub fn number_of_in_flight_fetch(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        14, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_number_of_in_flight_fetch(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        14, val.into()
      )
    }
  }

  // task_max_waiting_in_queue_millis: optional uint32
  pub fn task_max_waiting_in_queue_millis(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        15, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_task_max_waiting_in_queue_millis(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        15, val.into()
      )
    }
  }

  // active_shards_percent_as_number: optional float
  pub fn active_shards_percent_as_number(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        16, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_active_shards_percent_as_number(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        16, val.into()
      )
    }
  }

}  // impl OpenSearch

impl ::std::ops::Drop for OpenSearch {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for OpenSearch {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for OpenSearch {
  type Proxied = Self;
  fn as_view(&self) -> OpenSearchView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for OpenSearch {
  type MutProxied = Self;
  fn as_mut(&mut self) -> OpenSearchMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for OpenSearch {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__lavender__v1__OpenSearch_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X/P)P)P/P/P)P)P)P)P)P)P)P)P)P!P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__lavender__v1__OpenSearch_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__lavender__v1__OpenSearch_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OpenSearch {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OpenSearch {
  type Msg = OpenSearch;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OpenSearch> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OpenSearch {
  type Msg = OpenSearch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OpenSearch> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OpenSearchMut<'_> {
  type Msg = OpenSearch;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OpenSearch> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OpenSearchMut<'_> {
  type Msg = OpenSearch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OpenSearch> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OpenSearchView<'_> {
  type Msg = OpenSearch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OpenSearch> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OpenSearchMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



