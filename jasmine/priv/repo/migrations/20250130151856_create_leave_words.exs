defmodule Jasmine.Repo.Migrations.CreateLeaveWords do
  use Ecto.Migration

  def up do
    create table(:leave_words) do
      add :lang, :string, size: 15, null: false, default: "en-US"
      add :ip, :string, size: 45, null: false
      add :body, :text, null: false
      add :body_editor, :string, size: 15, null: false
      add :status, :string, size: 15, null: false
      add :deleted_at, :utc_datetime
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime, null: false
      add :created_at, :utc_datetime, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:leave_words, [:lang])
    create index(:leave_words, [:ip])
    create index(:leave_words, [:body_editor])
    create index(:leave_words, [:status])
  end

  def down do
    drop table(:leave_words)
  end
end
