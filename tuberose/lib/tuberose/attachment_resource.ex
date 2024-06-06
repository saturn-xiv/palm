defmodule Tuberose.AttachmentResource do
  use Ecto.Schema
  import Ecto.Changeset

  schema "attachment_resources" do
    field(:attachment_id, :integer)
    field(:resource_type, :string)
    field(:resource_id, :integer)
    field(:created_at, :utc_datetime)
  end

  @doc false
  def changeset(attachment_resource, attrs) do
    attachment_resource
    |> cast(attrs, [:attachment_id, :resource_type, :resource_id, :created_at])
    |> validate_required([:attachment_id, :resource_type, :resource_id])
  end
end
