defmodule Bougainvillea.Repo.Migrations.CreateShortenLinks do
  use Ecto.Migration

  def change do
    create table(:shorten_links) do
      add :title, :string, size: 127, null: false
      add :url, :string, size: 127, null: false
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime_usec, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("now()")
    end

    create index(:shorten_links, [:title])
    create unique_index(:shorten_links, [:url])
  end
end
