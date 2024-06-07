defmodule Tuberose.Repo.Migrations.CreateLeaveWords do
  use Ecto.Migration

  def change do
    create table(:leave_words) do
      add(:lang, :string, size: 15, null: false)
      add(:ip, :string, size: 45, null: false)
      add(:body, :text, null: false)
      add(:editor, :string, size: 15, null: false)
      add(:status, :string, size: 15, null: false)
      add(:published_at, :utc_datetime_usec)
      add(:deleted_at, :utc_datetime_usec)
      add(:version, :integer, null: false, default: 0)

      timestamps(type: :utc_datetime_usec)
    end

    create(index(:leave_words, [:lang]))
    create(index(:leave_words, [:ip]))
    create(index(:leave_words, [:editor]))
    create(index(:leave_words, [:status]))
  end
end
