defmodule Tuberose.LeaveWord do
  use Ecto.Schema
  import Ecto.Changeset

  schema "leave_words" do
    field(:lang, :string)
    field(:ip, :string)
    field(:body, :string)
    field(:editor, :string)
    field(:status, :string)
    field(:published_at, :utc_datetime_usec)
    field(:deleted_at, :utc_datetime_usec)
    field(:version, :integer)

    timestamps(type: :utc_datetime_usec)
  end

  @doc false
  def changeset(leave_word, attrs) do
    leave_word
    |> cast(attrs, [:lang, :ip, :body, :editor, :status, :published_at, :deleted_at, :version])
    |> validate_required([:lang, :ip, :body, :editor, :status])
  end
end
