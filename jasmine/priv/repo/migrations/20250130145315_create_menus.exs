defmodule Jasmine.Repo.Migrations.CreateMenus do
  use Ecto.Migration

  def up do
    create table(:menus) do
      add :parent_id, :bigint
      add :lang, :string, size: 15, null: false, default: "en-US"
      add :location, :string, size: 31, null: false
      add :href, :string, size: 255
      add :label, :string, size: 63, null: false
      add :priority, :integer, null: false, default: 0
      add :is_extra, :boolean, null: false, default: false
      add :deleted_at, :utc_datetime
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime_usec, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:menus, [:href], where: "href IS NOT NULL")
    create index(:menus, [:label])
    create index(:menus, [:location])
    create index(:menus, [:lang])
  end

  def down do
    drop table(:menus)
  end
end
