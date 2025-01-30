defmodule Jasmine.Repo.Migrations.CreateLocales do
  use Ecto.Migration

  def up do
    create table(:locales) do
      add :lang, :string, size: 15, null: false, default: "en-US"
      add :code, :string, size: 255, null: false
      add :message, :text, null: false
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime, null: false
      add :created_at, :utc_datetime, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:locales, [:lang, :code], unique: true)
    create index(:locales, [:lang])
    create index(:locales, [:code])
  end

  def down do
    drop table(:locales)
  end
end
