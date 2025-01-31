defmodule Jasmine.Repo.Migrations.CreateAccounting do
  use Ecto.Migration

  def up do
    create table(:accounting_books) do
      add :user_id, :bigint, null: false
      add :uid, :string, size: 36, null: false
      add :title, :string, size: 127, null: false
      add :memo, :string, size: 511, null: false
      add :status, :string, size: 15, null: false
      add :deleted_at, :utc_datetime
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime_usec, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:accounting_books, [:uid], unique: true)
    create index(:accounting_books, [:title])
    create index(:accounting_books, [:memo])
    create index(:accounting_books, [:status])
  end

  def down do
    drop table(:accounting_books)
  end
end
