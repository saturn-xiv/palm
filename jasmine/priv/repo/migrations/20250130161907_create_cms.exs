defmodule Jasmine.Repo.Migrations.CreateCms do
  use Ecto.Migration

  def up do
    create table(:cms_pages) do
      add :author_id, :bigint, null: false
      add :lang, :string, size: 15, null: false, default: "en-US"
      add :slug, :string, size: 63, null: false
      add :title, :string, size: 127, null: false
      add :summary, :string, size: 511, null: false
      add :body, :text, null: false
      add :body_editor, :string, size: 15, null: false
      add :template, :string, size: 15, null: false
      add :priority, :integer, null: false, default: 0
      add :profile, :binary, null: false
      add :status, :string, size: 15, null: false
      add :deleted_at, :utc_datetime
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime, null: false
      add :created_at, :utc_datetime, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:cms_pages, [:lang, :slug], unique: true)
    create index(:cms_pages, [:lang])
    create index(:cms_pages, [:slug])
    create index(:cms_pages, [:title])
    create index(:cms_pages, [:summary])
    create index(:cms_pages, [:body_editor])
    create index(:cms_pages, [:template])
    create index(:cms_pages, [:status])
  end

  def down do
    drop table(:cms_pages)
  end
end
