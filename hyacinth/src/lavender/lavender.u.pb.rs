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

  // items: repeated message palm.lavender.v1.SystemdRequest.Item
  pub fn items(self) -> ::protobuf::RepeatedView<'msg, super::systemd_request::Item> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::systemd_request::Item>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
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

  // items: repeated message palm.lavender.v1.SystemdRequest.Item
  pub fn items(&self) -> ::protobuf::RepeatedView<'_, super::systemd_request::Item> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::systemd_request::Item>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::systemd_request::Item> {
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
  pub fn set_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::systemd_request::Item>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
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

  // items: repeated message palm.lavender.v1.SystemdRequest.Item
  pub fn items(&self) -> ::protobuf::RepeatedView<'_, super::systemd_request::Item> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::systemd_request::Item>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::systemd_request::Item> {
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
  pub fn set_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::systemd_request::Item>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
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
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__lavender__v1__SystemdRequest_msg_init.0, &[<super::systemd_request::Item as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
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

pub mod systemd_request {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__lavender__v1__SystemdRequest__Item_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
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

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // pid: optional uint32
  pub fn pid(self) -> u32 {
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

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // pid: optional uint32
  pub fn pid(&self) -> u32 {
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
  pub fn set_pid(&mut self, val: u32) {
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

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // pid: optional uint32
  pub fn pid(&self) -> u32 {
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
  pub fn set_pid(&mut self, val: u32) {
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
        super::super::systemd_request::palm__lavender__v1__SystemdRequest__Item_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X)P1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::systemd_request::palm__lavender__v1__SystemdRequest__Item_msg_init.0, &[<super::super::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::systemd_request::palm__lavender__v1__SystemdRequest__Item_msg_init.0)
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



}  // pub mod systemd_request


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__lavender__v1__Http_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Http {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Http>
}

impl ::protobuf::Message for Http {}

impl ::std::default::Default for Http {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Http {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Http` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpMut`.
unsafe impl Sync for Http {}

// SAFETY:
// - `Http` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Http {}

impl ::protobuf::Proxied for Http {
  type View<'msg> = HttpView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Http {}

impl ::protobuf::MutProxied for Http {
  type Mut<'msg> = HttpMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Http>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpView<'msg> {
  type Message = Http;
}

impl ::std::fmt::Debug for HttpView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpView<'_> {
  fn default() -> HttpView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Http>> for HttpView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Http>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpView<'msg> {

  pub fn to_owned(&self) -> Http {
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

  // response_body: optional string
  pub fn response_body(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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
// - `HttpView` is `Sync` because it does not support mutation.
unsafe impl Sync for HttpView<'_> {}

// SAFETY:
// - `HttpView` is `Send` because while its alive a `HttpMut` cannot.
// - `HttpView` does not use thread-local data.
unsafe impl Send for HttpView<'_> {}

impl<'msg> ::protobuf::AsView for HttpView<'msg> {
  type Proxied = Http;
  fn as_view(&self) -> ::protobuf::View<'msg, Http> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpView<'msg> {
  fn into_view<'shorter>(self) -> HttpView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Http> for HttpView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Http {
    let mut dst = Http::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Http> for HttpMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Http {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Http {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for HttpView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for HttpMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Http>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpMut<'msg> {
  type Message = Http;
}

impl ::std::fmt::Debug for HttpMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Http>> for HttpMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Http>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Http> {
    self.inner
  }

  pub fn to_owned(&self) -> Http {
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

  // response_body: optional string
  pub fn response_body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_response_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `HttpMut` does not perform any shared mutation.
unsafe impl Send for HttpMut<'_> {}

// SAFETY:
// - `HttpMut` does not perform any shared mutation.
unsafe impl Sync for HttpMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpMut<'msg> {
  type Proxied = Http;
  fn as_view(&self) -> ::protobuf::View<'_, Http> {
    HttpView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Http>
  where
      'msg: 'shorter {
    HttpView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for HttpMut<'msg> {
  type MutProxied = Http;
  fn as_mut(&mut self) -> HttpMut<'msg> {
    HttpMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Http {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Http> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpMut<'_> {
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

  // response_body: optional string
  pub fn response_body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_response_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl Http

impl ::std::ops::Drop for Http {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Http {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Http {
  type Proxied = Self;
  fn as_view(&self) -> HttpView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Http {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Http {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__lavender__v1__Http_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$)P1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__lavender__v1__Http_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__lavender__v1__Http_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Http {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Http {
  type Msg = Http;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Http {
  type Msg = Http;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpMut<'_> {
  type Msg = Http;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpMut<'_> {
  type Msg = Http;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpView<'_> {
  type Msg = Http;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



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



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__lavender__v1__ReportRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ReportRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ReportRequest>
}

impl ::protobuf::Message for ReportRequest {}

impl ::std::default::Default for ReportRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ReportRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ReportRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `ReportRequestMut`.
unsafe impl Sync for ReportRequest {}

// SAFETY:
// - `ReportRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ReportRequest {}

impl ::protobuf::Proxied for ReportRequest {
  type View<'msg> = ReportRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ReportRequest {}

impl ::protobuf::MutProxied for ReportRequest {
  type Mut<'msg> = ReportRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ReportRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ReportRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ReportRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ReportRequestView<'msg> {
  type Message = ReportRequest;
}

impl ::std::fmt::Debug for ReportRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ReportRequestView<'_> {
  fn default() -> ReportRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ReportRequest>> for ReportRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ReportRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ReportRequestView<'msg> {

  pub fn to_owned(&self) -> ReportRequest {
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

  // timestamp: optional message palm.lavender.v1.Timestamp
  pub fn has_timestamp(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn timestamp_opt(self) -> ::protobuf::Optional<super::TimestampView<'msg>> {
        ::protobuf::Optional::new(self.timestamp(), self.has_timestamp())
  }
  pub fn timestamp(self) -> super::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TimestampView::default())
  }

  // items: repeated message palm.lavender.v1.ReportRequest.Item
  pub fn items(self) -> ::protobuf::RepeatedView<'msg, super::report_request::Item> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::report_request::Item>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ReportRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for ReportRequestView<'_> {}

// SAFETY:
// - `ReportRequestView` is `Send` because while its alive a `ReportRequestMut` cannot.
// - `ReportRequestView` does not use thread-local data.
unsafe impl Send for ReportRequestView<'_> {}

impl<'msg> ::protobuf::AsView for ReportRequestView<'msg> {
  type Proxied = ReportRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, ReportRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ReportRequestView<'msg> {
  fn into_view<'shorter>(self) -> ReportRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ReportRequest> for ReportRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ReportRequest {
    let mut dst = ReportRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ReportRequest> for ReportRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ReportRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ReportRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ReportRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ReportRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ReportRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ReportRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ReportRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ReportRequestMut<'msg> {
  type Message = ReportRequest;
}

impl ::std::fmt::Debug for ReportRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ReportRequest>> for ReportRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ReportRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ReportRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ReportRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> ReportRequest {
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

  // timestamp: optional message palm.lavender.v1.Timestamp
  pub fn has_timestamp(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_timestamp(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn timestamp_opt(&self) -> ::protobuf::Optional<super::TimestampView<'_>> {
        ::protobuf::Optional::new(self.timestamp(), self.has_timestamp())
  }
  pub fn timestamp(&self) -> super::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TimestampView::default())
  }
  pub fn timestamp_mut(&mut self) -> super::TimestampMut<'_> {
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
  pub fn set_timestamp(&mut self,
    val: impl ::protobuf::IntoProxied<super::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // items: repeated message palm.lavender.v1.ReportRequest.Item
  pub fn items(&self) -> ::protobuf::RepeatedView<'_, super::report_request::Item> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::report_request::Item>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::report_request::Item> {
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
  pub fn set_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::report_request::Item>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}

// SAFETY:
// - `ReportRequestMut` does not perform any shared mutation.
unsafe impl Send for ReportRequestMut<'_> {}

// SAFETY:
// - `ReportRequestMut` does not perform any shared mutation.
unsafe impl Sync for ReportRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for ReportRequestMut<'msg> {
  type Proxied = ReportRequest;
  fn as_view(&self) -> ::protobuf::View<'_, ReportRequest> {
    ReportRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ReportRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ReportRequest>
  where
      'msg: 'shorter {
    ReportRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ReportRequestMut<'msg> {
  type MutProxied = ReportRequest;
  fn as_mut(&mut self) -> ReportRequestMut<'msg> {
    ReportRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ReportRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> ReportRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ReportRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ReportRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ReportRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ReportRequestMut<'_> {
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

  // timestamp: optional message palm.lavender.v1.Timestamp
  pub fn has_timestamp(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_timestamp(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn timestamp_opt(&self) -> ::protobuf::Optional<super::TimestampView<'_>> {
        ::protobuf::Optional::new(self.timestamp(), self.has_timestamp())
  }
  pub fn timestamp(&self) -> super::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TimestampView::default())
  }
  pub fn timestamp_mut(&mut self) -> super::TimestampMut<'_> {
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
  pub fn set_timestamp(&mut self,
    val: impl ::protobuf::IntoProxied<super::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // items: repeated message palm.lavender.v1.ReportRequest.Item
  pub fn items(&self) -> ::protobuf::RepeatedView<'_, super::report_request::Item> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::report_request::Item>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::report_request::Item> {
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
  pub fn set_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::report_request::Item>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}  // impl ReportRequest

impl ::std::ops::Drop for ReportRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ReportRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ReportRequest {
  type Proxied = Self;
  fn as_view(&self) -> ReportRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ReportRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ReportRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ReportRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__lavender__v1__ReportRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3fG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__lavender__v1__ReportRequest_msg_init.0, &[<super::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::report_request::Item as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__lavender__v1__ReportRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ReportRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ReportRequest {
  type Msg = ReportRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ReportRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ReportRequest {
  type Msg = ReportRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ReportRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ReportRequestMut<'_> {
  type Msg = ReportRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ReportRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ReportRequestMut<'_> {
  type Msg = ReportRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ReportRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ReportRequestView<'_> {
  type Msg = ReportRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ReportRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ReportRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod report_request {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__lavender__v1__ReportRequest__Item_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
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

  // curation: optional message palm.lavender.v1.Duration
  pub fn has_curation(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn curation_opt(self) -> ::protobuf::Optional<super::super::DurationView<'msg>> {
        ::protobuf::Optional::new(self.curation(), self.has_curation())
  }
  pub fn curation(self) -> super::super::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::DurationView::default())
  }

  // http: optional message palm.lavender.v1.Http
  pub fn has_http(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn http_opt(self) -> ::protobuf::Optional<super::super::HttpView<'msg>> {
        ::protobuf::Optional::new(self.http(), self.has_http())
  }
  pub fn http(self) -> super::super::HttpView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::HttpView::default())
  }

  // postgresql: optional message palm.lavender.v1.PostgreSql
  pub fn has_postgresql(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn postgresql_opt(self) -> ::protobuf::Optional<super::super::PostgreSqlView<'msg>> {
        ::protobuf::Optional::new(self.postgresql(), self.has_postgresql())
  }
  pub fn postgresql(self) -> super::super::PostgreSqlView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::PostgreSqlView::default())
  }

  // mysql: optional message palm.lavender.v1.MySql
  pub fn has_mysql(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn mysql_opt(self) -> ::protobuf::Optional<super::super::MySqlView<'msg>> {
        ::protobuf::Optional::new(self.mysql(), self.has_mysql())
  }
  pub fn mysql(self) -> super::super::MySqlView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::MySqlView::default())
  }

  // redis: optional message palm.lavender.v1.Redis
  pub fn has_redis(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn redis_opt(self) -> ::protobuf::Optional<super::super::RedisView<'msg>> {
        ::protobuf::Optional::new(self.redis(), self.has_redis())
  }
  pub fn redis(self) -> super::super::RedisView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::RedisView::default())
  }

  // snmp: optional message palm.lavender.v1.Snmp
  pub fn has_snmp(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn snmp_opt(self) -> ::protobuf::Optional<super::super::SnmpView<'msg>> {
        ::protobuf::Optional::new(self.snmp(), self.has_snmp())
  }
  pub fn snmp(self) -> super::super::SnmpView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::SnmpView::default())
  }

  // opensearch: optional message palm.lavender.v1.OpenSearch
  pub fn has_opensearch(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn opensearch_opt(self) -> ::protobuf::Optional<super::super::OpenSearchView<'msg>> {
        ::protobuf::Optional::new(self.opensearch(), self.has_opensearch())
  }
  pub fn opensearch(self) -> super::super::OpenSearchView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::OpenSearchView::default())
  }

  pub fn node(self) -> super::super::report_request::item::NodeOneof<'msg> {
    match self.node_case() {
      super::super::report_request::item::NodeCase::Http =>
          super::super::report_request::item::NodeOneof::Http(self.http()),
      super::super::report_request::item::NodeCase::Postgresql =>
          super::super::report_request::item::NodeOneof::Postgresql(self.postgresql()),
      super::super::report_request::item::NodeCase::Mysql =>
          super::super::report_request::item::NodeOneof::Mysql(self.mysql()),
      super::super::report_request::item::NodeCase::Redis =>
          super::super::report_request::item::NodeOneof::Redis(self.redis()),
      super::super::report_request::item::NodeCase::Snmp =>
          super::super::report_request::item::NodeOneof::Snmp(self.snmp()),
      super::super::report_request::item::NodeCase::Opensearch =>
          super::super::report_request::item::NodeOneof::Opensearch(self.opensearch()),
      _ => super::super::report_request::item::NodeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn node_case(self) -> super::super::report_request::item::NodeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::report_request::item::NodeCase::try_from(field_num).unwrap_unchecked()
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

  // curation: optional message palm.lavender.v1.Duration
  pub fn has_curation(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_curation(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn curation_opt(&self) -> ::protobuf::Optional<super::super::DurationView<'_>> {
        ::protobuf::Optional::new(self.curation(), self.has_curation())
  }
  pub fn curation(&self) -> super::super::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::DurationView::default())
  }
  pub fn curation_mut(&mut self) -> super::super::DurationMut<'_> {
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
  pub fn set_curation(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // http: optional message palm.lavender.v1.Http
  pub fn has_http(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_http(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn http_opt(&self) -> ::protobuf::Optional<super::super::HttpView<'_>> {
        ::protobuf::Optional::new(self.http(), self.has_http())
  }
  pub fn http(&self) -> super::super::HttpView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::HttpView::default())
  }
  pub fn http_mut(&mut self) -> super::super::HttpMut<'_> {
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
  pub fn set_http(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Http>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // postgresql: optional message palm.lavender.v1.PostgreSql
  pub fn has_postgresql(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_postgresql(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn postgresql_opt(&self) -> ::protobuf::Optional<super::super::PostgreSqlView<'_>> {
        ::protobuf::Optional::new(self.postgresql(), self.has_postgresql())
  }
  pub fn postgresql(&self) -> super::super::PostgreSqlView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::PostgreSqlView::default())
  }
  pub fn postgresql_mut(&mut self) -> super::super::PostgreSqlMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_postgresql(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::PostgreSql>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // mysql: optional message palm.lavender.v1.MySql
  pub fn has_mysql(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_mysql(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn mysql_opt(&self) -> ::protobuf::Optional<super::super::MySqlView<'_>> {
        ::protobuf::Optional::new(self.mysql(), self.has_mysql())
  }
  pub fn mysql(&self) -> super::super::MySqlView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::MySqlView::default())
  }
  pub fn mysql_mut(&mut self) -> super::super::MySqlMut<'_> {
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
  pub fn set_mysql(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::MySql>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // redis: optional message palm.lavender.v1.Redis
  pub fn has_redis(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_redis(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn redis_opt(&self) -> ::protobuf::Optional<super::super::RedisView<'_>> {
        ::protobuf::Optional::new(self.redis(), self.has_redis())
  }
  pub fn redis(&self) -> super::super::RedisView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::RedisView::default())
  }
  pub fn redis_mut(&mut self) -> super::super::RedisMut<'_> {
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
  pub fn set_redis(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Redis>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // snmp: optional message palm.lavender.v1.Snmp
  pub fn has_snmp(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_snmp(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn snmp_opt(&self) -> ::protobuf::Optional<super::super::SnmpView<'_>> {
        ::protobuf::Optional::new(self.snmp(), self.has_snmp())
  }
  pub fn snmp(&self) -> super::super::SnmpView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::SnmpView::default())
  }
  pub fn snmp_mut(&mut self) -> super::super::SnmpMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_snmp(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Snmp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // opensearch: optional message palm.lavender.v1.OpenSearch
  pub fn has_opensearch(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_opensearch(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn opensearch_opt(&self) -> ::protobuf::Optional<super::super::OpenSearchView<'_>> {
        ::protobuf::Optional::new(self.opensearch(), self.has_opensearch())
  }
  pub fn opensearch(&self) -> super::super::OpenSearchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::OpenSearchView::default())
  }
  pub fn opensearch_mut(&mut self) -> super::super::OpenSearchMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_opensearch(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::OpenSearch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  pub fn node(&self) -> super::super::report_request::item::NodeOneof<'_> {
    match &self.node_case() {
      super::super::report_request::item::NodeCase::Http =>
          super::super::report_request::item::NodeOneof::Http(self.http()),
      super::super::report_request::item::NodeCase::Postgresql =>
          super::super::report_request::item::NodeOneof::Postgresql(self.postgresql()),
      super::super::report_request::item::NodeCase::Mysql =>
          super::super::report_request::item::NodeOneof::Mysql(self.mysql()),
      super::super::report_request::item::NodeCase::Redis =>
          super::super::report_request::item::NodeOneof::Redis(self.redis()),
      super::super::report_request::item::NodeCase::Snmp =>
          super::super::report_request::item::NodeOneof::Snmp(self.snmp()),
      super::super::report_request::item::NodeCase::Opensearch =>
          super::super::report_request::item::NodeOneof::Opensearch(self.opensearch()),
      _ => super::super::report_request::item::NodeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn node_case(&self) -> super::super::report_request::item::NodeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::report_request::item::NodeCase::try_from(field_num).unwrap_unchecked()
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

  // curation: optional message palm.lavender.v1.Duration
  pub fn has_curation(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_curation(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn curation_opt(&self) -> ::protobuf::Optional<super::super::DurationView<'_>> {
        ::protobuf::Optional::new(self.curation(), self.has_curation())
  }
  pub fn curation(&self) -> super::super::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::DurationView::default())
  }
  pub fn curation_mut(&mut self) -> super::super::DurationMut<'_> {
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
  pub fn set_curation(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // http: optional message palm.lavender.v1.Http
  pub fn has_http(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_http(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn http_opt(&self) -> ::protobuf::Optional<super::super::HttpView<'_>> {
        ::protobuf::Optional::new(self.http(), self.has_http())
  }
  pub fn http(&self) -> super::super::HttpView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::HttpView::default())
  }
  pub fn http_mut(&mut self) -> super::super::HttpMut<'_> {
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
  pub fn set_http(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Http>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // postgresql: optional message palm.lavender.v1.PostgreSql
  pub fn has_postgresql(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_postgresql(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn postgresql_opt(&self) -> ::protobuf::Optional<super::super::PostgreSqlView<'_>> {
        ::protobuf::Optional::new(self.postgresql(), self.has_postgresql())
  }
  pub fn postgresql(&self) -> super::super::PostgreSqlView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::PostgreSqlView::default())
  }
  pub fn postgresql_mut(&mut self) -> super::super::PostgreSqlMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_postgresql(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::PostgreSql>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // mysql: optional message palm.lavender.v1.MySql
  pub fn has_mysql(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_mysql(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn mysql_opt(&self) -> ::protobuf::Optional<super::super::MySqlView<'_>> {
        ::protobuf::Optional::new(self.mysql(), self.has_mysql())
  }
  pub fn mysql(&self) -> super::super::MySqlView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::MySqlView::default())
  }
  pub fn mysql_mut(&mut self) -> super::super::MySqlMut<'_> {
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
  pub fn set_mysql(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::MySql>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // redis: optional message palm.lavender.v1.Redis
  pub fn has_redis(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_redis(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn redis_opt(&self) -> ::protobuf::Optional<super::super::RedisView<'_>> {
        ::protobuf::Optional::new(self.redis(), self.has_redis())
  }
  pub fn redis(&self) -> super::super::RedisView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::RedisView::default())
  }
  pub fn redis_mut(&mut self) -> super::super::RedisMut<'_> {
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
  pub fn set_redis(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Redis>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // snmp: optional message palm.lavender.v1.Snmp
  pub fn has_snmp(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_snmp(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn snmp_opt(&self) -> ::protobuf::Optional<super::super::SnmpView<'_>> {
        ::protobuf::Optional::new(self.snmp(), self.has_snmp())
  }
  pub fn snmp(&self) -> super::super::SnmpView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::SnmpView::default())
  }
  pub fn snmp_mut(&mut self) -> super::super::SnmpMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_snmp(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Snmp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // opensearch: optional message palm.lavender.v1.OpenSearch
  pub fn has_opensearch(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_opensearch(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn opensearch_opt(&self) -> ::protobuf::Optional<super::super::OpenSearchView<'_>> {
        ::protobuf::Optional::new(self.opensearch(), self.has_opensearch())
  }
  pub fn opensearch(&self) -> super::super::OpenSearchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::OpenSearchView::default())
  }
  pub fn opensearch_mut(&mut self) -> super::super::OpenSearchMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_opensearch(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::OpenSearch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  pub fn node(&self) -> super::super::report_request::item::NodeOneof<'_> {
    match &self.node_case() {
      super::super::report_request::item::NodeCase::Http =>
          super::super::report_request::item::NodeOneof::Http(self.http()),
      super::super::report_request::item::NodeCase::Postgresql =>
          super::super::report_request::item::NodeOneof::Postgresql(self.postgresql()),
      super::super::report_request::item::NodeCase::Mysql =>
          super::super::report_request::item::NodeOneof::Mysql(self.mysql()),
      super::super::report_request::item::NodeCase::Redis =>
          super::super::report_request::item::NodeOneof::Redis(self.redis()),
      super::super::report_request::item::NodeCase::Snmp =>
          super::super::report_request::item::NodeOneof::Snmp(self.snmp()),
      super::super::report_request::item::NodeCase::Opensearch =>
          super::super::report_request::item::NodeOneof::Opensearch(self.opensearch()),
      _ => super::super::report_request::item::NodeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn node_case(&self) -> super::super::report_request::item::NodeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::report_request::item::NodeCase::try_from(field_num).unwrap_unchecked()
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
        super::super::report_request::palm__lavender__v1__ReportRequest__Item_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3i333333^-|.|/|0|1|2");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::report_request::palm__lavender__v1__ReportRequest__Item_msg_init.0, &[<super::super::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::Http as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::PostgreSql as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::MySql as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::Redis as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::Snmp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::OpenSearch as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::report_request::palm__lavender__v1__ReportRequest__Item_msg_init.0)
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
pub enum NodeOneof<'msg> {
  Http(::protobuf::View<'msg, super::super::super::Http>) = 11,
  Postgresql(::protobuf::View<'msg, super::super::super::PostgreSql>) = 12,
  Mysql(::protobuf::View<'msg, super::super::super::MySql>) = 13,
  Redis(::protobuf::View<'msg, super::super::super::Redis>) = 14,
  Snmp(::protobuf::View<'msg, super::super::super::Snmp>) = 15,
  Opensearch(::protobuf::View<'msg, super::super::super::OpenSearch>) = 16,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum NodeCase {
  Http = 11,
  Postgresql = 12,
  Mysql = 13,
  Redis = 14,
  Snmp = 15,
  Opensearch = 16,

  not_set = 0
}

impl NodeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<NodeCase> {
    match v {
      0 => Some(NodeCase::not_set),
      11 => Some(NodeCase::Http),
      12 => Some(NodeCase::Postgresql),
      13 => Some(NodeCase::Mysql),
      14 => Some(NodeCase::Redis),
      15 => Some(NodeCase::Snmp),
      16 => Some(NodeCase::Opensearch),
      _ => None
    }
  }
}
}  // pub mod item


}  // pub mod report_request


