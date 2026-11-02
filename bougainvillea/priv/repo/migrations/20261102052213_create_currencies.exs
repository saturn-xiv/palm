defmodule Bougainvillea.Repo.Migrations.CreateCurrencies do
  use Ecto.Migration

  def change do

    create table(:currencies) do
      add :name, :string, size: 127, null: false
      add :code, :string, size: 3, null: false
      add :country, :string, size: 127, null: false
      add :number, :string, size: 3, null: false
      add :units, :integer
      add :fund, :boolean
      add :created_at, :utc_datetime_usec, null: false, default: fragment("now()")
    end

    create index(:currencies, [:name])
    create index(:currencies, [:code])
    create index(:currencies, [:number])
    create index(:currencies, [:country])

  end
end
