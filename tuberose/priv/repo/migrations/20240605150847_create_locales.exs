defmodule Tuberose.Repo.Migrations.CreateLocales do
  use Ecto.Migration

  def change do
    create table(:locales) do
      add(:lang, :string, size: 15, null: false)
      add(:code, :string, size: 255, null: false)
      add(:message, :text, null: false)
      add(:version, :integer, default: 0, null: false)

      timestamps(type: :utc_datetime_usec)
    end

    create(unique_index(:locales, [:lang, :code]))
    create(index(:locales, [:lang]))
    create(index(:locales, [:code]))
  end
end
