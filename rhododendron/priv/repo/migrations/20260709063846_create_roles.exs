defmodule Rhododendron.Repo.Migrations.CreateRoles do
  use Ecto.Migration

  def change do
    create table(:roles) do
      add :code, :string, null: false, size: 31
      add :left, :integer, null: false
      add :right, :integer, null: false
      add :version, :integer, null: false, default: 0

      timestamps(type: :utc_datetime_usec)
    end

    create unique_index(:roles, [:code])

    create table(:roles_users, primary_key: false) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :role_id, references(:roles, on_delete: :delete_all), null: false
    end

    create unique_index(:roles_users, [:role_id, :user_id])
  end
end
