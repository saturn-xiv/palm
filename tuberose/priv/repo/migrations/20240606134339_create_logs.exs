defmodule Tuberose.Repo.Migrations.CreateLogs do
  use Ecto.Migration

  def change do
    create table(:logs) do
      add(:user_id, references(:users), null: false)
      add(:plugin, :string, size: 31, null: false)
      add(:ip, :string, size: 45, null: false)
      add(:level, :string, size: 8, null: false)
      add(:resource_type, :string, size: 127, null: false)
      add(:resource_id, :bigint)
      add(:message, :text, null: false)
      add(:created_at, :utc_datetime_usec, null: false, default: fragment("CURRENT_TIMESTAMP"))
    end

    create(index(:logs, [:plugin]))
    create(index(:logs, [:ip]))
    create(index(:logs, [:level]))
    create(index(:logs, [:resource_type]))
  end
end
