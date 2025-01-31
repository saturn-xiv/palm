defmodule Jasmine.Locale do
  use Ecto.Schema
  import Ecto.Changeset

  schema "locales" do
    field :lang, :string
    field :code, :string
    field :message, :string
    field :version, :integer
    field :updated_at, :utc_datetime_usec
    field :created_at, :utc_datetime_usec
  end

  @doc false
  def changeset(locale, attrs) do
    locale
    |> cast(attrs, [:lang, :code, :message, :version, :updated_at, :created_at])
    |> validate_required([:lang, :code, :message])
  end
end
