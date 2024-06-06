defmodule Tuberose.Repo.Migrations.CreateAttachments do
  use Ecto.Migration

  def change do
    create table(:attachments) do
      add(:user_id, references(:users), null: false)
      add(:bucket, :string, size: 63, null: false)
      add(:object, :string, size: 63, null: false)
      add(:title, :string, size: 127, null: false)
      add(:size, :bigint, null: false)
      add(:content_type, :string, size: 63, null: false)
      add(:uploaded_at, :utc_datetime_usec)
      add(:deleted_at, :utc_datetime_usec)
      add(:version, :integer, null: false, default: 0)

      timestamps(type: :utc_datetime_usec)
    end

    create(unique_index(:attachments, [:bucket, :object]))
    create(index(:attachments, [:bucket]))
    create(index(:attachments, [:object]))
    create(index(:attachments, [:title]))
    create(index(:attachments, [:content_type]))
  end
end
