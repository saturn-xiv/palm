defmodule Bamboo.Repo.Migrations.CreateRules do
  use Ecto.Migration

  def change do
    create table(:rules) do
      add :subject, :string, null: false, size: 63
      add :body, :text, null: false
      add :version, :integer, null: false, default: 0

      timestamps(type: :utc_datetime)
    end

    create index(:rules, [:subject], unique: true)

    create table(:hosts_rules, primary_key: false) do
      add :host_id, references(:hosts), null: false
      add :rule_id, references(:rules), null: false
      timestamps(type: :utc_datetime, updated_at: false)
    end

    create unique_index(:hosts_rules, [:host_id, :rule_id])
  end
end
