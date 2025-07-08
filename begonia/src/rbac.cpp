#include "palm/rbac.hpp"

std::string palm::casbin::user::to_subject(uint32_t id) {
  google::protobuf::Arena arena;
  palm::casbin::v1::User* user =
      google::protobuf::Arena::Create<palm::casbin::v1::User>(&arena);
  user->set_id(id);
  return user->SerializeAsString();
}
std::string palm::casbin::user::to_subject(const std::string& code) {
  google::protobuf::Arena arena;
  palm::casbin::v1::User* user =
      google::protobuf::Arena::Create<palm::casbin::v1::User>(&arena);
  user->set_code(code);
  return user->SerializeAsString();
}

std::string palm::casbin::role::root() {
  google::protobuf::Arena arena;
  palm::casbin::v1::Role* role =
      google::protobuf::Arena::Create<palm::casbin::v1::Role>(&arena);
  palm::casbin::v1::Role_Root* root =
      google::protobuf::Arena::Create<palm::casbin::v1::Role_Root>(&arena);
  role->set_allocated_root(root);
  return role->SerializeAsString();
}
std::string palm::casbin::role::administrator() {
  google::protobuf::Arena arena;
  palm::casbin::v1::Role* role =
      google::protobuf::Arena::Create<palm::casbin::v1::Role>(&arena);
  palm::casbin::v1::Role_Administrator* administrator =
      google::protobuf::Arena::Create<palm::casbin::v1::Role_Administrator>(
          &arena);
  role->set_allocated_administrator(administrator);
  return role->SerializeAsString();
}
std::string palm::casbin::role::other(const std::string& code) {
  google::protobuf::Arena arena;
  palm::casbin::v1::Role* role =
      google::protobuf::Arena::Create<palm::casbin::v1::Role>(&arena);
  palm::casbin::v1::Role_Other* other =
      google::protobuf::Arena::Create<palm::casbin::v1::Role_Other>(&arena);
  other->set_code(code);
  role->set_allocated_other(other);
  return role->SerializeAsString();
}

std::string palm::casbin::permission::read() {
  google::protobuf::Arena arena;
  palm::casbin::v1::Permission* permission =
      google::protobuf::Arena::Create<palm::casbin::v1::Permission>(&arena);
  palm::casbin::v1::Permission_Read* read =
      google::protobuf::Arena::Create<palm::casbin::v1::Permission_Read>(
          &arena);
  permission->set_allocated_read(read);
  return permission->SerializeAsString();
}
std::string palm::casbin::permission::write() {
  google::protobuf::Arena arena;
  palm::casbin::v1::Permission* permission =
      google::protobuf::Arena::Create<palm::casbin::v1::Permission>(&arena);
  palm::casbin::v1::Permission_Write* write =
      google::protobuf::Arena::Create<palm::casbin::v1::Permission_Write>(
          &arena);
  permission->set_allocated_write(write);
  return permission->SerializeAsString();
}
std::string palm::casbin::permission::append() {
  google::protobuf::Arena arena;
  palm::casbin::v1::Permission* permission =
      google::protobuf::Arena::Create<palm::casbin::v1::Permission>(&arena);
  palm::casbin::v1::Permission_Append* append =
      google::protobuf::Arena::Create<palm::casbin::v1::Permission_Append>(
          &arena);
  permission->set_allocated_append(append);
  return permission->SerializeAsString();
}
std::string palm::casbin::permission::execute() {
  google::protobuf::Arena arena;
  palm::casbin::v1::Permission* permission =
      google::protobuf::Arena::Create<palm::casbin::v1::Permission>(&arena);
  palm::casbin::v1::Permission_Execute* execute =
      google::protobuf::Arena::Create<palm::casbin::v1::Permission_Execute>(
          &arena);
  permission->set_allocated_execute(execute);
  return permission->SerializeAsString();
}
std::string palm::casbin::permission::credit() {
  google::protobuf::Arena arena;
  palm::casbin::v1::Permission* permission =
      google::protobuf::Arena::Create<palm::casbin::v1::Permission>(&arena);
  palm::casbin::v1::Permission_Credit* credit =
      google::protobuf::Arena::Create<palm::casbin::v1::Permission_Credit>(
          &arena);
  permission->set_allocated_credit(credit);
  return permission->SerializeAsString();
}
std::string palm::casbin::permission::debit() {
  google::protobuf::Arena arena;
  palm::casbin::v1::Permission* permission =
      google::protobuf::Arena::Create<palm::casbin::v1::Permission>(&arena);
  palm::casbin::v1::Permission_Debit* debit =
      google::protobuf::Arena::Create<palm::casbin::v1::Permission_Debit>(
          &arena);
  permission->set_allocated_debit(debit);
  return permission->SerializeAsString();
}
std::string palm::casbin::permission::inquiry() {
  google::protobuf::Arena arena;
  palm::casbin::v1::Permission* permission =
      google::protobuf::Arena::Create<palm::casbin::v1::Permission>(&arena);
  palm::casbin::v1::Permission_Inquiry* inquiry =
      google::protobuf::Arena::Create<palm::casbin::v1::Permission_Inquiry>(
          &arena);
  permission->set_allocated_inquiry(inquiry);
  return permission->SerializeAsString();
}
std::string palm::casbin::permission::other(const std::string& code) {
  google::protobuf::Arena arena;
  palm::casbin::v1::Permission* permission =
      google::protobuf::Arena::Create<palm::casbin::v1::Permission>(&arena);
  palm::casbin::v1::Permission_Other* other =
      google::protobuf::Arena::Create<palm::casbin::v1::Permission_Other>(
          &arena);
  other->set_code(code);
  permission->set_allocated_other(other);
  return permission->SerializeAsString();
}

std::string palm::casbin::resource::to_object(const std::string& type,
                                              uint32_t id) {
  google::protobuf::Arena arena;
  palm::casbin::v1::Resource* resource =
      google::protobuf::Arena::Create<palm::casbin::v1::Resource>(&arena);
  resource->set_type(type);
  resource->set_id(id);
  return resource->SerializeAsString();
}
std::string palm::casbin::resource::to_object(const std::string& type,
                                              const std::string& code) {
  google::protobuf::Arena arena;
  palm::casbin::v1::Resource* resource =
      google::protobuf::Arena::Create<palm::casbin::v1::Resource>(&arena);
  resource->set_type(type);
  resource->set_code(code);
  return resource->SerializeAsString();
}
std::string palm::casbin::resource::to_object(const std::string& type) {
  google::protobuf::Arena arena;
  palm::casbin::v1::Resource* resource =
      google::protobuf::Arena::Create<palm::casbin::v1::Resource>(&arena);
  resource->set_type(type);
  return resource->SerializeAsString();
}
