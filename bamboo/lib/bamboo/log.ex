defmodule Bamboo.Log do
  use Ecto.Schema
  import Ecto.Changeset

  schema "logs" do
    field :level, :string
    field :message, :string
    belongs_to :user, Bamboo.User

    timestamps(type: :utc_datetime, updated_at: false)
  end

  @doc false
  def changeset(log, attrs) do
    log
    |> cast(attrs, [:level, :message])
    |> validate_required([:level, :message])
  end
end
