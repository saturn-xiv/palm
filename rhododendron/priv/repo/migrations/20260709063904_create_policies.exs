defmodule Rhododendron.Repo.Migrations.CreatePolicies do
  use Ecto.Migration

  def change do
    create table(:policies) do
      add :subject, :string, null: false, size: 127
      add :object, :string, null: false, size: 255
      add :action, :string, null: false, size: 31

      timestamps(updated_at: false, type: :utc_datetime_usec)
    end

    create unique_index(:policies, [:subject, :object, :action])
    create index(:policies, [:subject])
    create index(:policies, [:object])
    create index(:policies, [:action])
  end
end
