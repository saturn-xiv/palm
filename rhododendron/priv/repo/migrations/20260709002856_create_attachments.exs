defmodule Rhododendron.Repo.Migrations.CreateAttachments do
  use Ecto.Migration

  def change do
    create table(:attachments) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :bucket, :string, null: false, size: 63
      add :object, :string, null: false, size: 63
      add :title, :string, null: false, size: 127
      add :size, :integer, null: false
      add :content_type, :string, null: false, size: 63
      add :public, :boolean, null: false, default: false
      add :uploaded_at, :utc_datetime_usec
      add :deleted_at, :utc_datetime_usec
      add :version, :integer, null: false, default: 0

      timestamps(type: :utc_datetime_usec)
    end

    create unique_index(:attachments, [:bucket, :object])
    create index(:attachments, [:bucket])
    create index(:attachments, [:object])
    create index(:attachments, [:title])
    create index(:attachments, [:content_type])
  end
end
