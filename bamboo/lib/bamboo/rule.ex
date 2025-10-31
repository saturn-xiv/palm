defmodule Bamboo.Rule do
  use Ecto.Schema
  import Ecto.Changeset

  schema "rules" do
    field :subject, :string
    field :body, :string
    field :version, :integer
    many_to_many :hosts, Bamboo.Host, join_through: "hosts_rules"

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(rule, attrs) do
    rule
    |> cast(attrs, [:subject, :body])
    |> validate_required([:subject, :body])
  end
end
