defmodule Jasmine.Repo.Migrations.CreateAttachments do
  use Ecto.Migration

  def up do
    create table(:attachments) do
      add :user_id, :bigint, null: false
      add :bucket, :string, size: 63, null: false
      add :object, :string, size: 63, null: false
      add :title, :string, size: 127, null: false
      add :size, :integer, null: false
      add :content_type, :string, size: 63, null: false
      add :uploaded_at, :utc_datetime
      add :deleted_at, :utc_datetime
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime_usec, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:attachments, [:bucket, :object], unique: true)
    create index(:attachments, [:title])
    create index(:attachments, [:content_type])

    create table(:attachment_resources) do
      add :attachment_id, :bigint, null: false
      add :resource_type, :string, size: 255, null: false
      add :resource_id, :bigint
      add :priority, :integer, null: false, default: 0
      add :created_at, :utc_datetime_usec, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:attachment_resources, [:attachment_id, :resource_type, :resource_id],
             unique: true,
             where: "resource_id IS NOT NULL"
           )

    create index(:attachment_resources, [:attachment_id, :resource_type],
             unique: true,
             where: "resource_id IS NULL"
           )

    create index(:attachment_resources, [:resource_type])
  end

  def down do
    drop table(:attachment_resources)
    drop table(:attachments)
  end
end
