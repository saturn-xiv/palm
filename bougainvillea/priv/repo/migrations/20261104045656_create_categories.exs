defmodule Bougainvillea.Repo.Migrations.CreateCategories do
  use Ecto.Migration

  def change do
    create table(:categories) do
      add :code, :string, size: 31, null: false
      add :left, :integer, null: false
      add :right, :integer, null: false
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime_usec, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("now()")
    end

    create index(:categories, [:code])
  end
end
