defmodule Rhododendron.Repo.Migrations.CreatePolicies do
  use Ecto.Migration

  def change do
    create table(:policies) do
      add :action, :string, null: false, size: 31
      add :object, :string, null: false, size: 255

      add :role_id, references(:roles, on_delete: :delete_all)
      add :user_id, references(:users, on_delete: :delete_all)

      timestamps(updated_at: false, type: :utc_datetime_usec)
    end

    create unique_index(:policies, [:user_id, :object, :action], where: "user_id IS NOT NULL")
    create unique_index(:policies, [:role_id, :object, :action], where: "role_id IS NOT NULL")
    create index(:policies, [:action])
    create index(:policies, [:object])
  end
end
