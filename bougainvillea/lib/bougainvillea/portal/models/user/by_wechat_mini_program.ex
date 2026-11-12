defmodule Bougainvillea.Portal.Models.User.ByWechatMiniProgram do
  use Ecto.Schema
  import Ecto.Changeset

  schema "wechat_mini_program_users" do
    field :code, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(wechat_mini_program_user, attrs) do
    wechat_mini_program_user
    |> cast(attrs, [:code])
    |> validate_required([:code])
  end
end
