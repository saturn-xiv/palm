defmodule Rhododendron.Policy do
  use Ecto.Schema
  import Ecto.Changeset

  schema "policies" do
    field :action, :string
    field :resource_type, :string
    field :resource_id, :integer

    belongs_to :role, Rhododendron.Role
    belongs_to :user, Rhododendron.User

    timestamps(updated_at: false, type: :utc_datetime_usec)
  end

  @doc false
  def changeset(policy, attrs) do
    policy
    |> cast(attrs, [:action, :resource_type, :resource_id])
    |> validate_required([:action, :resource_type])
  end
end
