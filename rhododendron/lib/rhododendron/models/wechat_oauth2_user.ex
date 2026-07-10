defmodule Rhododendron.WechatOauth2User do
  use Ecto.Schema
  import Ecto.Changeset

  schema "wechat_oauth2_users" do
    field :name, :string

    timestamps(type: :utc_datetime_usec)
  end

  @doc false
  def changeset(wechat_oauth2_user, attrs) do
    wechat_oauth2_user
    |> cast(attrs, [:name])
    |> validate_required([:name])
  end
end
