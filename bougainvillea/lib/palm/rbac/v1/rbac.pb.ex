defmodule Palm.Rbac.V1.Object do
  @moduledoc false

  use Protobuf,
    full_name: "palm.rbac.v1.Object",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  oneof(:by, 0)

  field :type, 1, type: :string
  field :id, 11, type: :int64, oneof: 0
  field :code, 12, type: :string, oneof: 0
  field :all, 19, type: Google.Protobuf.Empty, oneof: 0
end

defmodule Palm.Rbac.V1.Subject.Role.Root do
  @moduledoc false

  use Protobuf,
    full_name: "palm.rbac.v1.Subject.Role.Root",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3
end

defmodule Palm.Rbac.V1.Subject.Role.Administrator do
  @moduledoc false

  use Protobuf,
    full_name: "palm.rbac.v1.Subject.Role.Administrator",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3
end

defmodule Palm.Rbac.V1.Subject.Role do
  @moduledoc false

  use Protobuf,
    full_name: "palm.rbac.v1.Subject.Role",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  oneof(:by, 0)

  field :root, 1, type: Palm.Rbac.V1.Subject.Role.Root, oneof: 0
  field :administrator, 2, type: Palm.Rbac.V1.Subject.Role.Administrator, oneof: 0
  field :id, 8, type: :int64, oneof: 0
  field :code, 9, type: :string, oneof: 0
end

defmodule Palm.Rbac.V1.Subject.User do
  @moduledoc false

  use Protobuf,
    full_name: "palm.rbac.v1.Subject.User",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  oneof(:by, 0)

  field :id, 1, type: :int64, oneof: 0
  field :code, 2, type: :string, oneof: 0
end

defmodule Palm.Rbac.V1.Subject do
  @moduledoc false

  use Protobuf,
    full_name: "palm.rbac.v1.Subject",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  oneof(:by, 0)

  field :user, 1, type: Palm.Rbac.V1.Subject.User, oneof: 0
  field :role, 2, type: Palm.Rbac.V1.Subject.Role, oneof: 0
end

defmodule Palm.Rbac.V1.Action.Read do
  @moduledoc false

  use Protobuf,
    full_name: "palm.rbac.v1.Action.Read",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3
end

defmodule Palm.Rbac.V1.Action.Write do
  @moduledoc false

  use Protobuf,
    full_name: "palm.rbac.v1.Action.Write",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3
end

defmodule Palm.Rbac.V1.Action.Append do
  @moduledoc false

  use Protobuf,
    full_name: "palm.rbac.v1.Action.Append",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3
end

defmodule Palm.Rbac.V1.Action.Execute do
  @moduledoc false

  use Protobuf,
    full_name: "palm.rbac.v1.Action.Execute",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3
end

defmodule Palm.Rbac.V1.Action.Credit do
  @moduledoc false

  use Protobuf,
    full_name: "palm.rbac.v1.Action.Credit",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3
end

defmodule Palm.Rbac.V1.Action.Debit do
  @moduledoc false

  use Protobuf,
    full_name: "palm.rbac.v1.Action.Debit",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3
end

defmodule Palm.Rbac.V1.Action.Inquiry do
  @moduledoc false

  use Protobuf,
    full_name: "palm.rbac.v1.Action.Inquiry",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3
end

defmodule Palm.Rbac.V1.Action do
  @moduledoc false

  use Protobuf,
    full_name: "palm.rbac.v1.Action",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  oneof(:by, 0)

  field :read, 1, type: Palm.Rbac.V1.Action.Read, oneof: 0
  field :write, 2, type: Palm.Rbac.V1.Action.Write, oneof: 0
  field :append, 3, type: Palm.Rbac.V1.Action.Append, oneof: 0
  field :execute, 4, type: Palm.Rbac.V1.Action.Execute, oneof: 0
  field :credit, 5, type: Palm.Rbac.V1.Action.Credit, oneof: 0
  field :debit, 6, type: Palm.Rbac.V1.Action.Debit, oneof: 0
  field :inquiry, 7, type: Palm.Rbac.V1.Action.Inquiry, oneof: 0
  field :code, 9, type: :string, oneof: 0
end

