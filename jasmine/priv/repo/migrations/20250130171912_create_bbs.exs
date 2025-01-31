defmodule Jasmine.Repo.Migrations.CreateBbs do
  use Ecto.Migration

  def up do
    create table(:bbs_forums) do
      add :lang, :string, size: 15
      add :slug, :string, size: 63, null: false
      add :title, :string, size: 127, null: false
      add :memo, :string, size: 511, null: false
      add :priority, :integer, null: false, default: 0
      add :status, :string, size: 15, null: false
      add :profile, :binary, null: false
      add :deleted_at, :utc_datetime
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime_usec, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:bbs_forums, [:slug], unique: true)
    create index(:bbs_forums, [:lang], where: "lang IS NOT NULL")
    create index(:bbs_forums, [:title])
    create index(:bbs_forums, [:memo])
    create index(:bbs_forums, [:status])

    create table(:bbs_topics) do
      add :user_id, :bigint, null: false
      add :forum_id, :bigint, null: false
      add :slug, :string, size: 63, null: false
      add :title, :string, size: 127, null: false
      add :body, :text, null: false
      add :body_editor, :string, size: 15, null: false
      add :status, :string, size: 15, null: false
      add :deleted_at, :utc_datetime
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime_usec, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:bbs_topics, [:forum_id, :slug], unique: true)
    create index(:bbs_topics, [:slug])
    create index(:bbs_topics, [:title])
    create index(:bbs_topics, [:body_editor])
    create index(:bbs_topics, [:status])

    create table(:bbs_posts) do
      add :user_id, :bigint, null: false
      add :forum_id, :bigint, null: false
      add :topic_id, :bigint, null: false
      add :parent_id, :bigint
      add :body, :text, null: false
      add :body_editor, :string, size: 15, null: false
      add :status, :string, size: 15, null: false
      add :deleted_at, :utc_datetime
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime_usec, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:bbs_posts, [:body_editor], unique: true)
    create index(:bbs_posts, [:status], unique: true)
  end

  def down do
    drop table(:bbs_posts)
    drop table(:bbs_topics)
    drop table(:bbs_forums)
  end
end
