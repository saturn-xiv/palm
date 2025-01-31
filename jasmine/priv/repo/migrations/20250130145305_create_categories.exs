defmodule Jasmine.Repo.Migrations.CreateCategories do
  use Ecto.Migration

  def up do
    create table(:categories) do
      add :tree, :string, size: 31, null: false
      add :name, :string, size: 63, null: false
      add :left, :integer, null: false
      add :right, :integer, null: false
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime_usec, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:categories, [:tree])
    create index(:categories, [:name])
    create index(:categories, [:tree, :name], unique: true)

    create table(:category_resources) do
      add :category_id, :bigint, null: false
      add :resource_type, :string, size: 255, null: false
      add :resource_id, :bigint
      add :priority, :integer, null: false, default: 0
      add :created_at, :utc_datetime_usec, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:category_resources, [:category_id, :resource_type, :resource_id],
             unique: true,
             where: "resource_id IS NOT NULL"
           )

    create index(:category_resources, [:category_id, :resource_type],
             unique: true,
             where: "resource_id IS NULL"
           )

    create index(:category_resources, [:resource_type])
  end

  def down do
    drop table(:category_resources)
    drop table(:categories)
  end
end
