defmodule Jasmine.Repo.Migrations.CreateEmailUsers do
  use Ecto.Migration

  def up do
    create table(:email_users) do
      add :user_id, :bigint, null: false
      add :name, :string, size: 63, null: false
      add :email, :string, size: 127, null: false
      add :password, :binary, null: false
      add :avatar, :string, size: 255, null: false
      add :confirmed_at, :utc_datetime
      add :locked_at, :utc_datetime
      add :deleted_at, :utc_datetime
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime_usec, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:email_users, [:email], unique: true)
    create index(:email_users, [:name])
  end

  def down do
    drop table(:email_users)
  end
end
