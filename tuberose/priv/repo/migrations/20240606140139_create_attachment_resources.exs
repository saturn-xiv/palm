defmodule Tuberose.Repo.Migrations.CreateAttachmentResources do
  use Ecto.Migration

  def change do
    create table(:attachment_resources) do
      add(:attachment_id, :bigint, null: false)
      add(:resource_type, :string, size: 127, null: false)
      add(:resource_id, :bigint, null: false)
      add(:created_at, :utc_datetime, null: false, default: fragment("CURRENT_TIMESTAMP"))
    end

    create(unique_index(:attachment_resources, [:attachment_id, :resource_type, :resource_id]))
    create(index(:attachment_resources, [:resource_type]))
  end
end
