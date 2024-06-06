defmodule Tuberose.UserSession do
  use Ecto.Schema
  import Ecto.Changeset

  schema "user_sessions" do
    field :ip, :string
    field :user_id, :integer
    field :uid, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(user_session, attrs) do
    user_session
    |> cast(attrs, [:user_id, :uid, :ip])
    |> validate_required([:user_id, :uid, :ip])
  end
end
