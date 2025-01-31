defmodule Jasmine.Repo.Migrations.CreateQuestionnaires do
  use Ecto.Migration

  def up do
    create table(:questionnaire_forms) do
      add :user_id, :bigint, null: false
      add :uid, :string, size: 36, null: false
      add :title, :string, size: 127, null: false
      add :memo, :string, size: 511, null: false
      add :not_before, :utc_datetime, null: false
      add :expires_at, :utc_datetime, null: false
      add :deleted_at, :utc_datetime
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime, null: false
      add :created_at, :utc_datetime, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:questionnaire_forms, [:uid], unique: true)
    create index(:questionnaire_forms, [:title])
    create index(:questionnaire_forms, [:memo])

    create table(:questionnaire_fields) do
      add :form_id, :bigint, null: false
      add :uid, :string, size: 36, null: false
      add :label, :string, size: 63, null: false
      add :memo, :string, size: 511
      add :type, :string, size: 15, null: false
      add :priority, :integer, null: false, default: 0
      add :profile, :binary, null: false
      add :deleted_at, :utc_datetime
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime, null: false
      add :created_at, :utc_datetime, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:questionnaire_fields, [:uid], unique: true)
    create index(:questionnaire_fields, [:label])
    create index(:questionnaire_fields, [:memo])
    create index(:questionnaire_fields, [:type])

    create table(:questionnaire_values) do
      add :form_id, :bigint, null: false
      add :batch, :string, size: 31, null: false
      add :value, :binary, null: false
      add :deleted_at, :utc_datetime
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime, null: false
      add :created_at, :utc_datetime, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:questionnaire_values, [:batch], unique: true)
  end

  def down do
    drop table(:questionnaire_values)
    drop table(:questionnaire_fields)
    drop table(:questionnaire_forms)
  end
end
