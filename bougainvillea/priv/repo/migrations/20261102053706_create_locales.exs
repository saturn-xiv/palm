defmodule Bougainvillea.Repo.Migrations.CreateLocales do
  use Ecto.Migration

  def change do

    create table(:locales) do
      add :lang, :string, size: 7, null: false
      add :code, :string, size: 255, null: false
      add :message, :text, null: false
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime_usec, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("now()")
    end

    create index(:locales, [:lang])
    create index(:locales, [:code])
    create unique_index(:locales, [:lang, :code])

  end
end
