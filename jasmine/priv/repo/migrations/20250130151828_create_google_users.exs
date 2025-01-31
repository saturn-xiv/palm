defmodule Jasmine.Repo.Migrations.CreateGoogleUsers do
  use Ecto.Migration

  def up do
    create table(:google_oauth2_users) do
      add :user_id, :bigint, null: false
      add :subject, :string, size: 127, null: false
      add :email, :string, size: 127
      add :email_verified, :boolean, null: false, default: false
      add :name, :string, size: 63
      add :picture, :string, size: 255
      add :locale, :string, size: 15
      add :deleted_at, :utc_datetime
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime, null: false
      add :created_at, :utc_datetime, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:google_oauth2_users, [:subject], unique: true)
    create index(:google_oauth2_users, [:email], where: "email IS NOT NULL")
    create index(:google_oauth2_users, [:name], where: "name IS NOT NULL")
    create index(:google_oauth2_users, [:locale], where: "locale IS NOT NULL")
  end

  def down do
    drop table(:google_oauth2_users)
  end
end