defmodule Palm.Rbac.V1.Permission do
  @moduledoc false

  use Protobuf,
    full_name: "palm.rbac.v1.Permission",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :subject, 1, type: Palm.Rbac.V1.Subject
  field :object, 2, type: Palm.Rbac.V1.Object
  field :action, 3, type: Palm.Rbac.V1.Action
end

defmodule Palm.Rbac.V1.UserRoleRequest do
  @moduledoc false

  use Protobuf,
    full_name: "palm.rbac.v1.UserRoleRequest",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :user, 1, type: Palm.Rbac.V1.Subject.User
  field :role, 2, type: Palm.Rbac.V1.Subject.Role
end

defmodule Palm.Rbac.V1.RolesResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.rbac.v1.RolesResponse",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :items, 1, repeated: true, type: Palm.Rbac.V1.Subject.Role
end

defmodule Palm.Rbac.V1.UsersResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.rbac.v1.UsersResponse",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :items, 1, repeated: true, type: Palm.Rbac.V1.Subject.User
end

defmodule Palm.Rbac.V1.SubjectsResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.rbac.v1.SubjectsResponse",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :items, 1, repeated: true, type: Palm.Rbac.V1.Subject
end

defmodule Palm.Rbac.V1.ObjectsResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.rbac.v1.ObjectsResponse",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :items, 1, repeated: true, type: Palm.Rbac.V1.Object
end

defmodule Palm.Rbac.V1.ActionsResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.rbac.v1.ActionsResponse",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :items, 1, repeated: true, type: Palm.Rbac.V1.Action
end

defmodule Palm.Rbac.V1.PermissionsResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.rbac.v1.PermissionsResponse",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :items, 1, repeated: true, type: Palm.Rbac.V1.Permission
end

defmodule Palm.Rbac.V1.Enforcer.Service do
  @moduledoc false

  use GRPC.Service, name: "palm.rbac.v1.Enforcer", protoc_gen_elixir_version: "0.17.1"

  rpc(:GetAllSubjects, Google.Protobuf.Empty, Palm.Rbac.V1.SubjectsResponse)

  rpc(:GetAllObjects, Google.Protobuf.Empty, Palm.Rbac.V1.ObjectsResponse)

  rpc(:GetAllActions, Google.Protobuf.Empty, Palm.Rbac.V1.ActionsResponse)

  rpc(:GetAllRoles, Google.Protobuf.Empty, Palm.Rbac.V1.RolesResponse)

  rpc(:GetRolesForUser, Palm.Rbac.V1.Subject.User, Palm.Rbac.V1.RolesResponse)

  rpc(:GetImplicitRolesForUser, Palm.Rbac.V1.Subject.User, Palm.Rbac.V1.RolesResponse)

  rpc(:GetUsersForRole, Palm.Rbac.V1.Subject.Role, Palm.Rbac.V1.UsersResponse)

  rpc(:GetImplicitUsersForRole, Palm.Rbac.V1.Subject.Role, Palm.Rbac.V1.UsersResponse)

  rpc(:HasRoleForUser, Palm.Rbac.V1.UserRoleRequest, Google.Protobuf.Empty)

  rpc(:AddRoleForUser, Palm.Rbac.V1.UserRoleRequest, Google.Protobuf.Empty)

  rpc(:DeleteRoleForUser, Palm.Rbac.V1.UserRoleRequest, Google.Protobuf.Empty)

  rpc(:DeleteUser, Palm.Rbac.V1.Subject.User, Google.Protobuf.Empty)

  rpc(:DeleteRole, Palm.Rbac.V1.Subject.Role, Google.Protobuf.Empty)

  rpc(:GetPermissions, Palm.Rbac.V1.Subject, Palm.Rbac.V1.PermissionsResponse)

  rpc(:GetImplicitPermissions, Palm.Rbac.V1.Subject, Palm.Rbac.V1.PermissionsResponse)

  rpc(:DeletePermission, Palm.Rbac.V1.Permission, Google.Protobuf.Empty)

  rpc(:AddPermission, Palm.Rbac.V1.Permission, Google.Protobuf.Empty)

  rpc(:HasPermission, Palm.Rbac.V1.Permission, Google.Protobuf.Empty)
end

defmodule Palm.Rbac.V1.Enforcer.Stub do
  @moduledoc false

  use GRPC.Stub, service: Palm.Rbac.V1.Enforcer.Service
end
