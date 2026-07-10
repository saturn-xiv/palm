defmodule Rhododendron.Log do
  use Ecto.Schema
  import Ecto.Changeset

  schema "logs" do
    field :plugin, :string
    field :level, :string
    field :ip, :string
    field :resource, :string
    field :message, :string

    belongs_to :user, Rhododendron.User

    timestamps(updated_at: false, type: :utc_datetime_usec)
  end

  @doc false
  def changeset(log, attrs) do
    log
    |> cast(attrs, [:plugin, :level, :ip, :resource, :message])
    |> validate_required([:plugin, :level, :ip, :resource, :message])
  end
end
