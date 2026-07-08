defmodule Rhododendron.Repo.Migrations.CreateLocales do
  use Ecto.Migration

  def change do
    create table(:locales) do
      add :lang, :string, null: false, size: 15
      add :code, :string, null: false, size: 255
      add :message, :text, null: false
      add :version, :integer, null: false, default: 0

      timestamps(type: :utc_datetime_usec)
    end

    create index(:locales, [:lang])
    create index(:locales, [:code])
    create unique_index(:locales, [:code, :lang])
  end
end
