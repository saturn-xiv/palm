defmodule Bamboo.User do
  use Ecto.Schema
  import Ecto.Changeset

  schema "users" do
    field :name, :string
    field :password, :string
    field :version, :integer
    has_many :logs, Bamboo.Log

    timestamps(type: :utc_datetime)
  end

  def generate_password(secret_key, plain) do
    # TODO salt: # :crypto.strong_rand_bytes(8) |> Base.encode64()
    :crypto.mac(:hmac, :sha512, secret_key, plain) |> Base.encode64()
  end

  def rules() do
    %{
      name: [required: true, type: :string, min: 2, max: 15],
      password: [required: true, type: :string, min: 6, max: 31]
    }
  end

  @doc false
  def changeset(user, attrs) do
    user
    |> cast(attrs, [:name, :password])
    |> validate_required([:name, :password])
  end
end
