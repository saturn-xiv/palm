defmodule Bougainvillea.Portal.Models.Attachment do
  use Ecto.Schema
  import Ecto.Changeset

  schema "attachments" do
    field :code, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(attachment, attrs) do
    attachment
    |> cast(attrs, [:code])
    |> validate_required([:code])
  end
end
