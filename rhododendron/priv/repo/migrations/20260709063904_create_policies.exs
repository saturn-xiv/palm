defmodule Rhododendron.Repo.Migrations.CreatePolicies do
  use Ecto.Migration

  def change do
    create table(:policies) do
      add :action, :string, null: false, size: 31
      add :resource_type, :string, null: false, size: 255
      add :resource_id, :integer

      add :role_id, references(:roles, on_delete: :delete_all)
      add :user_id, references(:users, on_delete: :delete_all)

      timestamps(updated_at: false, type: :utc_datetime_usec)
    end

    create unique_index(:policies, [:user_id, :resource_type, :action],
             where: "user_id IS NOT NULL AND resource_id IS NULL"
           )

    create unique_index(:policies, [:user_id, :resource_type, :resource_id, :action],
             where: "user_id IS NOT NULL AND resource_id IS NOT NULL"
           )

    create unique_index(:policies, [:role_id, :resource_type, :action],
             where: "role_id IS NOT NULL AND resource_id IS NULL"
           )

    create unique_index(:policies, [:role_id, :resource_type, :resource_id, :action],
             where: "role_id IS NOT NULL AND resource_id IS NOT NULL"
           )

    create index(:policies, [:action])
    create index(:policies, [:resource_type])
  end
end
