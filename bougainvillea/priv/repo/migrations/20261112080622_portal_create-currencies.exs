defmodule Bougainvillea.Repo.Migrations.Portal.CreateCurrencies do
  use Ecto.Migration

  def change do
    create table(:currencies) do
      add :name, :string, size: 127, null: false
      add :code, :string, size: 3, null: false
      add :country, :string, size: 127, null: false
      add :number, :string, size: 3, null: false
      add :units, :integer
      add :fund, :boolean

      timestamps(type: :utc_datetime_usec, updated_at: false)
    end

    create index(:currencies, [:name])
    create index(:currencies, [:code])
    create index(:currencies, [:number])
    create index(:currencies, [:country])
  end
end
