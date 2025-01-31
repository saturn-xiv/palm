defmodule Jasmine.Repo.Migrations.CreateTags do
  use Ecto.Migration

  def up do
    create table(:tags) do
      add :name, :string, size: 63, null: false
      add :priority, :integer, null: false, default: 0
      add :deleted_at, :utc_datetime
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime_usec, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:tags, [:name], unique: true)

    create table(:tag_resources) do
      add :tag_id, :bigint, null: false
      add :resource_type, :string, size: 255, null: false
      add :resource_id, :bigint
      add :priority, :integer, null: false, default: 0
      add :created_at, :utc_datetime_usec, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:tag_resources, [:tag_id, :resource_type, :resource_id],
             unique: true,
             where: "resource_id IS NOT NULL"
           )

    create index(:tag_resources, [:tag_id, :resource_type],
             unique: true,
             where: "resource_id IS NULL"
           )

    create index(:tag_resources, [:resource_type])
  end

  def down do
    drop table(:tag_resources)
    drop table(:tags)
  end
end
