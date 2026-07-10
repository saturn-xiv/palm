defmodule Rhododendron.Attachment do
  use Ecto.Schema
  import Ecto.Changeset

  schema "attachments" do
    field :name, :string

    timestamps(type: :utc_datetime_usec)
  end

  @doc false
  def changeset(attachment, attrs) do
    attachment
    |> cast(attrs, [:name])
    |> validate_required([:name])
  end
end
