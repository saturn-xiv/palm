defmodule Tuberose.Locale do
  use Ecto.Schema
  import Ecto.Changeset

  schema "locales" do
    field(:lang, :string)
    field(:code, :string)
    field(:message, :string)
    field(:version, :integer)

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(locale, attrs) do
    locale
    |> cast(attrs, [:lang, :code, :message, :version])
    |> validate_required([:lang, :code, :message])
  end
end
