defmodule Jasmine.Log do
  use Ecto.Schema
  import Ecto.Changeset

  schema "logs" do
    field :user_id, :integer
    field :plugin, :string
    field :ip, :string
    field :level, Ecto.Enum, values: [:debug, :info, :warning, :error]
    field :resource_type, :string
    field :resource_id, :integer
    field :message, :string
    field :created_at, :utc_datetime_usec
  end

  @doc false
  def changeset(log, attrs) do
    log
    |> cast(attrs, [
      :user_id,
      :plugin,
      :ip,
      :level,
      :resource_type,
      :resource_id,
      :message,
      :created_at
    ])
    |> validate_required([:user_id, :plugin, :ip, :level, :resource_type, :message])
  end
end
