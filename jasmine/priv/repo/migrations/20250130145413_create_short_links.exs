defmodule Jasmine.Repo.Migrations.CreateShortLinks do
  use Ecto.Migration

  def up do
    create table(:short_links) do
      add :url, :string, size: 255, null: false
      add :title, :string, size: 127, null: false
      add :memo, :string, size: 511, null: false, default: ""
      add :deleted_at, :utc_datetime
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime, null: false
      add :created_at, :utc_datetime, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:short_links, [:url], unique: true)
    create index(:short_links, [:title])
    create index(:short_links, [:memo])
  end

  def down do
    drop table(:short_links)
  end
end
