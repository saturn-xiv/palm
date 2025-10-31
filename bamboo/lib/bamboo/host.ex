defmodule Bamboo.Host do
  use Ecto.Schema
  import Ecto.Changeset

  schema "hosts" do
    field :name, :string
    field :mac, :string
    field :ip, :string
    field :version, :integer
    field :deleted_at, :utc_datetime
    belongs_to :member, Bamboo.Member
    many_to_many :rules, Bamboo.Rule, join_through: "hosts_rules"

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(host, attrs) do
    host
    |> cast(attrs, [:name, :mac, :ip, :version, :deleted_at])
    |> validate_required([:name, :mac, :ip, :version])
  end
end
