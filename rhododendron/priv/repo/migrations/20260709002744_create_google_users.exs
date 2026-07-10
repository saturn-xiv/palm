defmodule Rhododendron.Repo.Migrations.CreateGoogleUsers do
  use Ecto.Migration

  def change do
    create table(:google_users) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :email, :string, size: 255
      add :email_verified, :boolean, null: false
      add :name, :string, size: 63
      add :picture, :string, size: 255
      add :sub, :string, size: 127, null: false
      add :code, :binary, null: false
      add :token, :string, null: false, size: 127
      add :locked_at, :utc_datetime_usec
      add :deleted_at, :utc_datetime_usec
      add :version, :integer, null: false, default: 0

      timestamps(type: :utc_datetime_usec)
    end

    create unique_index(:google_users, [:sub])
    create index(:google_users, [:email], where: "email IS NOT NULL")
    create index(:google_users, [:name], where: "name IS NOT NULL")
  end
end
