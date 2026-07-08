defmodule Rhododendron.Repo.Migrations.CreateCurrencies do
  use Ecto.Migration

  def change do
    create table(:currencies) do
      add :name, :string, null: false, size: 127
      add :code, :string, null: false, size: 3
      add :country, :string, null: false, size: 127
      add :number, :integer, null: false
      add :units, :integer
      add :is_fund, :boolean

      timestamps(type: :utc_datetime_usec)
    end

    create index(:currencies, [:name])
    create index(:currencies, [:code])
    create index(:currencies, [:country])
  end
end
