defmodule Rhododendron.Role do
  use Ecto.Schema
  import Ecto.Changeset

  schema "roles" do
    field :code, :string
    field :left, :integer
    field :right, :integer
    field :version, :integer

    many_to_many :users, Rhododendron.User, join_through: "roles_users"

    timestamps(type: :utc_datetime_usec)
  end

  @doc false
  def changeset(role, attrs) do
    role
    |> cast(attrs, [:code, :left, :right, :version])
    |> validate_required([:code, :left, :right])
  end
end
