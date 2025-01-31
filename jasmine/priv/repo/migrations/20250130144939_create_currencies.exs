defmodule Jasmine.Repo.Migrations.CreateCurrencies do
  use Ecto.Migration

  def up do
    create table(:currencies) do
      add :code, :string, size: 3, null: false
      add :number, :string, size: 3, null: false
      add :name, :string, size: 127, null: false
      add :country, :string, size: 127, null: false
      add :units, :integer, null: false, default: 0
      add :created_at, :utc_datetime_usec, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:currencies, [:code])
    create index(:currencies, [:number])
    create index(:currencies, [:name])
    create index(:currencies, [:country])
  end

  def down do
    drop table(:currencies)
  end
end
