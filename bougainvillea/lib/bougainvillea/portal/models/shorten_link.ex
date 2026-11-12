defmodule Bougainvillea.Portal.Models.ShortenLink do
  use Ecto.Schema
  import Ecto.Changeset

  schema "shorten_links" do
    field :code, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(shorten_link, attrs) do
    shorten_link
    |> cast(attrs, [:code])
    |> validate_required([:code])
  end
end
