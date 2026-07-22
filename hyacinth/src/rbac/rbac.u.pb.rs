const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.34.0-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__rbac__v1__Empty_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
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
        super::palm__rbac__v1__Empty_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__rbac__v1__Empty_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__rbac__v1__Empty_msg_init.0)
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
pub(crate) static mut palm__rbac__v1__Object_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Object {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Object>
}

impl ::protobuf::Message for Object {}

impl ::std::default::Default for Object {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Object {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Object` is `Sync` because it does not implement interior mutability.
//    Neither does `ObjectMut`.
unsafe impl Sync for Object {}

// SAFETY:
// - `Object` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Object {}

impl ::protobuf::Proxied for Object {
  type View<'msg> = ObjectView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Object {}

impl ::protobuf::MutProxied for Object {
  type Mut<'msg> = ObjectMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ObjectView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Object>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ObjectView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ObjectView<'msg> {
  type Message = Object;
}

impl ::std::fmt::Debug for ObjectView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ObjectView<'_> {
  fn default() -> ObjectView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Object>> for ObjectView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Object>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ObjectView<'msg> {

  pub fn to_owned(&self) -> Object {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // type: optional string
  pub fn r#type(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // id: optional int64
  pub fn has_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn id_opt(self) -> ::protobuf::Optional<i64> {
        ::protobuf::Optional::new(self.id(), self.has_id())
  }
  pub fn id(self) -> i64 {
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

  // code: optional string
  pub fn has_code(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn code_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.code(), self.has_code())
  }
  pub fn code(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // all: optional message palm.rbac.v1.Empty
  pub fn has_all(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn all_opt(self) -> ::protobuf::Optional<super::EmptyView<'msg>> {
        ::protobuf::Optional::new(self.all(), self.has_all())
  }
  pub fn all(self) -> super::EmptyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EmptyView::default())
  }

  pub fn by(self) -> super::object::ByOneof<'msg> {
    match self.by_case() {
      super::object::ByCase::Id =>
          super::object::ByOneof::Id(self.id()),
      super::object::ByCase::Code =>
          super::object::ByOneof::Code(self.code()),
      super::object::ByCase::All =>
          super::object::ByOneof::All(self.all()),
      _ => super::object::ByOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn by_case(self) -> super::object::ByCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::object::ByCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ObjectView` is `Sync` because it does not support mutation.
unsafe impl Sync for ObjectView<'_> {}

// SAFETY:
// - `ObjectView` is `Send` because while its alive a `ObjectMut` cannot.
// - `ObjectView` does not use thread-local data.
unsafe impl Send for ObjectView<'_> {}

impl<'msg> ::protobuf::AsView for ObjectView<'msg> {
  type Proxied = Object;
  fn as_view(&self) -> ::protobuf::View<'msg, Object> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ObjectView<'msg> {
  fn into_view<'shorter>(self) -> ObjectView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Object> for ObjectView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Object {
    let mut dst = Object::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Object> for ObjectMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Object {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Object {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ObjectView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ObjectMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ObjectMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Object>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ObjectMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ObjectMut<'msg> {
  type Message = Object;
}

impl ::std::fmt::Debug for ObjectMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Object>> for ObjectMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Object>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ObjectMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Object> {
    self.inner
  }

  pub fn to_owned(&self) -> Object {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // type: optional string
  pub fn r#type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // id: optional int64
  pub fn has_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn id_opt(&self) -> ::protobuf::Optional<i64> {
        ::protobuf::Optional::new(self.id(), self.has_id())
  }
  pub fn id(&self) -> i64 {
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
  pub fn set_id(&mut self, val: i64) {
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

  // code: optional string
  pub fn has_code(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_code(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn code_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.code(), self.has_code())
  }
  pub fn code(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_code(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // all: optional message palm.rbac.v1.Empty
  pub fn has_all(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_all(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn all_opt(&self) -> ::protobuf::Optional<super::EmptyView<'_>> {
        ::protobuf::Optional::new(self.all(), self.has_all())
  }
  pub fn all(&self) -> super::EmptyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EmptyView::default())
  }
  pub fn all_mut(&mut self) -> super::EmptyMut<'_> {
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
  pub fn set_all(&mut self,
    val: impl ::protobuf::IntoProxied<super::Empty>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  pub fn by(&self) -> super::object::ByOneof<'_> {
    match &self.by_case() {
      super::object::ByCase::Id =>
          super::object::ByOneof::Id(self.id()),
      super::object::ByCase::Code =>
          super::object::ByOneof::Code(self.code()),
      super::object::ByCase::All =>
          super::object::ByOneof::All(self.all()),
      _ => super::object::ByOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn by_case(&self) -> super::object::ByCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::object::ByCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ObjectMut` does not perform any shared mutation.
unsafe impl Send for ObjectMut<'_> {}

// SAFETY:
// - `ObjectMut` does not perform any shared mutation.
unsafe impl Sync for ObjectMut<'_> {}

impl<'msg> ::protobuf::AsView for ObjectMut<'msg> {
  type Proxied = Object;
  fn as_view(&self) -> ::protobuf::View<'_, Object> {
    ObjectView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ObjectMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Object>
  where
      'msg: 'shorter {
    ObjectView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ObjectMut<'msg> {
  type MutProxied = Object;
  fn as_mut(&mut self) -> ObjectMut<'msg> {
    ObjectMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ObjectMut<'msg> {
  fn into_mut<'shorter>(self) -> ObjectMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Object {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Object> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ObjectView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ObjectMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // type: optional string
  pub fn r#type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // id: optional int64
  pub fn has_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn id_opt(&self) -> ::protobuf::Optional<i64> {
        ::protobuf::Optional::new(self.id(), self.has_id())
  }
  pub fn id(&self) -> i64 {
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
  pub fn set_id(&mut self, val: i64) {
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

  // code: optional string
  pub fn has_code(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_code(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn code_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.code(), self.has_code())
  }
  pub fn code(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_code(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // all: optional message palm.rbac.v1.Empty
  pub fn has_all(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_all(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn all_opt(&self) -> ::protobuf::Optional<super::EmptyView<'_>> {
        ::protobuf::Optional::new(self.all(), self.has_all())
  }
  pub fn all(&self) -> super::EmptyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EmptyView::default())
  }
  pub fn all_mut(&mut self) -> super::EmptyMut<'_> {
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
  pub fn set_all(&mut self,
    val: impl ::protobuf::IntoProxied<super::Empty>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  pub fn by(&self) -> super::object::ByOneof<'_> {
    match &self.by_case() {
      super::object::ByCase::Id =>
          super::object::ByOneof::Id(self.id()),
      super::object::ByCase::Code =>
          super::object::ByOneof::Code(self.code()),
      super::object::ByCase::All =>
          super::object::ByOneof::All(self.all()),
      _ => super::object::ByOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn by_case(&self) -> super::object::ByCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::object::ByCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Object

impl ::std::ops::Drop for Object {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Object {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Object {
  type Proxied = Self;
  fn as_view(&self) -> ObjectView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Object {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ObjectMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Object {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__rbac__v1__Object_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1Xi+1Tf3^-|.|5");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__rbac__v1__Object_msg_init.0, &[<super::Empty as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__rbac__v1__Object_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Object {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Object {
  type Msg = Object;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Object> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Object {
  type Msg = Object;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Object> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ObjectMut<'_> {
  type Msg = Object;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Object> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ObjectMut<'_> {
  type Msg = Object;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Object> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ObjectView<'_> {
  type Msg = Object;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Object> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ObjectMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod object {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ByOneof<'msg> {
  Id(i64) = 11,
  Code(&'msg ::protobuf::ProtoStr) = 12,
  All(::protobuf::View<'msg, super::super::Empty>) = 19,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ByCase {
  Id = 11,
  Code = 12,
  All = 19,

  not_set = 0
}

impl ByCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ByCase> {
    match v {
      0 => Some(ByCase::not_set),
      11 => Some(ByCase::Id),
      12 => Some(ByCase::Code),
      19 => Some(ByCase::All),
      _ => None
    }
  }
}
}  // pub mod object


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__rbac__v1__Subject_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Subject {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Subject>
}

impl ::protobuf::Message for Subject {}

impl ::std::default::Default for Subject {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Subject {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Subject` is `Sync` because it does not implement interior mutability.
//    Neither does `SubjectMut`.
unsafe impl Sync for Subject {}

// SAFETY:
// - `Subject` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Subject {}

impl ::protobuf::Proxied for Subject {
  type View<'msg> = SubjectView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Subject {}

impl ::protobuf::MutProxied for Subject {
  type Mut<'msg> = SubjectMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SubjectView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Subject>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SubjectView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SubjectView<'msg> {
  type Message = Subject;
}

impl ::std::fmt::Debug for SubjectView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SubjectView<'_> {
  fn default() -> SubjectView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Subject>> for SubjectView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Subject>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SubjectView<'msg> {

  pub fn to_owned(&self) -> Subject {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // user: optional message palm.rbac.v1.Subject.User
  pub fn has_user(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn user_opt(self) -> ::protobuf::Optional<super::subject::UserView<'msg>> {
        ::protobuf::Optional::new(self.user(), self.has_user())
  }
  pub fn user(self) -> super::subject::UserView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::subject::UserView::default())
  }

  // role: optional message palm.rbac.v1.Subject.Role
  pub fn has_role(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn role_opt(self) -> ::protobuf::Optional<super::subject::RoleView<'msg>> {
        ::protobuf::Optional::new(self.role(), self.has_role())
  }
  pub fn role(self) -> super::subject::RoleView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::subject::RoleView::default())
  }

  pub fn by(self) -> super::subject::ByOneof<'msg> {
    match self.by_case() {
      super::subject::ByCase::User =>
          super::subject::ByOneof::User(self.user()),
      super::subject::ByCase::Role =>
          super::subject::ByOneof::Role(self.role()),
      _ => super::subject::ByOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn by_case(self) -> super::subject::ByCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::subject::ByCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `SubjectView` is `Sync` because it does not support mutation.
unsafe impl Sync for SubjectView<'_> {}

// SAFETY:
// - `SubjectView` is `Send` because while its alive a `SubjectMut` cannot.
// - `SubjectView` does not use thread-local data.
unsafe impl Send for SubjectView<'_> {}

impl<'msg> ::protobuf::AsView for SubjectView<'msg> {
  type Proxied = Subject;
  fn as_view(&self) -> ::protobuf::View<'msg, Subject> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SubjectView<'msg> {
  fn into_view<'shorter>(self) -> SubjectView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Subject> for SubjectView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Subject {
    let mut dst = Subject::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Subject> for SubjectMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Subject {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Subject {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SubjectView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SubjectMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SubjectMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Subject>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SubjectMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SubjectMut<'msg> {
  type Message = Subject;
}

impl ::std::fmt::Debug for SubjectMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Subject>> for SubjectMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Subject>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SubjectMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Subject> {
    self.inner
  }

  pub fn to_owned(&self) -> Subject {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // user: optional message palm.rbac.v1.Subject.User
  pub fn has_user(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_user(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn user_opt(&self) -> ::protobuf::Optional<super::subject::UserView<'_>> {
        ::protobuf::Optional::new(self.user(), self.has_user())
  }
  pub fn user(&self) -> super::subject::UserView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::subject::UserView::default())
  }
  pub fn user_mut(&mut self) -> super::subject::UserMut<'_> {
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
  pub fn set_user(&mut self,
    val: impl ::protobuf::IntoProxied<super::subject::User>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // role: optional message palm.rbac.v1.Subject.Role
  pub fn has_role(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_role(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn role_opt(&self) -> ::protobuf::Optional<super::subject::RoleView<'_>> {
        ::protobuf::Optional::new(self.role(), self.has_role())
  }
  pub fn role(&self) -> super::subject::RoleView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::subject::RoleView::default())
  }
  pub fn role_mut(&mut self) -> super::subject::RoleMut<'_> {
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
  pub fn set_role(&mut self,
    val: impl ::protobuf::IntoProxied<super::subject::Role>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn by(&self) -> super::subject::ByOneof<'_> {
    match &self.by_case() {
      super::subject::ByCase::User =>
          super::subject::ByOneof::User(self.user()),
      super::subject::ByCase::Role =>
          super::subject::ByOneof::Role(self.role()),
      _ => super::subject::ByOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn by_case(&self) -> super::subject::ByCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::subject::ByCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `SubjectMut` does not perform any shared mutation.
unsafe impl Send for SubjectMut<'_> {}

// SAFETY:
// - `SubjectMut` does not perform any shared mutation.
unsafe impl Sync for SubjectMut<'_> {}

impl<'msg> ::protobuf::AsView for SubjectMut<'msg> {
  type Proxied = Subject;
  fn as_view(&self) -> ::protobuf::View<'_, Subject> {
    SubjectView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SubjectMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Subject>
  where
      'msg: 'shorter {
    SubjectView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for SubjectMut<'msg> {
  type MutProxied = Subject;
  fn as_mut(&mut self) -> SubjectMut<'msg> {
    SubjectMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SubjectMut<'msg> {
  fn into_mut<'shorter>(self) -> SubjectMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Subject {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Subject> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SubjectView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SubjectMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // user: optional message palm.rbac.v1.Subject.User
  pub fn has_user(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_user(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn user_opt(&self) -> ::protobuf::Optional<super::subject::UserView<'_>> {
        ::protobuf::Optional::new(self.user(), self.has_user())
  }
  pub fn user(&self) -> super::subject::UserView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::subject::UserView::default())
  }
  pub fn user_mut(&mut self) -> super::subject::UserMut<'_> {
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
  pub fn set_user(&mut self,
    val: impl ::protobuf::IntoProxied<super::subject::User>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // role: optional message palm.rbac.v1.Subject.Role
  pub fn has_role(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_role(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn role_opt(&self) -> ::protobuf::Optional<super::subject::RoleView<'_>> {
        ::protobuf::Optional::new(self.role(), self.has_role())
  }
  pub fn role(&self) -> super::subject::RoleView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::subject::RoleView::default())
  }
  pub fn role_mut(&mut self) -> super::subject::RoleMut<'_> {
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
  pub fn set_role(&mut self,
    val: impl ::protobuf::IntoProxied<super::subject::Role>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn by(&self) -> super::subject::ByOneof<'_> {
    match &self.by_case() {
      super::subject::ByCase::User =>
          super::subject::ByOneof::User(self.user()),
      super::subject::ByCase::Role =>
          super::subject::ByOneof::Role(self.role()),
      _ => super::subject::ByOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn by_case(&self) -> super::subject::ByCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::subject::ByCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Subject

impl ::std::ops::Drop for Subject {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Subject {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Subject {
  type Proxied = Self;
  fn as_view(&self) -> SubjectView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Subject {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SubjectMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Subject {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__rbac__v1__Subject_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__rbac__v1__Subject_msg_init.0, &[<super::subject::User as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::subject::Role as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__rbac__v1__Subject_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Subject {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Subject {
  type Msg = Subject;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Subject> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Subject {
  type Msg = Subject;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Subject> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SubjectMut<'_> {
  type Msg = Subject;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Subject> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SubjectMut<'_> {
  type Msg = Subject;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Subject> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SubjectView<'_> {
  type Msg = Subject;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Subject> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SubjectMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod subject {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__rbac__v1__Subject__Role_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Role {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Role>
}

impl ::protobuf::Message for Role {}

impl ::std::default::Default for Role {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Role {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Role` is `Sync` because it does not implement interior mutability.
//    Neither does `RoleMut`.
unsafe impl Sync for Role {}

// SAFETY:
// - `Role` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Role {}

impl ::protobuf::Proxied for Role {
  type View<'msg> = RoleView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Role {}

impl ::protobuf::MutProxied for Role {
  type Mut<'msg> = RoleMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RoleView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Role>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RoleView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RoleView<'msg> {
  type Message = Role;
}

impl ::std::fmt::Debug for RoleView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RoleView<'_> {
  fn default() -> RoleView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Role>> for RoleView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Role>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RoleView<'msg> {

  pub fn to_owned(&self) -> Role {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // root: optional message palm.rbac.v1.Subject.Role.Root
  pub fn has_root(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn root_opt(self) -> ::protobuf::Optional<super::super::subject::role::RootView<'msg>> {
        ::protobuf::Optional::new(self.root(), self.has_root())
  }
  pub fn root(self) -> super::super::subject::role::RootView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::subject::role::RootView::default())
  }

  // administrator: optional message palm.rbac.v1.Subject.Role.Administrator
  pub fn has_administrator(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn administrator_opt(self) -> ::protobuf::Optional<super::super::subject::role::AdministratorView<'msg>> {
        ::protobuf::Optional::new(self.administrator(), self.has_administrator())
  }
  pub fn administrator(self) -> super::super::subject::role::AdministratorView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::subject::role::AdministratorView::default())
  }

  // id: optional int64
  pub fn has_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn id_opt(self) -> ::protobuf::Optional<i64> {
        ::protobuf::Optional::new(self.id(), self.has_id())
  }
  pub fn id(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // code: optional string
  pub fn has_code(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn code_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.code(), self.has_code())
  }
  pub fn code(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  pub fn by(self) -> super::super::subject::role::ByOneof<'msg> {
    match self.by_case() {
      super::super::subject::role::ByCase::Root =>
          super::super::subject::role::ByOneof::Root(self.root()),
      super::super::subject::role::ByCase::Administrator =>
          super::super::subject::role::ByOneof::Administrator(self.administrator()),
      super::super::subject::role::ByCase::Id =>
          super::super::subject::role::ByOneof::Id(self.id()),
      super::super::subject::role::ByCase::Code =>
          super::super::subject::role::ByOneof::Code(self.code()),
      _ => super::super::subject::role::ByOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn by_case(self) -> super::super::subject::role::ByCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::subject::role::ByCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `RoleView` is `Sync` because it does not support mutation.
unsafe impl Sync for RoleView<'_> {}

// SAFETY:
// - `RoleView` is `Send` because while its alive a `RoleMut` cannot.
// - `RoleView` does not use thread-local data.
unsafe impl Send for RoleView<'_> {}

impl<'msg> ::protobuf::AsView for RoleView<'msg> {
  type Proxied = Role;
  fn as_view(&self) -> ::protobuf::View<'msg, Role> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RoleView<'msg> {
  fn into_view<'shorter>(self) -> RoleView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Role> for RoleView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Role {
    let mut dst = Role::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Role> for RoleMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Role {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Role {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for RoleView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for RoleMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RoleMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Role>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RoleMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RoleMut<'msg> {
  type Message = Role;
}

impl ::std::fmt::Debug for RoleMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Role>> for RoleMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Role>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RoleMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Role> {
    self.inner
  }

  pub fn to_owned(&self) -> Role {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // root: optional message palm.rbac.v1.Subject.Role.Root
  pub fn has_root(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_root(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn root_opt(&self) -> ::protobuf::Optional<super::super::subject::role::RootView<'_>> {
        ::protobuf::Optional::new(self.root(), self.has_root())
  }
  pub fn root(&self) -> super::super::subject::role::RootView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::subject::role::RootView::default())
  }
  pub fn root_mut(&mut self) -> super::super::subject::role::RootMut<'_> {
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
  pub fn set_root(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::subject::role::Root>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // administrator: optional message palm.rbac.v1.Subject.Role.Administrator
  pub fn has_administrator(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_administrator(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn administrator_opt(&self) -> ::protobuf::Optional<super::super::subject::role::AdministratorView<'_>> {
        ::protobuf::Optional::new(self.administrator(), self.has_administrator())
  }
  pub fn administrator(&self) -> super::super::subject::role::AdministratorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::subject::role::AdministratorView::default())
  }
  pub fn administrator_mut(&mut self) -> super::super::subject::role::AdministratorMut<'_> {
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
  pub fn set_administrator(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::subject::role::Administrator>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // id: optional int64
  pub fn has_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn id_opt(&self) -> ::protobuf::Optional<i64> {
        ::protobuf::Optional::new(self.id(), self.has_id())
  }
  pub fn id(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
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
        2, val.into()
      )
    }
  }

  // code: optional string
  pub fn has_code(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_code(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn code_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.code(), self.has_code())
  }
  pub fn code(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_code(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  pub fn by(&self) -> super::super::subject::role::ByOneof<'_> {
    match &self.by_case() {
      super::super::subject::role::ByCase::Root =>
          super::super::subject::role::ByOneof::Root(self.root()),
      super::super::subject::role::ByCase::Administrator =>
          super::super::subject::role::ByOneof::Administrator(self.administrator()),
      super::super::subject::role::ByCase::Id =>
          super::super::subject::role::ByOneof::Id(self.id()),
      super::super::subject::role::ByCase::Code =>
          super::super::subject::role::ByOneof::Code(self.code()),
      _ => super::super::subject::role::ByOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn by_case(&self) -> super::super::subject::role::ByCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::subject::role::ByCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `RoleMut` does not perform any shared mutation.
unsafe impl Send for RoleMut<'_> {}

// SAFETY:
// - `RoleMut` does not perform any shared mutation.
unsafe impl Sync for RoleMut<'_> {}

impl<'msg> ::protobuf::AsView for RoleMut<'msg> {
  type Proxied = Role;
  fn as_view(&self) -> ::protobuf::View<'_, Role> {
    RoleView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RoleMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Role>
  where
      'msg: 'shorter {
    RoleView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for RoleMut<'msg> {
  type MutProxied = Role;
  fn as_mut(&mut self) -> RoleMut<'msg> {
    RoleMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RoleMut<'msg> {
  fn into_mut<'shorter>(self) -> RoleMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Role {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Role> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RoleView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RoleMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // root: optional message palm.rbac.v1.Subject.Role.Root
  pub fn has_root(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_root(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn root_opt(&self) -> ::protobuf::Optional<super::super::subject::role::RootView<'_>> {
        ::protobuf::Optional::new(self.root(), self.has_root())
  }
  pub fn root(&self) -> super::super::subject::role::RootView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::subject::role::RootView::default())
  }
  pub fn root_mut(&mut self) -> super::super::subject::role::RootMut<'_> {
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
  pub fn set_root(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::subject::role::Root>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // administrator: optional message palm.rbac.v1.Subject.Role.Administrator
  pub fn has_administrator(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_administrator(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn administrator_opt(&self) -> ::protobuf::Optional<super::super::subject::role::AdministratorView<'_>> {
        ::protobuf::Optional::new(self.administrator(), self.has_administrator())
  }
  pub fn administrator(&self) -> super::super::subject::role::AdministratorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::subject::role::AdministratorView::default())
  }
  pub fn administrator_mut(&mut self) -> super::super::subject::role::AdministratorMut<'_> {
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
  pub fn set_administrator(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::subject::role::Administrator>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // id: optional int64
  pub fn has_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn id_opt(&self) -> ::protobuf::Optional<i64> {
        ::protobuf::Optional::new(self.id(), self.has_id())
  }
  pub fn id(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
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
        2, val.into()
      )
    }
  }

  // code: optional string
  pub fn has_code(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_code(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn code_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.code(), self.has_code())
  }
  pub fn code(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_code(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  pub fn by(&self) -> super::super::subject::role::ByOneof<'_> {
    match &self.by_case() {
      super::super::subject::role::ByCase::Root =>
          super::super::subject::role::ByOneof::Root(self.root()),
      super::super::subject::role::ByCase::Administrator =>
          super::super::subject::role::ByOneof::Administrator(self.administrator()),
      super::super::subject::role::ByCase::Id =>
          super::super::subject::role::ByOneof::Id(self.id()),
      super::super::subject::role::ByCase::Code =>
          super::super::subject::role::ByOneof::Code(self.code()),
      _ => super::super::subject::role::ByOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn by_case(&self) -> super::super::subject::role::ByCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::subject::role::ByCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Role

impl ::std::ops::Drop for Role {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Role {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Role {
  type Proxied = Self;
  fn as_view(&self) -> RoleView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Role {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RoleMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Role {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::subject::palm__rbac__v1__Subject__Role_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33e+1T^!|#|*|+");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::subject::palm__rbac__v1__Subject__Role_msg_init.0, &[<super::super::subject::role::Root as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::subject::role::Administrator as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::subject::palm__rbac__v1__Subject__Role_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Role {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Role {
  type Msg = Role;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Role> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Role {
  type Msg = Role;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Role> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RoleMut<'_> {
  type Msg = Role;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Role> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RoleMut<'_> {
  type Msg = Role;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Role> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RoleView<'_> {
  type Msg = Role;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Role> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RoleMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod role {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__rbac__v1__Subject__Role__Root_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Root {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Root>
}

impl ::protobuf::Message for Root {}

impl ::std::default::Default for Root {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Root {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Root` is `Sync` because it does not implement interior mutability.
//    Neither does `RootMut`.
unsafe impl Sync for Root {}

// SAFETY:
// - `Root` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Root {}

impl ::protobuf::Proxied for Root {
  type View<'msg> = RootView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Root {}

impl ::protobuf::MutProxied for Root {
  type Mut<'msg> = RootMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RootView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Root>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RootView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RootView<'msg> {
  type Message = Root;
}

impl ::std::fmt::Debug for RootView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RootView<'_> {
  fn default() -> RootView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Root>> for RootView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Root>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RootView<'msg> {

  pub fn to_owned(&self) -> Root {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `RootView` is `Sync` because it does not support mutation.
unsafe impl Sync for RootView<'_> {}

// SAFETY:
// - `RootView` is `Send` because while its alive a `RootMut` cannot.
// - `RootView` does not use thread-local data.
unsafe impl Send for RootView<'_> {}

impl<'msg> ::protobuf::AsView for RootView<'msg> {
  type Proxied = Root;
  fn as_view(&self) -> ::protobuf::View<'msg, Root> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RootView<'msg> {
  fn into_view<'shorter>(self) -> RootView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Root> for RootView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Root {
    let mut dst = Root::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Root> for RootMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Root {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Root {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for RootView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for RootMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RootMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Root>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RootMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RootMut<'msg> {
  type Message = Root;
}

impl ::std::fmt::Debug for RootMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Root>> for RootMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Root>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RootMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Root> {
    self.inner
  }

  pub fn to_owned(&self) -> Root {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `RootMut` does not perform any shared mutation.
unsafe impl Send for RootMut<'_> {}

// SAFETY:
// - `RootMut` does not perform any shared mutation.
unsafe impl Sync for RootMut<'_> {}

impl<'msg> ::protobuf::AsView for RootMut<'msg> {
  type Proxied = Root;
  fn as_view(&self) -> ::protobuf::View<'_, Root> {
    RootView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RootMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Root>
  where
      'msg: 'shorter {
    RootView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for RootMut<'msg> {
  type MutProxied = Root;
  fn as_mut(&mut self) -> RootMut<'msg> {
    RootMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RootMut<'msg> {
  fn into_mut<'shorter>(self) -> RootMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Root {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Root> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RootView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RootMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl Root

impl ::std::ops::Drop for Root {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Root {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Root {
  type Proxied = Self;
  fn as_view(&self) -> RootView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Root {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RootMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Root {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::subject::role::palm__rbac__v1__Subject__Role__Root_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::subject::role::palm__rbac__v1__Subject__Role__Root_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::subject::role::palm__rbac__v1__Subject__Role__Root_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Root {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Root {
  type Msg = Root;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Root> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Root {
  type Msg = Root;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Root> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RootMut<'_> {
  type Msg = Root;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Root> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RootMut<'_> {
  type Msg = Root;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Root> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RootView<'_> {
  type Msg = Root;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Root> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RootMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__rbac__v1__Subject__Role__Administrator_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Administrator {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Administrator>
}

impl ::protobuf::Message for Administrator {}

impl ::std::default::Default for Administrator {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Administrator {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Administrator` is `Sync` because it does not implement interior mutability.
//    Neither does `AdministratorMut`.
unsafe impl Sync for Administrator {}

// SAFETY:
// - `Administrator` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Administrator {}

impl ::protobuf::Proxied for Administrator {
  type View<'msg> = AdministratorView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Administrator {}

impl ::protobuf::MutProxied for Administrator {
  type Mut<'msg> = AdministratorMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AdministratorView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Administrator>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AdministratorView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AdministratorView<'msg> {
  type Message = Administrator;
}

impl ::std::fmt::Debug for AdministratorView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AdministratorView<'_> {
  fn default() -> AdministratorView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Administrator>> for AdministratorView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Administrator>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AdministratorView<'msg> {

  pub fn to_owned(&self) -> Administrator {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `AdministratorView` is `Sync` because it does not support mutation.
unsafe impl Sync for AdministratorView<'_> {}

// SAFETY:
// - `AdministratorView` is `Send` because while its alive a `AdministratorMut` cannot.
// - `AdministratorView` does not use thread-local data.
unsafe impl Send for AdministratorView<'_> {}

impl<'msg> ::protobuf::AsView for AdministratorView<'msg> {
  type Proxied = Administrator;
  fn as_view(&self) -> ::protobuf::View<'msg, Administrator> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AdministratorView<'msg> {
  fn into_view<'shorter>(self) -> AdministratorView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Administrator> for AdministratorView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Administrator {
    let mut dst = Administrator::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Administrator> for AdministratorMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Administrator {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Administrator {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AdministratorView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AdministratorMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AdministratorMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Administrator>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AdministratorMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AdministratorMut<'msg> {
  type Message = Administrator;
}

impl ::std::fmt::Debug for AdministratorMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Administrator>> for AdministratorMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Administrator>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AdministratorMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Administrator> {
    self.inner
  }

  pub fn to_owned(&self) -> Administrator {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `AdministratorMut` does not perform any shared mutation.
unsafe impl Send for AdministratorMut<'_> {}

// SAFETY:
// - `AdministratorMut` does not perform any shared mutation.
unsafe impl Sync for AdministratorMut<'_> {}

impl<'msg> ::protobuf::AsView for AdministratorMut<'msg> {
  type Proxied = Administrator;
  fn as_view(&self) -> ::protobuf::View<'_, Administrator> {
    AdministratorView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AdministratorMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Administrator>
  where
      'msg: 'shorter {
    AdministratorView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for AdministratorMut<'msg> {
  type MutProxied = Administrator;
  fn as_mut(&mut self) -> AdministratorMut<'msg> {
    AdministratorMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AdministratorMut<'msg> {
  fn into_mut<'shorter>(self) -> AdministratorMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Administrator {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Administrator> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AdministratorView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AdministratorMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl Administrator

impl ::std::ops::Drop for Administrator {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Administrator {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Administrator {
  type Proxied = Self;
  fn as_view(&self) -> AdministratorView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Administrator {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AdministratorMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Administrator {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::subject::role::palm__rbac__v1__Subject__Role__Administrator_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::subject::role::palm__rbac__v1__Subject__Role__Administrator_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::subject::role::palm__rbac__v1__Subject__Role__Administrator_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Administrator {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Administrator {
  type Msg = Administrator;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Administrator> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Administrator {
  type Msg = Administrator;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Administrator> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AdministratorMut<'_> {
  type Msg = Administrator;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Administrator> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AdministratorMut<'_> {
  type Msg = Administrator;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Administrator> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AdministratorView<'_> {
  type Msg = Administrator;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Administrator> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AdministratorMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ByOneof<'msg> {
  Root(::protobuf::View<'msg, super::super::super::subject::role::Root>) = 1,
  Administrator(::protobuf::View<'msg, super::super::super::subject::role::Administrator>) = 2,
  Id(i64) = 8,
  Code(&'msg ::protobuf::ProtoStr) = 9,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ByCase {
  Root = 1,
  Administrator = 2,
  Id = 8,
  Code = 9,

  not_set = 0
}

impl ByCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ByCase> {
    match v {
      0 => Some(ByCase::not_set),
      1 => Some(ByCase::Root),
      2 => Some(ByCase::Administrator),
      8 => Some(ByCase::Id),
      9 => Some(ByCase::Code),
      _ => None
    }
  }
}
}  // pub mod role

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__rbac__v1__Subject__User_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct User {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<User>
}

impl ::protobuf::Message for User {}

impl ::std::default::Default for User {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for User {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `User` is `Sync` because it does not implement interior mutability.
//    Neither does `UserMut`.
unsafe impl Sync for User {}

// SAFETY:
// - `User` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for User {}

impl ::protobuf::Proxied for User {
  type View<'msg> = UserView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for User {}

impl ::protobuf::MutProxied for User {
  type Mut<'msg> = UserMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UserView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, User>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UserView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UserView<'msg> {
  type Message = User;
}

impl ::std::fmt::Debug for UserView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UserView<'_> {
  fn default() -> UserView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, User>> for UserView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, User>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UserView<'msg> {

  pub fn to_owned(&self) -> User {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // id: optional int64
  pub fn has_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn id_opt(self) -> ::protobuf::Optional<i64> {
        ::protobuf::Optional::new(self.id(), self.has_id())
  }
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

  // code: optional string
  pub fn has_code(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn code_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.code(), self.has_code())
  }
  pub fn code(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  pub fn by(self) -> super::super::subject::user::ByOneof<'msg> {
    match self.by_case() {
      super::super::subject::user::ByCase::Id =>
          super::super::subject::user::ByOneof::Id(self.id()),
      super::super::subject::user::ByCase::Code =>
          super::super::subject::user::ByOneof::Code(self.code()),
      _ => super::super::subject::user::ByOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn by_case(self) -> super::super::subject::user::ByCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::subject::user::ByCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `UserView` is `Sync` because it does not support mutation.
unsafe impl Sync for UserView<'_> {}

// SAFETY:
// - `UserView` is `Send` because while its alive a `UserMut` cannot.
// - `UserView` does not use thread-local data.
unsafe impl Send for UserView<'_> {}

impl<'msg> ::protobuf::AsView for UserView<'msg> {
  type Proxied = User;
  fn as_view(&self) -> ::protobuf::View<'msg, User> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UserView<'msg> {
  fn into_view<'shorter>(self) -> UserView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<User> for UserView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> User {
    let mut dst = User::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<User> for UserMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> User {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for User {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for UserView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for UserMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UserMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, User>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UserMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UserMut<'msg> {
  type Message = User;
}

impl ::std::fmt::Debug for UserMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, User>> for UserMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, User>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UserMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, User> {
    self.inner
  }

  pub fn to_owned(&self) -> User {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // id: optional int64
  pub fn has_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn id_opt(&self) -> ::protobuf::Optional<i64> {
        ::protobuf::Optional::new(self.id(), self.has_id())
  }
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

  // code: optional string
  pub fn has_code(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_code(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn code_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.code(), self.has_code())
  }
  pub fn code(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_code(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  pub fn by(&self) -> super::super::subject::user::ByOneof<'_> {
    match &self.by_case() {
      super::super::subject::user::ByCase::Id =>
          super::super::subject::user::ByOneof::Id(self.id()),
      super::super::subject::user::ByCase::Code =>
          super::super::subject::user::ByOneof::Code(self.code()),
      _ => super::super::subject::user::ByOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn by_case(&self) -> super::super::subject::user::ByCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::subject::user::ByCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `UserMut` does not perform any shared mutation.
unsafe impl Send for UserMut<'_> {}

// SAFETY:
// - `UserMut` does not perform any shared mutation.
unsafe impl Sync for UserMut<'_> {}

impl<'msg> ::protobuf::AsView for UserMut<'msg> {
  type Proxied = User;
  fn as_view(&self) -> ::protobuf::View<'_, User> {
    UserView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UserMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, User>
  where
      'msg: 'shorter {
    UserView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for UserMut<'msg> {
  type MutProxied = User;
  fn as_mut(&mut self) -> UserMut<'msg> {
    UserMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UserMut<'msg> {
  fn into_mut<'shorter>(self) -> UserMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl User {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, User> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UserView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UserMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // id: optional int64
  pub fn has_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn id_opt(&self) -> ::protobuf::Optional<i64> {
        ::protobuf::Optional::new(self.id(), self.has_id())
  }
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

  // code: optional string
  pub fn has_code(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_code(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn code_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.code(), self.has_code())
  }
  pub fn code(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_code(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  pub fn by(&self) -> super::super::subject::user::ByOneof<'_> {
    match &self.by_case() {
      super::super::subject::user::ByCase::Id =>
          super::super::subject::user::ByOneof::Id(self.id()),
      super::super::subject::user::ByCase::Code =>
          super::super::subject::user::ByOneof::Code(self.code()),
      _ => super::super::subject::user::ByOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn by_case(&self) -> super::super::subject::user::ByCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::subject::user::ByCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl User

impl ::std::ops::Drop for User {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for User {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for User {
  type Proxied = Self;
  fn as_view(&self) -> UserView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for User {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UserMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for User {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::subject::palm__rbac__v1__Subject__User_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$+1T^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::subject::palm__rbac__v1__Subject__User_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::subject::palm__rbac__v1__Subject__User_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for User {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for User {
  type Msg = User;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<User> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for User {
  type Msg = User;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<User> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UserMut<'_> {
  type Msg = User;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<User> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UserMut<'_> {
  type Msg = User;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<User> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UserView<'_> {
  type Msg = User;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<User> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UserMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod user {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ByOneof<'msg> {
  Id(i64) = 1,
  Code(&'msg ::protobuf::ProtoStr) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ByCase {
  Id = 1,
  Code = 2,

  not_set = 0
}

impl ByCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ByCase> {
    match v {
      0 => Some(ByCase::not_set),
      1 => Some(ByCase::Id),
      2 => Some(ByCase::Code),
      _ => None
    }
  }
}
}  // pub mod user


#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ByOneof<'msg> {
  User(::protobuf::View<'msg, super::super::subject::User>) = 1,
  Role(::protobuf::View<'msg, super::super::subject::Role>) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ByCase {
  User = 1,
  Role = 2,

  not_set = 0
}

impl ByCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ByCase> {
    match v {
      0 => Some(ByCase::not_set),
      1 => Some(ByCase::User),
      2 => Some(ByCase::Role),
      _ => None
    }
  }
}
}  // pub mod subject


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__rbac__v1__Action_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Action {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Action>
}

impl ::protobuf::Message for Action {}

impl ::std::default::Default for Action {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Action {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Action` is `Sync` because it does not implement interior mutability.
//    Neither does `ActionMut`.
unsafe impl Sync for Action {}

// SAFETY:
// - `Action` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Action {}

impl ::protobuf::Proxied for Action {
  type View<'msg> = ActionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Action {}

impl ::protobuf::MutProxied for Action {
  type Mut<'msg> = ActionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ActionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Action>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ActionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ActionView<'msg> {
  type Message = Action;
}

impl ::std::fmt::Debug for ActionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ActionView<'_> {
  fn default() -> ActionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Action>> for ActionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Action>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ActionView<'msg> {

  pub fn to_owned(&self) -> Action {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // read: optional message palm.rbac.v1.Action.Read
  pub fn has_read(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn read_opt(self) -> ::protobuf::Optional<super::action::ReadView<'msg>> {
        ::protobuf::Optional::new(self.read(), self.has_read())
  }
  pub fn read(self) -> super::action::ReadView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::action::ReadView::default())
  }

  // write: optional message palm.rbac.v1.Action.Write
  pub fn has_write(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn write_opt(self) -> ::protobuf::Optional<super::action::WriteView<'msg>> {
        ::protobuf::Optional::new(self.write(), self.has_write())
  }
  pub fn write(self) -> super::action::WriteView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::action::WriteView::default())
  }

  // append: optional message palm.rbac.v1.Action.Append
  pub fn has_append(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn append_opt(self) -> ::protobuf::Optional<super::action::AppendView<'msg>> {
        ::protobuf::Optional::new(self.append(), self.has_append())
  }
  pub fn append(self) -> super::action::AppendView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::action::AppendView::default())
  }

  // execute: optional message palm.rbac.v1.Action.Execute
  pub fn has_execute(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn execute_opt(self) -> ::protobuf::Optional<super::action::ExecuteView<'msg>> {
        ::protobuf::Optional::new(self.execute(), self.has_execute())
  }
  pub fn execute(self) -> super::action::ExecuteView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::action::ExecuteView::default())
  }

  // credit: optional message palm.rbac.v1.Action.Credit
  pub fn has_credit(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn credit_opt(self) -> ::protobuf::Optional<super::action::CreditView<'msg>> {
        ::protobuf::Optional::new(self.credit(), self.has_credit())
  }
  pub fn credit(self) -> super::action::CreditView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::action::CreditView::default())
  }

  // debit: optional message palm.rbac.v1.Action.Debit
  pub fn has_debit(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn debit_opt(self) -> ::protobuf::Optional<super::action::DebitView<'msg>> {
        ::protobuf::Optional::new(self.debit(), self.has_debit())
  }
  pub fn debit(self) -> super::action::DebitView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::action::DebitView::default())
  }

  // inquiry: optional message palm.rbac.v1.Action.Inquiry
  pub fn has_inquiry(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn inquiry_opt(self) -> ::protobuf::Optional<super::action::InquiryView<'msg>> {
        ::protobuf::Optional::new(self.inquiry(), self.has_inquiry())
  }
  pub fn inquiry(self) -> super::action::InquiryView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::action::InquiryView::default())
  }

  // code: optional string
  pub fn has_code(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn code_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.code(), self.has_code())
  }
  pub fn code(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  pub fn by(self) -> super::action::ByOneof<'msg> {
    match self.by_case() {
      super::action::ByCase::Read =>
          super::action::ByOneof::Read(self.read()),
      super::action::ByCase::Write =>
          super::action::ByOneof::Write(self.write()),
      super::action::ByCase::Append =>
          super::action::ByOneof::Append(self.append()),
      super::action::ByCase::Execute =>
          super::action::ByOneof::Execute(self.execute()),
      super::action::ByCase::Credit =>
          super::action::ByOneof::Credit(self.credit()),
      super::action::ByCase::Debit =>
          super::action::ByOneof::Debit(self.debit()),
      super::action::ByCase::Inquiry =>
          super::action::ByOneof::Inquiry(self.inquiry()),
      super::action::ByCase::Code =>
          super::action::ByOneof::Code(self.code()),
      _ => super::action::ByOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn by_case(self) -> super::action::ByCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::action::ByCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ActionView` is `Sync` because it does not support mutation.
unsafe impl Sync for ActionView<'_> {}

// SAFETY:
// - `ActionView` is `Send` because while its alive a `ActionMut` cannot.
// - `ActionView` does not use thread-local data.
unsafe impl Send for ActionView<'_> {}

impl<'msg> ::protobuf::AsView for ActionView<'msg> {
  type Proxied = Action;
  fn as_view(&self) -> ::protobuf::View<'msg, Action> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ActionView<'msg> {
  fn into_view<'shorter>(self) -> ActionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Action> for ActionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Action {
    let mut dst = Action::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Action> for ActionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Action {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Action {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ActionView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ActionMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ActionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Action>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ActionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ActionMut<'msg> {
  type Message = Action;
}

impl ::std::fmt::Debug for ActionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Action>> for ActionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Action>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ActionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Action> {
    self.inner
  }

  pub fn to_owned(&self) -> Action {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // read: optional message palm.rbac.v1.Action.Read
  pub fn has_read(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_read(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn read_opt(&self) -> ::protobuf::Optional<super::action::ReadView<'_>> {
        ::protobuf::Optional::new(self.read(), self.has_read())
  }
  pub fn read(&self) -> super::action::ReadView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::action::ReadView::default())
  }
  pub fn read_mut(&mut self) -> super::action::ReadMut<'_> {
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
  pub fn set_read(&mut self,
    val: impl ::protobuf::IntoProxied<super::action::Read>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // write: optional message palm.rbac.v1.Action.Write
  pub fn has_write(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_write(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn write_opt(&self) -> ::protobuf::Optional<super::action::WriteView<'_>> {
        ::protobuf::Optional::new(self.write(), self.has_write())
  }
  pub fn write(&self) -> super::action::WriteView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::action::WriteView::default())
  }
  pub fn write_mut(&mut self) -> super::action::WriteMut<'_> {
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
  pub fn set_write(&mut self,
    val: impl ::protobuf::IntoProxied<super::action::Write>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // append: optional message palm.rbac.v1.Action.Append
  pub fn has_append(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_append(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn append_opt(&self) -> ::protobuf::Optional<super::action::AppendView<'_>> {
        ::protobuf::Optional::new(self.append(), self.has_append())
  }
  pub fn append(&self) -> super::action::AppendView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::action::AppendView::default())
  }
  pub fn append_mut(&mut self) -> super::action::AppendMut<'_> {
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
  pub fn set_append(&mut self,
    val: impl ::protobuf::IntoProxied<super::action::Append>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // execute: optional message palm.rbac.v1.Action.Execute
  pub fn has_execute(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_execute(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn execute_opt(&self) -> ::protobuf::Optional<super::action::ExecuteView<'_>> {
        ::protobuf::Optional::new(self.execute(), self.has_execute())
  }
  pub fn execute(&self) -> super::action::ExecuteView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::action::ExecuteView::default())
  }
  pub fn execute_mut(&mut self) -> super::action::ExecuteMut<'_> {
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
  pub fn set_execute(&mut self,
    val: impl ::protobuf::IntoProxied<super::action::Execute>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // credit: optional message palm.rbac.v1.Action.Credit
  pub fn has_credit(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_credit(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn credit_opt(&self) -> ::protobuf::Optional<super::action::CreditView<'_>> {
        ::protobuf::Optional::new(self.credit(), self.has_credit())
  }
  pub fn credit(&self) -> super::action::CreditView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::action::CreditView::default())
  }
  pub fn credit_mut(&mut self) -> super::action::CreditMut<'_> {
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
  pub fn set_credit(&mut self,
    val: impl ::protobuf::IntoProxied<super::action::Credit>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // debit: optional message palm.rbac.v1.Action.Debit
  pub fn has_debit(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_debit(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn debit_opt(&self) -> ::protobuf::Optional<super::action::DebitView<'_>> {
        ::protobuf::Optional::new(self.debit(), self.has_debit())
  }
  pub fn debit(&self) -> super::action::DebitView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::action::DebitView::default())
  }
  pub fn debit_mut(&mut self) -> super::action::DebitMut<'_> {
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
  pub fn set_debit(&mut self,
    val: impl ::protobuf::IntoProxied<super::action::Debit>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // inquiry: optional message palm.rbac.v1.Action.Inquiry
  pub fn has_inquiry(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_inquiry(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn inquiry_opt(&self) -> ::protobuf::Optional<super::action::InquiryView<'_>> {
        ::protobuf::Optional::new(self.inquiry(), self.has_inquiry())
  }
  pub fn inquiry(&self) -> super::action::InquiryView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::action::InquiryView::default())
  }
  pub fn inquiry_mut(&mut self) -> super::action::InquiryMut<'_> {
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
  pub fn set_inquiry(&mut self,
    val: impl ::protobuf::IntoProxied<super::action::Inquiry>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // code: optional string
  pub fn has_code(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_code(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn code_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.code(), self.has_code())
  }
  pub fn code(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_code(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val);
    }
  }

  pub fn by(&self) -> super::action::ByOneof<'_> {
    match &self.by_case() {
      super::action::ByCase::Read =>
          super::action::ByOneof::Read(self.read()),
      super::action::ByCase::Write =>
          super::action::ByOneof::Write(self.write()),
      super::action::ByCase::Append =>
          super::action::ByOneof::Append(self.append()),
      super::action::ByCase::Execute =>
          super::action::ByOneof::Execute(self.execute()),
      super::action::ByCase::Credit =>
          super::action::ByOneof::Credit(self.credit()),
      super::action::ByCase::Debit =>
          super::action::ByOneof::Debit(self.debit()),
      super::action::ByCase::Inquiry =>
          super::action::ByOneof::Inquiry(self.inquiry()),
      super::action::ByCase::Code =>
          super::action::ByOneof::Code(self.code()),
      _ => super::action::ByOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn by_case(&self) -> super::action::ByCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::action::ByCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ActionMut` does not perform any shared mutation.
unsafe impl Send for ActionMut<'_> {}

// SAFETY:
// - `ActionMut` does not perform any shared mutation.
unsafe impl Sync for ActionMut<'_> {}

impl<'msg> ::protobuf::AsView for ActionMut<'msg> {
  type Proxied = Action;
  fn as_view(&self) -> ::protobuf::View<'_, Action> {
    ActionView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ActionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Action>
  where
      'msg: 'shorter {
    ActionView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ActionMut<'msg> {
  type MutProxied = Action;
  fn as_mut(&mut self) -> ActionMut<'msg> {
    ActionMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ActionMut<'msg> {
  fn into_mut<'shorter>(self) -> ActionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Action {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Action> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ActionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ActionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // read: optional message palm.rbac.v1.Action.Read
  pub fn has_read(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_read(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn read_opt(&self) -> ::protobuf::Optional<super::action::ReadView<'_>> {
        ::protobuf::Optional::new(self.read(), self.has_read())
  }
  pub fn read(&self) -> super::action::ReadView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::action::ReadView::default())
  }
  pub fn read_mut(&mut self) -> super::action::ReadMut<'_> {
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
  pub fn set_read(&mut self,
    val: impl ::protobuf::IntoProxied<super::action::Read>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // write: optional message palm.rbac.v1.Action.Write
  pub fn has_write(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_write(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn write_opt(&self) -> ::protobuf::Optional<super::action::WriteView<'_>> {
        ::protobuf::Optional::new(self.write(), self.has_write())
  }
  pub fn write(&self) -> super::action::WriteView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::action::WriteView::default())
  }
  pub fn write_mut(&mut self) -> super::action::WriteMut<'_> {
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
  pub fn set_write(&mut self,
    val: impl ::protobuf::IntoProxied<super::action::Write>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // append: optional message palm.rbac.v1.Action.Append
  pub fn has_append(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_append(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn append_opt(&self) -> ::protobuf::Optional<super::action::AppendView<'_>> {
        ::protobuf::Optional::new(self.append(), self.has_append())
  }
  pub fn append(&self) -> super::action::AppendView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::action::AppendView::default())
  }
  pub fn append_mut(&mut self) -> super::action::AppendMut<'_> {
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
  pub fn set_append(&mut self,
    val: impl ::protobuf::IntoProxied<super::action::Append>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // execute: optional message palm.rbac.v1.Action.Execute
  pub fn has_execute(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_execute(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn execute_opt(&self) -> ::protobuf::Optional<super::action::ExecuteView<'_>> {
        ::protobuf::Optional::new(self.execute(), self.has_execute())
  }
  pub fn execute(&self) -> super::action::ExecuteView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::action::ExecuteView::default())
  }
  pub fn execute_mut(&mut self) -> super::action::ExecuteMut<'_> {
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
  pub fn set_execute(&mut self,
    val: impl ::protobuf::IntoProxied<super::action::Execute>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // credit: optional message palm.rbac.v1.Action.Credit
  pub fn has_credit(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_credit(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn credit_opt(&self) -> ::protobuf::Optional<super::action::CreditView<'_>> {
        ::protobuf::Optional::new(self.credit(), self.has_credit())
  }
  pub fn credit(&self) -> super::action::CreditView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::action::CreditView::default())
  }
  pub fn credit_mut(&mut self) -> super::action::CreditMut<'_> {
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
  pub fn set_credit(&mut self,
    val: impl ::protobuf::IntoProxied<super::action::Credit>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // debit: optional message palm.rbac.v1.Action.Debit
  pub fn has_debit(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_debit(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn debit_opt(&self) -> ::protobuf::Optional<super::action::DebitView<'_>> {
        ::protobuf::Optional::new(self.debit(), self.has_debit())
  }
  pub fn debit(&self) -> super::action::DebitView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::action::DebitView::default())
  }
  pub fn debit_mut(&mut self) -> super::action::DebitMut<'_> {
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
  pub fn set_debit(&mut self,
    val: impl ::protobuf::IntoProxied<super::action::Debit>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // inquiry: optional message palm.rbac.v1.Action.Inquiry
  pub fn has_inquiry(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_inquiry(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn inquiry_opt(&self) -> ::protobuf::Optional<super::action::InquiryView<'_>> {
        ::protobuf::Optional::new(self.inquiry(), self.has_inquiry())
  }
  pub fn inquiry(&self) -> super::action::InquiryView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::action::InquiryView::default())
  }
  pub fn inquiry_mut(&mut self) -> super::action::InquiryMut<'_> {
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
  pub fn set_inquiry(&mut self,
    val: impl ::protobuf::IntoProxied<super::action::Inquiry>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // code: optional string
  pub fn has_code(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_code(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn code_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.code(), self.has_code())
  }
  pub fn code(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_code(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val);
    }
  }

  pub fn by(&self) -> super::action::ByOneof<'_> {
    match &self.by_case() {
      super::action::ByCase::Read =>
          super::action::ByOneof::Read(self.read()),
      super::action::ByCase::Write =>
          super::action::ByOneof::Write(self.write()),
      super::action::ByCase::Append =>
          super::action::ByOneof::Append(self.append()),
      super::action::ByCase::Execute =>
          super::action::ByOneof::Execute(self.execute()),
      super::action::ByCase::Credit =>
          super::action::ByOneof::Credit(self.credit()),
      super::action::ByCase::Debit =>
          super::action::ByOneof::Debit(self.debit()),
      super::action::ByCase::Inquiry =>
          super::action::ByOneof::Inquiry(self.inquiry()),
      super::action::ByCase::Code =>
          super::action::ByOneof::Code(self.code()),
      _ => super::action::ByOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn by_case(&self) -> super::action::ByCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::action::ByCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Action

impl ::std::ops::Drop for Action {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Action {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Action {
  type Proxied = Self;
  fn as_view(&self) -> ActionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Action {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ActionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Action {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__rbac__v1__Action_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3333333a1T^!|#|$|%|&|(|)|+");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__rbac__v1__Action_msg_init.0, &[<super::action::Read as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::action::Write as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::action::Append as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::action::Execute as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::action::Credit as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::action::Debit as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::action::Inquiry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__rbac__v1__Action_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Action {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Action {
  type Msg = Action;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Action> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Action {
  type Msg = Action;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Action> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ActionMut<'_> {
  type Msg = Action;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Action> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ActionMut<'_> {
  type Msg = Action;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Action> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ActionView<'_> {
  type Msg = Action;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Action> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ActionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod action {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__rbac__v1__Action__Read_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Read {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Read>
}

impl ::protobuf::Message for Read {}

impl ::std::default::Default for Read {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Read {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Read` is `Sync` because it does not implement interior mutability.
//    Neither does `ReadMut`.
unsafe impl Sync for Read {}

// SAFETY:
// - `Read` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Read {}

impl ::protobuf::Proxied for Read {
  type View<'msg> = ReadView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Read {}

impl ::protobuf::MutProxied for Read {
  type Mut<'msg> = ReadMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ReadView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Read>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ReadView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ReadView<'msg> {
  type Message = Read;
}

impl ::std::fmt::Debug for ReadView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ReadView<'_> {
  fn default() -> ReadView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Read>> for ReadView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Read>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ReadView<'msg> {

  pub fn to_owned(&self) -> Read {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `ReadView` is `Sync` because it does not support mutation.
unsafe impl Sync for ReadView<'_> {}

// SAFETY:
// - `ReadView` is `Send` because while its alive a `ReadMut` cannot.
// - `ReadView` does not use thread-local data.
unsafe impl Send for ReadView<'_> {}

impl<'msg> ::protobuf::AsView for ReadView<'msg> {
  type Proxied = Read;
  fn as_view(&self) -> ::protobuf::View<'msg, Read> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ReadView<'msg> {
  fn into_view<'shorter>(self) -> ReadView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Read> for ReadView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Read {
    let mut dst = Read::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Read> for ReadMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Read {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Read {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ReadView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ReadMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ReadMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Read>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ReadMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ReadMut<'msg> {
  type Message = Read;
}

impl ::std::fmt::Debug for ReadMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Read>> for ReadMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Read>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ReadMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Read> {
    self.inner
  }

  pub fn to_owned(&self) -> Read {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `ReadMut` does not perform any shared mutation.
unsafe impl Send for ReadMut<'_> {}

// SAFETY:
// - `ReadMut` does not perform any shared mutation.
unsafe impl Sync for ReadMut<'_> {}

impl<'msg> ::protobuf::AsView for ReadMut<'msg> {
  type Proxied = Read;
  fn as_view(&self) -> ::protobuf::View<'_, Read> {
    ReadView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ReadMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Read>
  where
      'msg: 'shorter {
    ReadView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ReadMut<'msg> {
  type MutProxied = Read;
  fn as_mut(&mut self) -> ReadMut<'msg> {
    ReadMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ReadMut<'msg> {
  fn into_mut<'shorter>(self) -> ReadMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Read {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Read> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ReadView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ReadMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl Read

impl ::std::ops::Drop for Read {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Read {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Read {
  type Proxied = Self;
  fn as_view(&self) -> ReadView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Read {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ReadMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Read {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::action::palm__rbac__v1__Action__Read_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::action::palm__rbac__v1__Action__Read_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::action::palm__rbac__v1__Action__Read_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Read {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Read {
  type Msg = Read;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Read> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Read {
  type Msg = Read;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Read> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ReadMut<'_> {
  type Msg = Read;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Read> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ReadMut<'_> {
  type Msg = Read;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Read> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ReadView<'_> {
  type Msg = Read;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Read> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ReadMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__rbac__v1__Action__Write_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Write {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Write>
}

impl ::protobuf::Message for Write {}

impl ::std::default::Default for Write {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Write {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Write` is `Sync` because it does not implement interior mutability.
//    Neither does `WriteMut`.
unsafe impl Sync for Write {}

// SAFETY:
// - `Write` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Write {}

impl ::protobuf::Proxied for Write {
  type View<'msg> = WriteView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Write {}

impl ::protobuf::MutProxied for Write {
  type Mut<'msg> = WriteMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct WriteView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Write>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for WriteView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for WriteView<'msg> {
  type Message = Write;
}

impl ::std::fmt::Debug for WriteView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for WriteView<'_> {
  fn default() -> WriteView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Write>> for WriteView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Write>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> WriteView<'msg> {

  pub fn to_owned(&self) -> Write {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `WriteView` is `Sync` because it does not support mutation.
unsafe impl Sync for WriteView<'_> {}

// SAFETY:
// - `WriteView` is `Send` because while its alive a `WriteMut` cannot.
// - `WriteView` does not use thread-local data.
unsafe impl Send for WriteView<'_> {}

impl<'msg> ::protobuf::AsView for WriteView<'msg> {
  type Proxied = Write;
  fn as_view(&self) -> ::protobuf::View<'msg, Write> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for WriteView<'msg> {
  fn into_view<'shorter>(self) -> WriteView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Write> for WriteView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Write {
    let mut dst = Write::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Write> for WriteMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Write {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Write {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for WriteView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for WriteMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct WriteMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Write>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for WriteMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for WriteMut<'msg> {
  type Message = Write;
}

impl ::std::fmt::Debug for WriteMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Write>> for WriteMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Write>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> WriteMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Write> {
    self.inner
  }

  pub fn to_owned(&self) -> Write {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `WriteMut` does not perform any shared mutation.
unsafe impl Send for WriteMut<'_> {}

// SAFETY:
// - `WriteMut` does not perform any shared mutation.
unsafe impl Sync for WriteMut<'_> {}

impl<'msg> ::protobuf::AsView for WriteMut<'msg> {
  type Proxied = Write;
  fn as_view(&self) -> ::protobuf::View<'_, Write> {
    WriteView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for WriteMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Write>
  where
      'msg: 'shorter {
    WriteView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for WriteMut<'msg> {
  type MutProxied = Write;
  fn as_mut(&mut self) -> WriteMut<'msg> {
    WriteMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for WriteMut<'msg> {
  fn into_mut<'shorter>(self) -> WriteMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Write {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Write> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> WriteView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> WriteMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl Write

impl ::std::ops::Drop for Write {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Write {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Write {
  type Proxied = Self;
  fn as_view(&self) -> WriteView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Write {
  type MutProxied = Self;
  fn as_mut(&mut self) -> WriteMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Write {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::action::palm__rbac__v1__Action__Write_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::action::palm__rbac__v1__Action__Write_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::action::palm__rbac__v1__Action__Write_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Write {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Write {
  type Msg = Write;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Write> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Write {
  type Msg = Write;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Write> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for WriteMut<'_> {
  type Msg = Write;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Write> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WriteMut<'_> {
  type Msg = Write;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Write> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WriteView<'_> {
  type Msg = Write;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Write> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for WriteMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__rbac__v1__Action__Append_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Append {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Append>
}

impl ::protobuf::Message for Append {}

impl ::std::default::Default for Append {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Append {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Append` is `Sync` because it does not implement interior mutability.
//    Neither does `AppendMut`.
unsafe impl Sync for Append {}

// SAFETY:
// - `Append` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Append {}

impl ::protobuf::Proxied for Append {
  type View<'msg> = AppendView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Append {}

impl ::protobuf::MutProxied for Append {
  type Mut<'msg> = AppendMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AppendView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Append>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AppendView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AppendView<'msg> {
  type Message = Append;
}

impl ::std::fmt::Debug for AppendView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AppendView<'_> {
  fn default() -> AppendView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Append>> for AppendView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Append>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AppendView<'msg> {

  pub fn to_owned(&self) -> Append {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `AppendView` is `Sync` because it does not support mutation.
unsafe impl Sync for AppendView<'_> {}

// SAFETY:
// - `AppendView` is `Send` because while its alive a `AppendMut` cannot.
// - `AppendView` does not use thread-local data.
unsafe impl Send for AppendView<'_> {}

impl<'msg> ::protobuf::AsView for AppendView<'msg> {
  type Proxied = Append;
  fn as_view(&self) -> ::protobuf::View<'msg, Append> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AppendView<'msg> {
  fn into_view<'shorter>(self) -> AppendView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Append> for AppendView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Append {
    let mut dst = Append::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Append> for AppendMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Append {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Append {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AppendView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AppendMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AppendMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Append>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AppendMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AppendMut<'msg> {
  type Message = Append;
}

impl ::std::fmt::Debug for AppendMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Append>> for AppendMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Append>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AppendMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Append> {
    self.inner
  }

  pub fn to_owned(&self) -> Append {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `AppendMut` does not perform any shared mutation.
unsafe impl Send for AppendMut<'_> {}

// SAFETY:
// - `AppendMut` does not perform any shared mutation.
unsafe impl Sync for AppendMut<'_> {}

impl<'msg> ::protobuf::AsView for AppendMut<'msg> {
  type Proxied = Append;
  fn as_view(&self) -> ::protobuf::View<'_, Append> {
    AppendView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AppendMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Append>
  where
      'msg: 'shorter {
    AppendView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for AppendMut<'msg> {
  type MutProxied = Append;
  fn as_mut(&mut self) -> AppendMut<'msg> {
    AppendMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AppendMut<'msg> {
  fn into_mut<'shorter>(self) -> AppendMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Append {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Append> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AppendView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AppendMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl Append

impl ::std::ops::Drop for Append {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Append {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Append {
  type Proxied = Self;
  fn as_view(&self) -> AppendView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Append {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AppendMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Append {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::action::palm__rbac__v1__Action__Append_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::action::palm__rbac__v1__Action__Append_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::action::palm__rbac__v1__Action__Append_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Append {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Append {
  type Msg = Append;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Append> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Append {
  type Msg = Append;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Append> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AppendMut<'_> {
  type Msg = Append;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Append> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AppendMut<'_> {
  type Msg = Append;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Append> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AppendView<'_> {
  type Msg = Append;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Append> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AppendMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__rbac__v1__Action__Execute_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Execute {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Execute>
}

impl ::protobuf::Message for Execute {}

impl ::std::default::Default for Execute {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Execute {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Execute` is `Sync` because it does not implement interior mutability.
//    Neither does `ExecuteMut`.
unsafe impl Sync for Execute {}

// SAFETY:
// - `Execute` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Execute {}

impl ::protobuf::Proxied for Execute {
  type View<'msg> = ExecuteView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Execute {}

impl ::protobuf::MutProxied for Execute {
  type Mut<'msg> = ExecuteMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExecuteView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Execute>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExecuteView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExecuteView<'msg> {
  type Message = Execute;
}

impl ::std::fmt::Debug for ExecuteView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExecuteView<'_> {
  fn default() -> ExecuteView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Execute>> for ExecuteView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Execute>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExecuteView<'msg> {

  pub fn to_owned(&self) -> Execute {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `ExecuteView` is `Sync` because it does not support mutation.
unsafe impl Sync for ExecuteView<'_> {}

// SAFETY:
// - `ExecuteView` is `Send` because while its alive a `ExecuteMut` cannot.
// - `ExecuteView` does not use thread-local data.
unsafe impl Send for ExecuteView<'_> {}

impl<'msg> ::protobuf::AsView for ExecuteView<'msg> {
  type Proxied = Execute;
  fn as_view(&self) -> ::protobuf::View<'msg, Execute> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExecuteView<'msg> {
  fn into_view<'shorter>(self) -> ExecuteView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Execute> for ExecuteView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Execute {
    let mut dst = Execute::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Execute> for ExecuteMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Execute {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Execute {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ExecuteView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ExecuteMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExecuteMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Execute>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExecuteMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExecuteMut<'msg> {
  type Message = Execute;
}

impl ::std::fmt::Debug for ExecuteMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Execute>> for ExecuteMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Execute>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExecuteMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Execute> {
    self.inner
  }

  pub fn to_owned(&self) -> Execute {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `ExecuteMut` does not perform any shared mutation.
unsafe impl Send for ExecuteMut<'_> {}

// SAFETY:
// - `ExecuteMut` does not perform any shared mutation.
unsafe impl Sync for ExecuteMut<'_> {}

impl<'msg> ::protobuf::AsView for ExecuteMut<'msg> {
  type Proxied = Execute;
  fn as_view(&self) -> ::protobuf::View<'_, Execute> {
    ExecuteView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExecuteMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Execute>
  where
      'msg: 'shorter {
    ExecuteView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ExecuteMut<'msg> {
  type MutProxied = Execute;
  fn as_mut(&mut self) -> ExecuteMut<'msg> {
    ExecuteMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExecuteMut<'msg> {
  fn into_mut<'shorter>(self) -> ExecuteMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Execute {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Execute> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExecuteView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExecuteMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl Execute

impl ::std::ops::Drop for Execute {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Execute {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Execute {
  type Proxied = Self;
  fn as_view(&self) -> ExecuteView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Execute {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExecuteMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Execute {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::action::palm__rbac__v1__Action__Execute_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::action::palm__rbac__v1__Action__Execute_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::action::palm__rbac__v1__Action__Execute_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Execute {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Execute {
  type Msg = Execute;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Execute> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Execute {
  type Msg = Execute;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Execute> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExecuteMut<'_> {
  type Msg = Execute;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Execute> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExecuteMut<'_> {
  type Msg = Execute;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Execute> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExecuteView<'_> {
  type Msg = Execute;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Execute> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExecuteMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__rbac__v1__Action__Credit_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Credit {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Credit>
}

impl ::protobuf::Message for Credit {}

impl ::std::default::Default for Credit {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Credit {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Credit` is `Sync` because it does not implement interior mutability.
//    Neither does `CreditMut`.
unsafe impl Sync for Credit {}

// SAFETY:
// - `Credit` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Credit {}

impl ::protobuf::Proxied for Credit {
  type View<'msg> = CreditView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Credit {}

impl ::protobuf::MutProxied for Credit {
  type Mut<'msg> = CreditMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CreditView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Credit>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CreditView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CreditView<'msg> {
  type Message = Credit;
}

impl ::std::fmt::Debug for CreditView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CreditView<'_> {
  fn default() -> CreditView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Credit>> for CreditView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Credit>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CreditView<'msg> {

  pub fn to_owned(&self) -> Credit {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `CreditView` is `Sync` because it does not support mutation.
unsafe impl Sync for CreditView<'_> {}

// SAFETY:
// - `CreditView` is `Send` because while its alive a `CreditMut` cannot.
// - `CreditView` does not use thread-local data.
unsafe impl Send for CreditView<'_> {}

impl<'msg> ::protobuf::AsView for CreditView<'msg> {
  type Proxied = Credit;
  fn as_view(&self) -> ::protobuf::View<'msg, Credit> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CreditView<'msg> {
  fn into_view<'shorter>(self) -> CreditView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Credit> for CreditView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Credit {
    let mut dst = Credit::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Credit> for CreditMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Credit {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Credit {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for CreditView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for CreditMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CreditMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Credit>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CreditMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CreditMut<'msg> {
  type Message = Credit;
}

impl ::std::fmt::Debug for CreditMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Credit>> for CreditMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Credit>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CreditMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Credit> {
    self.inner
  }

  pub fn to_owned(&self) -> Credit {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `CreditMut` does not perform any shared mutation.
unsafe impl Send for CreditMut<'_> {}

// SAFETY:
// - `CreditMut` does not perform any shared mutation.
unsafe impl Sync for CreditMut<'_> {}

impl<'msg> ::protobuf::AsView for CreditMut<'msg> {
  type Proxied = Credit;
  fn as_view(&self) -> ::protobuf::View<'_, Credit> {
    CreditView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CreditMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Credit>
  where
      'msg: 'shorter {
    CreditView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for CreditMut<'msg> {
  type MutProxied = Credit;
  fn as_mut(&mut self) -> CreditMut<'msg> {
    CreditMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CreditMut<'msg> {
  fn into_mut<'shorter>(self) -> CreditMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Credit {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Credit> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CreditView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CreditMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl Credit

impl ::std::ops::Drop for Credit {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Credit {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Credit {
  type Proxied = Self;
  fn as_view(&self) -> CreditView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Credit {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CreditMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Credit {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::action::palm__rbac__v1__Action__Credit_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::action::palm__rbac__v1__Action__Credit_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::action::palm__rbac__v1__Action__Credit_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Credit {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Credit {
  type Msg = Credit;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Credit> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Credit {
  type Msg = Credit;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Credit> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CreditMut<'_> {
  type Msg = Credit;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Credit> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CreditMut<'_> {
  type Msg = Credit;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Credit> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CreditView<'_> {
  type Msg = Credit;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Credit> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CreditMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__rbac__v1__Action__Debit_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Debit {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Debit>
}

impl ::protobuf::Message for Debit {}

impl ::std::default::Default for Debit {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Debit {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Debit` is `Sync` because it does not implement interior mutability.
//    Neither does `DebitMut`.
unsafe impl Sync for Debit {}

// SAFETY:
// - `Debit` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Debit {}

impl ::protobuf::Proxied for Debit {
  type View<'msg> = DebitView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Debit {}

impl ::protobuf::MutProxied for Debit {
  type Mut<'msg> = DebitMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DebitView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Debit>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DebitView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DebitView<'msg> {
  type Message = Debit;
}

impl ::std::fmt::Debug for DebitView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DebitView<'_> {
  fn default() -> DebitView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Debit>> for DebitView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Debit>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DebitView<'msg> {

  pub fn to_owned(&self) -> Debit {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `DebitView` is `Sync` because it does not support mutation.
unsafe impl Sync for DebitView<'_> {}

// SAFETY:
// - `DebitView` is `Send` because while its alive a `DebitMut` cannot.
// - `DebitView` does not use thread-local data.
unsafe impl Send for DebitView<'_> {}

impl<'msg> ::protobuf::AsView for DebitView<'msg> {
  type Proxied = Debit;
  fn as_view(&self) -> ::protobuf::View<'msg, Debit> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DebitView<'msg> {
  fn into_view<'shorter>(self) -> DebitView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Debit> for DebitView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Debit {
    let mut dst = Debit::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Debit> for DebitMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Debit {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Debit {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DebitView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DebitMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DebitMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Debit>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DebitMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DebitMut<'msg> {
  type Message = Debit;
}

impl ::std::fmt::Debug for DebitMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Debit>> for DebitMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Debit>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DebitMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Debit> {
    self.inner
  }

  pub fn to_owned(&self) -> Debit {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `DebitMut` does not perform any shared mutation.
unsafe impl Send for DebitMut<'_> {}

// SAFETY:
// - `DebitMut` does not perform any shared mutation.
unsafe impl Sync for DebitMut<'_> {}

impl<'msg> ::protobuf::AsView for DebitMut<'msg> {
  type Proxied = Debit;
  fn as_view(&self) -> ::protobuf::View<'_, Debit> {
    DebitView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DebitMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Debit>
  where
      'msg: 'shorter {
    DebitView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for DebitMut<'msg> {
  type MutProxied = Debit;
  fn as_mut(&mut self) -> DebitMut<'msg> {
    DebitMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DebitMut<'msg> {
  fn into_mut<'shorter>(self) -> DebitMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Debit {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Debit> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DebitView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DebitMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl Debit

impl ::std::ops::Drop for Debit {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Debit {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Debit {
  type Proxied = Self;
  fn as_view(&self) -> DebitView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Debit {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DebitMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Debit {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::action::palm__rbac__v1__Action__Debit_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::action::palm__rbac__v1__Action__Debit_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::action::palm__rbac__v1__Action__Debit_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Debit {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Debit {
  type Msg = Debit;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Debit> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Debit {
  type Msg = Debit;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Debit> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DebitMut<'_> {
  type Msg = Debit;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Debit> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DebitMut<'_> {
  type Msg = Debit;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Debit> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DebitView<'_> {
  type Msg = Debit;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Debit> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DebitMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__rbac__v1__Action__Inquiry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Inquiry {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Inquiry>
}

impl ::protobuf::Message for Inquiry {}

impl ::std::default::Default for Inquiry {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Inquiry {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Inquiry` is `Sync` because it does not implement interior mutability.
//    Neither does `InquiryMut`.
unsafe impl Sync for Inquiry {}

// SAFETY:
// - `Inquiry` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Inquiry {}

impl ::protobuf::Proxied for Inquiry {
  type View<'msg> = InquiryView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Inquiry {}

impl ::protobuf::MutProxied for Inquiry {
  type Mut<'msg> = InquiryMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct InquiryView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Inquiry>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for InquiryView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for InquiryView<'msg> {
  type Message = Inquiry;
}

impl ::std::fmt::Debug for InquiryView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for InquiryView<'_> {
  fn default() -> InquiryView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Inquiry>> for InquiryView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Inquiry>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> InquiryView<'msg> {

  pub fn to_owned(&self) -> Inquiry {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `InquiryView` is `Sync` because it does not support mutation.
unsafe impl Sync for InquiryView<'_> {}

// SAFETY:
// - `InquiryView` is `Send` because while its alive a `InquiryMut` cannot.
// - `InquiryView` does not use thread-local data.
unsafe impl Send for InquiryView<'_> {}

impl<'msg> ::protobuf::AsView for InquiryView<'msg> {
  type Proxied = Inquiry;
  fn as_view(&self) -> ::protobuf::View<'msg, Inquiry> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for InquiryView<'msg> {
  fn into_view<'shorter>(self) -> InquiryView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Inquiry> for InquiryView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Inquiry {
    let mut dst = Inquiry::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Inquiry> for InquiryMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Inquiry {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Inquiry {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for InquiryView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for InquiryMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct InquiryMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Inquiry>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for InquiryMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for InquiryMut<'msg> {
  type Message = Inquiry;
}

impl ::std::fmt::Debug for InquiryMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Inquiry>> for InquiryMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Inquiry>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> InquiryMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Inquiry> {
    self.inner
  }

  pub fn to_owned(&self) -> Inquiry {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `InquiryMut` does not perform any shared mutation.
unsafe impl Send for InquiryMut<'_> {}

// SAFETY:
// - `InquiryMut` does not perform any shared mutation.
unsafe impl Sync for InquiryMut<'_> {}

impl<'msg> ::protobuf::AsView for InquiryMut<'msg> {
  type Proxied = Inquiry;
  fn as_view(&self) -> ::protobuf::View<'_, Inquiry> {
    InquiryView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for InquiryMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Inquiry>
  where
      'msg: 'shorter {
    InquiryView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for InquiryMut<'msg> {
  type MutProxied = Inquiry;
  fn as_mut(&mut self) -> InquiryMut<'msg> {
    InquiryMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for InquiryMut<'msg> {
  fn into_mut<'shorter>(self) -> InquiryMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Inquiry {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Inquiry> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> InquiryView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> InquiryMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl Inquiry

impl ::std::ops::Drop for Inquiry {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Inquiry {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Inquiry {
  type Proxied = Self;
  fn as_view(&self) -> InquiryView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Inquiry {
  type MutProxied = Self;
  fn as_mut(&mut self) -> InquiryMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Inquiry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::action::palm__rbac__v1__Action__Inquiry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::action::palm__rbac__v1__Action__Inquiry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::action::palm__rbac__v1__Action__Inquiry_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Inquiry {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Inquiry {
  type Msg = Inquiry;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Inquiry> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Inquiry {
  type Msg = Inquiry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Inquiry> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for InquiryMut<'_> {
  type Msg = Inquiry;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Inquiry> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for InquiryMut<'_> {
  type Msg = Inquiry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Inquiry> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for InquiryView<'_> {
  type Msg = Inquiry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Inquiry> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for InquiryMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ByOneof<'msg> {
  Read(::protobuf::View<'msg, super::super::action::Read>) = 1,
  Write(::protobuf::View<'msg, super::super::action::Write>) = 2,
  Append(::protobuf::View<'msg, super::super::action::Append>) = 3,
  Execute(::protobuf::View<'msg, super::super::action::Execute>) = 4,
  Credit(::protobuf::View<'msg, super::super::action::Credit>) = 5,
  Debit(::protobuf::View<'msg, super::super::action::Debit>) = 6,
  Inquiry(::protobuf::View<'msg, super::super::action::Inquiry>) = 7,
  Code(&'msg ::protobuf::ProtoStr) = 9,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ByCase {
  Read = 1,
  Write = 2,
  Append = 3,
  Execute = 4,
  Credit = 5,
  Debit = 6,
  Inquiry = 7,
  Code = 9,

  not_set = 0
}

impl ByCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ByCase> {
    match v {
      0 => Some(ByCase::not_set),
      1 => Some(ByCase::Read),
      2 => Some(ByCase::Write),
      3 => Some(ByCase::Append),
      4 => Some(ByCase::Execute),
      5 => Some(ByCase::Credit),
      6 => Some(ByCase::Debit),
      7 => Some(ByCase::Inquiry),
      9 => Some(ByCase::Code),
      _ => None
    }
  }
}
}  // pub mod action


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__rbac__v1__Permission_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Permission {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Permission>
}

impl ::protobuf::Message for Permission {}

impl ::std::default::Default for Permission {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Permission {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Permission` is `Sync` because it does not implement interior mutability.
//    Neither does `PermissionMut`.
unsafe impl Sync for Permission {}

// SAFETY:
// - `Permission` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Permission {}

impl ::protobuf::Proxied for Permission {
  type View<'msg> = PermissionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Permission {}

impl ::protobuf::MutProxied for Permission {
  type Mut<'msg> = PermissionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PermissionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Permission>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PermissionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PermissionView<'msg> {
  type Message = Permission;
}

impl ::std::fmt::Debug for PermissionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PermissionView<'_> {
  fn default() -> PermissionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Permission>> for PermissionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Permission>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PermissionView<'msg> {

  pub fn to_owned(&self) -> Permission {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // subject: optional message palm.rbac.v1.Subject
  pub fn has_subject(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn subject_opt(self) -> ::protobuf::Optional<super::SubjectView<'msg>> {
        ::protobuf::Optional::new(self.subject(), self.has_subject())
  }
  pub fn subject(self) -> super::SubjectView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SubjectView::default())
  }

  // object: optional message palm.rbac.v1.Object
  pub fn has_object(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn object_opt(self) -> ::protobuf::Optional<super::ObjectView<'msg>> {
        ::protobuf::Optional::new(self.object(), self.has_object())
  }
  pub fn object(self) -> super::ObjectView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ObjectView::default())
  }

  // action: optional message palm.rbac.v1.Action
  pub fn has_action(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn action_opt(self) -> ::protobuf::Optional<super::ActionView<'msg>> {
        ::protobuf::Optional::new(self.action(), self.has_action())
  }
  pub fn action(self) -> super::ActionView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ActionView::default())
  }

}

// SAFETY:
// - `PermissionView` is `Sync` because it does not support mutation.
unsafe impl Sync for PermissionView<'_> {}

// SAFETY:
// - `PermissionView` is `Send` because while its alive a `PermissionMut` cannot.
// - `PermissionView` does not use thread-local data.
unsafe impl Send for PermissionView<'_> {}

impl<'msg> ::protobuf::AsView for PermissionView<'msg> {
  type Proxied = Permission;
  fn as_view(&self) -> ::protobuf::View<'msg, Permission> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PermissionView<'msg> {
  fn into_view<'shorter>(self) -> PermissionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Permission> for PermissionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Permission {
    let mut dst = Permission::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Permission> for PermissionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Permission {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Permission {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PermissionView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PermissionMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PermissionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Permission>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PermissionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PermissionMut<'msg> {
  type Message = Permission;
}

impl ::std::fmt::Debug for PermissionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Permission>> for PermissionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Permission>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PermissionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Permission> {
    self.inner
  }

  pub fn to_owned(&self) -> Permission {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // subject: optional message palm.rbac.v1.Subject
  pub fn has_subject(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_subject(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn subject_opt(&self) -> ::protobuf::Optional<super::SubjectView<'_>> {
        ::protobuf::Optional::new(self.subject(), self.has_subject())
  }
  pub fn subject(&self) -> super::SubjectView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SubjectView::default())
  }
  pub fn subject_mut(&mut self) -> super::SubjectMut<'_> {
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
  pub fn set_subject(&mut self,
    val: impl ::protobuf::IntoProxied<super::Subject>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // object: optional message palm.rbac.v1.Object
  pub fn has_object(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_object(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn object_opt(&self) -> ::protobuf::Optional<super::ObjectView<'_>> {
        ::protobuf::Optional::new(self.object(), self.has_object())
  }
  pub fn object(&self) -> super::ObjectView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ObjectView::default())
  }
  pub fn object_mut(&mut self) -> super::ObjectMut<'_> {
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
  pub fn set_object(&mut self,
    val: impl ::protobuf::IntoProxied<super::Object>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // action: optional message palm.rbac.v1.Action
  pub fn has_action(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_action(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn action_opt(&self) -> ::protobuf::Optional<super::ActionView<'_>> {
        ::protobuf::Optional::new(self.action(), self.has_action())
  }
  pub fn action(&self) -> super::ActionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ActionView::default())
  }
  pub fn action_mut(&mut self) -> super::ActionMut<'_> {
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
  pub fn set_action(&mut self,
    val: impl ::protobuf::IntoProxied<super::Action>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}

// SAFETY:
// - `PermissionMut` does not perform any shared mutation.
unsafe impl Send for PermissionMut<'_> {}

// SAFETY:
// - `PermissionMut` does not perform any shared mutation.
unsafe impl Sync for PermissionMut<'_> {}

impl<'msg> ::protobuf::AsView for PermissionMut<'msg> {
  type Proxied = Permission;
  fn as_view(&self) -> ::protobuf::View<'_, Permission> {
    PermissionView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PermissionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Permission>
  where
      'msg: 'shorter {
    PermissionView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for PermissionMut<'msg> {
  type MutProxied = Permission;
  fn as_mut(&mut self) -> PermissionMut<'msg> {
    PermissionMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PermissionMut<'msg> {
  fn into_mut<'shorter>(self) -> PermissionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Permission {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Permission> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PermissionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PermissionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // subject: optional message palm.rbac.v1.Subject
  pub fn has_subject(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_subject(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn subject_opt(&self) -> ::protobuf::Optional<super::SubjectView<'_>> {
        ::protobuf::Optional::new(self.subject(), self.has_subject())
  }
  pub fn subject(&self) -> super::SubjectView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SubjectView::default())
  }
  pub fn subject_mut(&mut self) -> super::SubjectMut<'_> {
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
  pub fn set_subject(&mut self,
    val: impl ::protobuf::IntoProxied<super::Subject>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // object: optional message palm.rbac.v1.Object
  pub fn has_object(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_object(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn object_opt(&self) -> ::protobuf::Optional<super::ObjectView<'_>> {
        ::protobuf::Optional::new(self.object(), self.has_object())
  }
  pub fn object(&self) -> super::ObjectView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ObjectView::default())
  }
  pub fn object_mut(&mut self) -> super::ObjectMut<'_> {
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
  pub fn set_object(&mut self,
    val: impl ::protobuf::IntoProxied<super::Object>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // action: optional message palm.rbac.v1.Action
  pub fn has_action(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_action(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn action_opt(&self) -> ::protobuf::Optional<super::ActionView<'_>> {
        ::protobuf::Optional::new(self.action(), self.has_action())
  }
  pub fn action(&self) -> super::ActionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ActionView::default())
  }
  pub fn action_mut(&mut self) -> super::ActionMut<'_> {
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
  pub fn set_action(&mut self,
    val: impl ::protobuf::IntoProxied<super::Action>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl Permission

impl ::std::ops::Drop for Permission {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Permission {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Permission {
  type Proxied = Self;
  fn as_view(&self) -> PermissionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Permission {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PermissionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Permission {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__rbac__v1__Permission_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__rbac__v1__Permission_msg_init.0, &[<super::Subject as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Object as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Action as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__rbac__v1__Permission_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Permission {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Permission {
  type Msg = Permission;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Permission> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Permission {
  type Msg = Permission;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Permission> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PermissionMut<'_> {
  type Msg = Permission;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Permission> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PermissionMut<'_> {
  type Msg = Permission;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Permission> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PermissionView<'_> {
  type Msg = Permission;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Permission> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PermissionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__rbac__v1__UserRoleRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
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

  // user: optional message palm.rbac.v1.Subject.User
  pub fn has_user(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn user_opt(self) -> ::protobuf::Optional<super::subject::UserView<'msg>> {
        ::protobuf::Optional::new(self.user(), self.has_user())
  }
  pub fn user(self) -> super::subject::UserView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::subject::UserView::default())
  }

  // role: optional message palm.rbac.v1.Subject.Role
  pub fn has_role(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn role_opt(self) -> ::protobuf::Optional<super::subject::RoleView<'msg>> {
        ::protobuf::Optional::new(self.role(), self.has_role())
  }
  pub fn role(self) -> super::subject::RoleView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::subject::RoleView::default())
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

  // user: optional message palm.rbac.v1.Subject.User
  pub fn has_user(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_user(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn user_opt(&self) -> ::protobuf::Optional<super::subject::UserView<'_>> {
        ::protobuf::Optional::new(self.user(), self.has_user())
  }
  pub fn user(&self) -> super::subject::UserView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::subject::UserView::default())
  }
  pub fn user_mut(&mut self) -> super::subject::UserMut<'_> {
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
  pub fn set_user(&mut self,
    val: impl ::protobuf::IntoProxied<super::subject::User>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // role: optional message palm.rbac.v1.Subject.Role
  pub fn has_role(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_role(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn role_opt(&self) -> ::protobuf::Optional<super::subject::RoleView<'_>> {
        ::protobuf::Optional::new(self.role(), self.has_role())
  }
  pub fn role(&self) -> super::subject::RoleView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::subject::RoleView::default())
  }
  pub fn role_mut(&mut self) -> super::subject::RoleMut<'_> {
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
  pub fn set_role(&mut self,
    val: impl ::protobuf::IntoProxied<super::subject::Role>) {

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

  // user: optional message palm.rbac.v1.Subject.User
  pub fn has_user(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_user(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn user_opt(&self) -> ::protobuf::Optional<super::subject::UserView<'_>> {
        ::protobuf::Optional::new(self.user(), self.has_user())
  }
  pub fn user(&self) -> super::subject::UserView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::subject::UserView::default())
  }
  pub fn user_mut(&mut self) -> super::subject::UserMut<'_> {
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
  pub fn set_user(&mut self,
    val: impl ::protobuf::IntoProxied<super::subject::User>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // role: optional message palm.rbac.v1.Subject.Role
  pub fn has_role(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_role(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn role_opt(&self) -> ::protobuf::Optional<super::subject::RoleView<'_>> {
        ::protobuf::Optional::new(self.role(), self.has_role())
  }
  pub fn role(&self) -> super::subject::RoleView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::subject::RoleView::default())
  }
  pub fn role_mut(&mut self) -> super::subject::RoleMut<'_> {
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
  pub fn set_role(&mut self,
    val: impl ::protobuf::IntoProxied<super::subject::Role>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
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
        super::palm__rbac__v1__UserRoleRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__rbac__v1__UserRoleRequest_msg_init.0, &[<super::subject::User as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::subject::Role as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__rbac__v1__UserRoleRequest_msg_init.0)
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
pub(crate) static mut palm__rbac__v1__RolesResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RolesResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RolesResponse>
}

impl ::protobuf::Message for RolesResponse {}

impl ::std::default::Default for RolesResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RolesResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RolesResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `RolesResponseMut`.
unsafe impl Sync for RolesResponse {}

// SAFETY:
// - `RolesResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for RolesResponse {}

impl ::protobuf::Proxied for RolesResponse {
  type View<'msg> = RolesResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RolesResponse {}

impl ::protobuf::MutProxied for RolesResponse {
  type Mut<'msg> = RolesResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RolesResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RolesResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RolesResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RolesResponseView<'msg> {
  type Message = RolesResponse;
}

impl ::std::fmt::Debug for RolesResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RolesResponseView<'_> {
  fn default() -> RolesResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RolesResponse>> for RolesResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RolesResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RolesResponseView<'msg> {

  pub fn to_owned(&self) -> RolesResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // items: repeated message palm.rbac.v1.Subject.Role
  pub fn items(self) -> ::protobuf::RepeatedView<'msg, super::subject::Role> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::subject::Role>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `RolesResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for RolesResponseView<'_> {}

// SAFETY:
// - `RolesResponseView` is `Send` because while its alive a `RolesResponseMut` cannot.
// - `RolesResponseView` does not use thread-local data.
unsafe impl Send for RolesResponseView<'_> {}

impl<'msg> ::protobuf::AsView for RolesResponseView<'msg> {
  type Proxied = RolesResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, RolesResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RolesResponseView<'msg> {
  fn into_view<'shorter>(self) -> RolesResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RolesResponse> for RolesResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RolesResponse {
    let mut dst = RolesResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RolesResponse> for RolesResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RolesResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for RolesResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for RolesResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for RolesResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RolesResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RolesResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RolesResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RolesResponseMut<'msg> {
  type Message = RolesResponse;
}

impl ::std::fmt::Debug for RolesResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RolesResponse>> for RolesResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RolesResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RolesResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RolesResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> RolesResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // items: repeated message palm.rbac.v1.Subject.Role
  pub fn items(&self) -> ::protobuf::RepeatedView<'_, super::subject::Role> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::subject::Role>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::subject::Role> {
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
  pub fn set_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::subject::Role>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `RolesResponseMut` does not perform any shared mutation.
unsafe impl Send for RolesResponseMut<'_> {}

// SAFETY:
// - `RolesResponseMut` does not perform any shared mutation.
unsafe impl Sync for RolesResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for RolesResponseMut<'msg> {
  type Proxied = RolesResponse;
  fn as_view(&self) -> ::protobuf::View<'_, RolesResponse> {
    RolesResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RolesResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RolesResponse>
  where
      'msg: 'shorter {
    RolesResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for RolesResponseMut<'msg> {
  type MutProxied = RolesResponse;
  fn as_mut(&mut self) -> RolesResponseMut<'msg> {
    RolesResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RolesResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> RolesResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RolesResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RolesResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RolesResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RolesResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // items: repeated message palm.rbac.v1.Subject.Role
  pub fn items(&self) -> ::protobuf::RepeatedView<'_, super::subject::Role> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::subject::Role>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::subject::Role> {
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
  pub fn set_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::subject::Role>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl RolesResponse

impl ::std::ops::Drop for RolesResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RolesResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RolesResponse {
  type Proxied = Self;
  fn as_view(&self) -> RolesResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RolesResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RolesResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RolesResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__rbac__v1__RolesResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__rbac__v1__RolesResponse_msg_init.0, &[<super::subject::Role as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__rbac__v1__RolesResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RolesResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RolesResponse {
  type Msg = RolesResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RolesResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RolesResponse {
  type Msg = RolesResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RolesResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RolesResponseMut<'_> {
  type Msg = RolesResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RolesResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RolesResponseMut<'_> {
  type Msg = RolesResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RolesResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RolesResponseView<'_> {
  type Msg = RolesResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RolesResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RolesResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__rbac__v1__UsersResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct UsersResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<UsersResponse>
}

impl ::protobuf::Message for UsersResponse {}

impl ::std::default::Default for UsersResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for UsersResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `UsersResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `UsersResponseMut`.
unsafe impl Sync for UsersResponse {}

// SAFETY:
// - `UsersResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for UsersResponse {}

impl ::protobuf::Proxied for UsersResponse {
  type View<'msg> = UsersResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for UsersResponse {}

impl ::protobuf::MutProxied for UsersResponse {
  type Mut<'msg> = UsersResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UsersResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UsersResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UsersResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UsersResponseView<'msg> {
  type Message = UsersResponse;
}

impl ::std::fmt::Debug for UsersResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UsersResponseView<'_> {
  fn default() -> UsersResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, UsersResponse>> for UsersResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UsersResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UsersResponseView<'msg> {

  pub fn to_owned(&self) -> UsersResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // items: repeated message palm.rbac.v1.Subject.User
  pub fn items(self) -> ::protobuf::RepeatedView<'msg, super::subject::User> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::subject::User>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `UsersResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for UsersResponseView<'_> {}

// SAFETY:
// - `UsersResponseView` is `Send` because while its alive a `UsersResponseMut` cannot.
// - `UsersResponseView` does not use thread-local data.
unsafe impl Send for UsersResponseView<'_> {}

impl<'msg> ::protobuf::AsView for UsersResponseView<'msg> {
  type Proxied = UsersResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, UsersResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UsersResponseView<'msg> {
  fn into_view<'shorter>(self) -> UsersResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<UsersResponse> for UsersResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UsersResponse {
    let mut dst = UsersResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<UsersResponse> for UsersResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UsersResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for UsersResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for UsersResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for UsersResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UsersResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UsersResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UsersResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UsersResponseMut<'msg> {
  type Message = UsersResponse;
}

impl ::std::fmt::Debug for UsersResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, UsersResponse>> for UsersResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UsersResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UsersResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, UsersResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> UsersResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // items: repeated message palm.rbac.v1.Subject.User
  pub fn items(&self) -> ::protobuf::RepeatedView<'_, super::subject::User> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::subject::User>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::subject::User> {
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
  pub fn set_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::subject::User>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `UsersResponseMut` does not perform any shared mutation.
unsafe impl Send for UsersResponseMut<'_> {}

// SAFETY:
// - `UsersResponseMut` does not perform any shared mutation.
unsafe impl Sync for UsersResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for UsersResponseMut<'msg> {
  type Proxied = UsersResponse;
  fn as_view(&self) -> ::protobuf::View<'_, UsersResponse> {
    UsersResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UsersResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, UsersResponse>
  where
      'msg: 'shorter {
    UsersResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for UsersResponseMut<'msg> {
  type MutProxied = UsersResponse;
  fn as_mut(&mut self) -> UsersResponseMut<'msg> {
    UsersResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UsersResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> UsersResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl UsersResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, UsersResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UsersResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UsersResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // items: repeated message palm.rbac.v1.Subject.User
  pub fn items(&self) -> ::protobuf::RepeatedView<'_, super::subject::User> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::subject::User>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::subject::User> {
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
  pub fn set_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::subject::User>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl UsersResponse

impl ::std::ops::Drop for UsersResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for UsersResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for UsersResponse {
  type Proxied = Self;
  fn as_view(&self) -> UsersResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for UsersResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UsersResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for UsersResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__rbac__v1__UsersResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__rbac__v1__UsersResponse_msg_init.0, &[<super::subject::User as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__rbac__v1__UsersResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UsersResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UsersResponse {
  type Msg = UsersResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UsersResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UsersResponse {
  type Msg = UsersResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UsersResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UsersResponseMut<'_> {
  type Msg = UsersResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UsersResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UsersResponseMut<'_> {
  type Msg = UsersResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UsersResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UsersResponseView<'_> {
  type Msg = UsersResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UsersResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UsersResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__rbac__v1__SubjectsResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SubjectsResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SubjectsResponse>
}

impl ::protobuf::Message for SubjectsResponse {}

impl ::std::default::Default for SubjectsResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SubjectsResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SubjectsResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `SubjectsResponseMut`.
unsafe impl Sync for SubjectsResponse {}

// SAFETY:
// - `SubjectsResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SubjectsResponse {}

impl ::protobuf::Proxied for SubjectsResponse {
  type View<'msg> = SubjectsResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SubjectsResponse {}

impl ::protobuf::MutProxied for SubjectsResponse {
  type Mut<'msg> = SubjectsResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SubjectsResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SubjectsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SubjectsResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SubjectsResponseView<'msg> {
  type Message = SubjectsResponse;
}

impl ::std::fmt::Debug for SubjectsResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SubjectsResponseView<'_> {
  fn default() -> SubjectsResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SubjectsResponse>> for SubjectsResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SubjectsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SubjectsResponseView<'msg> {

  pub fn to_owned(&self) -> SubjectsResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // items: repeated message palm.rbac.v1.Subject
  pub fn items(self) -> ::protobuf::RepeatedView<'msg, super::Subject> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Subject>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `SubjectsResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for SubjectsResponseView<'_> {}

// SAFETY:
// - `SubjectsResponseView` is `Send` because while its alive a `SubjectsResponseMut` cannot.
// - `SubjectsResponseView` does not use thread-local data.
unsafe impl Send for SubjectsResponseView<'_> {}

impl<'msg> ::protobuf::AsView for SubjectsResponseView<'msg> {
  type Proxied = SubjectsResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, SubjectsResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SubjectsResponseView<'msg> {
  fn into_view<'shorter>(self) -> SubjectsResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SubjectsResponse> for SubjectsResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SubjectsResponse {
    let mut dst = SubjectsResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SubjectsResponse> for SubjectsResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SubjectsResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for SubjectsResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SubjectsResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SubjectsResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SubjectsResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SubjectsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SubjectsResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SubjectsResponseMut<'msg> {
  type Message = SubjectsResponse;
}

impl ::std::fmt::Debug for SubjectsResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SubjectsResponse>> for SubjectsResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SubjectsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SubjectsResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SubjectsResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> SubjectsResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // items: repeated message palm.rbac.v1.Subject
  pub fn items(&self) -> ::protobuf::RepeatedView<'_, super::Subject> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Subject>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Subject> {
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
  pub fn set_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Subject>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `SubjectsResponseMut` does not perform any shared mutation.
unsafe impl Send for SubjectsResponseMut<'_> {}

// SAFETY:
// - `SubjectsResponseMut` does not perform any shared mutation.
unsafe impl Sync for SubjectsResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for SubjectsResponseMut<'msg> {
  type Proxied = SubjectsResponse;
  fn as_view(&self) -> ::protobuf::View<'_, SubjectsResponse> {
    SubjectsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SubjectsResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SubjectsResponse>
  where
      'msg: 'shorter {
    SubjectsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for SubjectsResponseMut<'msg> {
  type MutProxied = SubjectsResponse;
  fn as_mut(&mut self) -> SubjectsResponseMut<'msg> {
    SubjectsResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SubjectsResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> SubjectsResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SubjectsResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SubjectsResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SubjectsResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SubjectsResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // items: repeated message palm.rbac.v1.Subject
  pub fn items(&self) -> ::protobuf::RepeatedView<'_, super::Subject> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Subject>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Subject> {
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
  pub fn set_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Subject>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl SubjectsResponse

impl ::std::ops::Drop for SubjectsResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SubjectsResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SubjectsResponse {
  type Proxied = Self;
  fn as_view(&self) -> SubjectsResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SubjectsResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SubjectsResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SubjectsResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__rbac__v1__SubjectsResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__rbac__v1__SubjectsResponse_msg_init.0, &[<super::Subject as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__rbac__v1__SubjectsResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SubjectsResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SubjectsResponse {
  type Msg = SubjectsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SubjectsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SubjectsResponse {
  type Msg = SubjectsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SubjectsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SubjectsResponseMut<'_> {
  type Msg = SubjectsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SubjectsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SubjectsResponseMut<'_> {
  type Msg = SubjectsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SubjectsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SubjectsResponseView<'_> {
  type Msg = SubjectsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SubjectsResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SubjectsResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__rbac__v1__ObjectsResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ObjectsResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ObjectsResponse>
}

impl ::protobuf::Message for ObjectsResponse {}

impl ::std::default::Default for ObjectsResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ObjectsResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ObjectsResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `ObjectsResponseMut`.
unsafe impl Sync for ObjectsResponse {}

// SAFETY:
// - `ObjectsResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ObjectsResponse {}

impl ::protobuf::Proxied for ObjectsResponse {
  type View<'msg> = ObjectsResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ObjectsResponse {}

impl ::protobuf::MutProxied for ObjectsResponse {
  type Mut<'msg> = ObjectsResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ObjectsResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ObjectsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ObjectsResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ObjectsResponseView<'msg> {
  type Message = ObjectsResponse;
}

impl ::std::fmt::Debug for ObjectsResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ObjectsResponseView<'_> {
  fn default() -> ObjectsResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ObjectsResponse>> for ObjectsResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ObjectsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ObjectsResponseView<'msg> {

  pub fn to_owned(&self) -> ObjectsResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // items: repeated message palm.rbac.v1.Object
  pub fn items(self) -> ::protobuf::RepeatedView<'msg, super::Object> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Object>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ObjectsResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for ObjectsResponseView<'_> {}

// SAFETY:
// - `ObjectsResponseView` is `Send` because while its alive a `ObjectsResponseMut` cannot.
// - `ObjectsResponseView` does not use thread-local data.
unsafe impl Send for ObjectsResponseView<'_> {}

impl<'msg> ::protobuf::AsView for ObjectsResponseView<'msg> {
  type Proxied = ObjectsResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, ObjectsResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ObjectsResponseView<'msg> {
  fn into_view<'shorter>(self) -> ObjectsResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ObjectsResponse> for ObjectsResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ObjectsResponse {
    let mut dst = ObjectsResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ObjectsResponse> for ObjectsResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ObjectsResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ObjectsResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ObjectsResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ObjectsResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ObjectsResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ObjectsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ObjectsResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ObjectsResponseMut<'msg> {
  type Message = ObjectsResponse;
}

impl ::std::fmt::Debug for ObjectsResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ObjectsResponse>> for ObjectsResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ObjectsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ObjectsResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ObjectsResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> ObjectsResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // items: repeated message palm.rbac.v1.Object
  pub fn items(&self) -> ::protobuf::RepeatedView<'_, super::Object> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Object>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Object> {
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
  pub fn set_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Object>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `ObjectsResponseMut` does not perform any shared mutation.
unsafe impl Send for ObjectsResponseMut<'_> {}

// SAFETY:
// - `ObjectsResponseMut` does not perform any shared mutation.
unsafe impl Sync for ObjectsResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for ObjectsResponseMut<'msg> {
  type Proxied = ObjectsResponse;
  fn as_view(&self) -> ::protobuf::View<'_, ObjectsResponse> {
    ObjectsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ObjectsResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ObjectsResponse>
  where
      'msg: 'shorter {
    ObjectsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ObjectsResponseMut<'msg> {
  type MutProxied = ObjectsResponse;
  fn as_mut(&mut self) -> ObjectsResponseMut<'msg> {
    ObjectsResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ObjectsResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> ObjectsResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ObjectsResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ObjectsResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ObjectsResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ObjectsResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // items: repeated message palm.rbac.v1.Object
  pub fn items(&self) -> ::protobuf::RepeatedView<'_, super::Object> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Object>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Object> {
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
  pub fn set_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Object>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl ObjectsResponse

impl ::std::ops::Drop for ObjectsResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ObjectsResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ObjectsResponse {
  type Proxied = Self;
  fn as_view(&self) -> ObjectsResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ObjectsResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ObjectsResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ObjectsResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__rbac__v1__ObjectsResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__rbac__v1__ObjectsResponse_msg_init.0, &[<super::Object as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__rbac__v1__ObjectsResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ObjectsResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ObjectsResponse {
  type Msg = ObjectsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ObjectsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ObjectsResponse {
  type Msg = ObjectsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ObjectsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ObjectsResponseMut<'_> {
  type Msg = ObjectsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ObjectsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ObjectsResponseMut<'_> {
  type Msg = ObjectsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ObjectsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ObjectsResponseView<'_> {
  type Msg = ObjectsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ObjectsResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ObjectsResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__rbac__v1__ActionsResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ActionsResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ActionsResponse>
}

impl ::protobuf::Message for ActionsResponse {}

impl ::std::default::Default for ActionsResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ActionsResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ActionsResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `ActionsResponseMut`.
unsafe impl Sync for ActionsResponse {}

// SAFETY:
// - `ActionsResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ActionsResponse {}

impl ::protobuf::Proxied for ActionsResponse {
  type View<'msg> = ActionsResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ActionsResponse {}

impl ::protobuf::MutProxied for ActionsResponse {
  type Mut<'msg> = ActionsResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ActionsResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ActionsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ActionsResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ActionsResponseView<'msg> {
  type Message = ActionsResponse;
}

impl ::std::fmt::Debug for ActionsResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ActionsResponseView<'_> {
  fn default() -> ActionsResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ActionsResponse>> for ActionsResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ActionsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ActionsResponseView<'msg> {

  pub fn to_owned(&self) -> ActionsResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // items: repeated message palm.rbac.v1.Action
  pub fn items(self) -> ::protobuf::RepeatedView<'msg, super::Action> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Action>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ActionsResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for ActionsResponseView<'_> {}

// SAFETY:
// - `ActionsResponseView` is `Send` because while its alive a `ActionsResponseMut` cannot.
// - `ActionsResponseView` does not use thread-local data.
unsafe impl Send for ActionsResponseView<'_> {}

impl<'msg> ::protobuf::AsView for ActionsResponseView<'msg> {
  type Proxied = ActionsResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, ActionsResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ActionsResponseView<'msg> {
  fn into_view<'shorter>(self) -> ActionsResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ActionsResponse> for ActionsResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ActionsResponse {
    let mut dst = ActionsResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ActionsResponse> for ActionsResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ActionsResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ActionsResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ActionsResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ActionsResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ActionsResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ActionsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ActionsResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ActionsResponseMut<'msg> {
  type Message = ActionsResponse;
}

impl ::std::fmt::Debug for ActionsResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ActionsResponse>> for ActionsResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ActionsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ActionsResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ActionsResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> ActionsResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // items: repeated message palm.rbac.v1.Action
  pub fn items(&self) -> ::protobuf::RepeatedView<'_, super::Action> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Action>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Action> {
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
  pub fn set_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Action>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `ActionsResponseMut` does not perform any shared mutation.
unsafe impl Send for ActionsResponseMut<'_> {}

// SAFETY:
// - `ActionsResponseMut` does not perform any shared mutation.
unsafe impl Sync for ActionsResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for ActionsResponseMut<'msg> {
  type Proxied = ActionsResponse;
  fn as_view(&self) -> ::protobuf::View<'_, ActionsResponse> {
    ActionsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ActionsResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ActionsResponse>
  where
      'msg: 'shorter {
    ActionsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ActionsResponseMut<'msg> {
  type MutProxied = ActionsResponse;
  fn as_mut(&mut self) -> ActionsResponseMut<'msg> {
    ActionsResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ActionsResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> ActionsResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ActionsResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ActionsResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ActionsResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ActionsResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // items: repeated message palm.rbac.v1.Action
  pub fn items(&self) -> ::protobuf::RepeatedView<'_, super::Action> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Action>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Action> {
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
  pub fn set_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Action>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl ActionsResponse

impl ::std::ops::Drop for ActionsResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ActionsResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ActionsResponse {
  type Proxied = Self;
  fn as_view(&self) -> ActionsResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ActionsResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ActionsResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ActionsResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__rbac__v1__ActionsResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__rbac__v1__ActionsResponse_msg_init.0, &[<super::Action as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__rbac__v1__ActionsResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ActionsResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ActionsResponse {
  type Msg = ActionsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ActionsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ActionsResponse {
  type Msg = ActionsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ActionsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ActionsResponseMut<'_> {
  type Msg = ActionsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ActionsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ActionsResponseMut<'_> {
  type Msg = ActionsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ActionsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ActionsResponseView<'_> {
  type Msg = ActionsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ActionsResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ActionsResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut palm__rbac__v1__PermissionsResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PermissionsResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PermissionsResponse>
}

impl ::protobuf::Message for PermissionsResponse {}

impl ::std::default::Default for PermissionsResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PermissionsResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PermissionsResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `PermissionsResponseMut`.
unsafe impl Sync for PermissionsResponse {}

// SAFETY:
// - `PermissionsResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PermissionsResponse {}

impl ::protobuf::Proxied for PermissionsResponse {
  type View<'msg> = PermissionsResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PermissionsResponse {}

impl ::protobuf::MutProxied for PermissionsResponse {
  type Mut<'msg> = PermissionsResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PermissionsResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PermissionsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PermissionsResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PermissionsResponseView<'msg> {
  type Message = PermissionsResponse;
}

impl ::std::fmt::Debug for PermissionsResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PermissionsResponseView<'_> {
  fn default() -> PermissionsResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PermissionsResponse>> for PermissionsResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PermissionsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PermissionsResponseView<'msg> {

  pub fn to_owned(&self) -> PermissionsResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // items: repeated message palm.rbac.v1.Permission
  pub fn items(self) -> ::protobuf::RepeatedView<'msg, super::Permission> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Permission>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `PermissionsResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for PermissionsResponseView<'_> {}

// SAFETY:
// - `PermissionsResponseView` is `Send` because while its alive a `PermissionsResponseMut` cannot.
// - `PermissionsResponseView` does not use thread-local data.
unsafe impl Send for PermissionsResponseView<'_> {}

impl<'msg> ::protobuf::AsView for PermissionsResponseView<'msg> {
  type Proxied = PermissionsResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, PermissionsResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PermissionsResponseView<'msg> {
  fn into_view<'shorter>(self) -> PermissionsResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PermissionsResponse> for PermissionsResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PermissionsResponse {
    let mut dst = PermissionsResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PermissionsResponse> for PermissionsResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PermissionsResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for PermissionsResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PermissionsResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PermissionsResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PermissionsResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PermissionsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PermissionsResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PermissionsResponseMut<'msg> {
  type Message = PermissionsResponse;
}

impl ::std::fmt::Debug for PermissionsResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PermissionsResponse>> for PermissionsResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PermissionsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PermissionsResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PermissionsResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> PermissionsResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // items: repeated message palm.rbac.v1.Permission
  pub fn items(&self) -> ::protobuf::RepeatedView<'_, super::Permission> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Permission>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Permission> {
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
  pub fn set_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Permission>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `PermissionsResponseMut` does not perform any shared mutation.
unsafe impl Send for PermissionsResponseMut<'_> {}

// SAFETY:
// - `PermissionsResponseMut` does not perform any shared mutation.
unsafe impl Sync for PermissionsResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for PermissionsResponseMut<'msg> {
  type Proxied = PermissionsResponse;
  fn as_view(&self) -> ::protobuf::View<'_, PermissionsResponse> {
    PermissionsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PermissionsResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PermissionsResponse>
  where
      'msg: 'shorter {
    PermissionsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for PermissionsResponseMut<'msg> {
  type MutProxied = PermissionsResponse;
  fn as_mut(&mut self) -> PermissionsResponseMut<'msg> {
    PermissionsResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PermissionsResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> PermissionsResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PermissionsResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PermissionsResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PermissionsResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PermissionsResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // items: repeated message palm.rbac.v1.Permission
  pub fn items(&self) -> ::protobuf::RepeatedView<'_, super::Permission> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Permission>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Permission> {
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
  pub fn set_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Permission>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl PermissionsResponse

impl ::std::ops::Drop for PermissionsResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PermissionsResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PermissionsResponse {
  type Proxied = Self;
  fn as_view(&self) -> PermissionsResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PermissionsResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PermissionsResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PermissionsResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::palm__rbac__v1__PermissionsResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::palm__rbac__v1__PermissionsResponse_msg_init.0, &[<super::Permission as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::palm__rbac__v1__PermissionsResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PermissionsResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PermissionsResponse {
  type Msg = PermissionsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PermissionsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PermissionsResponse {
  type Msg = PermissionsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PermissionsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PermissionsResponseMut<'_> {
  type Msg = PermissionsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PermissionsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PermissionsResponseMut<'_> {
  type Msg = PermissionsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PermissionsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PermissionsResponseView<'_> {
  type Msg = PermissionsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PermissionsResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PermissionsResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



