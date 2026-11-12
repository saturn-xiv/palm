defmodule Bougainvillea.Repo.Migrations.Portal.CreateCategories do
  use Ecto.Migration

  def change do
    create table(:categories) do
      add :code, :string, size: 31, null: false
      add :left, :integer, null: false
      add :right, :integer, null: false
      add :version, :integer, null: false, default: 0

      timestamps(type: :utc_datetime_usec)
    end

    create index(:categories, [:code])
  end
end
