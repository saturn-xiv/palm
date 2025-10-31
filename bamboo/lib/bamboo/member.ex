defmodule Bamboo.Member do
  use Ecto.Schema
  import Ecto.Changeset

  schema "members" do
    field :sn, :string
    field :name, :string
    field :memo, :string
    field :wifi_password, :string
    field :version, :integer
    field :deleted_at, :utc_datetime
    has_many :hosts, Bamboo.Host

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(member, attrs) do
    member
    |> cast(attrs, [:sn, :name, :memo, :wifi_password, :version, :deleted_at])
    |> validate_required([:sn, :name, :memo, :wifi_password, :version])
  end
end
