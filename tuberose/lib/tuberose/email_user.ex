defmodule Tuberose.EmailUser do
  use Ecto.Schema
  import Ecto.Changeset

  schema "email_users" do
    belongs_to(:user, Tuberose.User)

    field(:real_name, :string)
    field(:nickname, :string)
    field(:email, :string)
    field(:password, :binary)
    field(:salt, :binary)
    field(:avatar, :string)
    field(:confirmed_at, :utc_datetime_usec)
    field(:deleted_at, :utc_datetime_usec)
    field(:version, :integer)

    timestamps(type: :utc_datetime_usec)
  end

  @doc false
  def changeset(email_user, attrs) do
    email_user
    |> cast(attrs, [
      :user_id,
      :real_name,
      :nickname,
      :email,
      :password,
      :salt,
      :avatar,
      :confirmed_at,
      :deleted_at,
      :version
    ])
    |> validate_required([:user_id, :real_name, :nickname, :email])
  end

  def gravatar(email), do: gravatar(email, nil, nil, nil)

  @doc """
  https://docs.gravatar.com/api/avatars/hash/
  """
  def gravatar(email, size, rating, default) do
    hash = :sha256 |> :crypto.hash(String.downcase(String.trim(email))) |> Base.encode16()
    query = %{}
    query = if size && size > 0, do: Map.put(query, "s", size), else: query
    query = if rating && length(rating) > 0, do: Map.put(query, "r", rating), else: query
    query = if default && length(default) > 0, do: Map.put(query, "d", default), else: query

    %URI{
      scheme: "https",
      host: "www.gravatar.com",
      path: "/avatar/#{hash}",
      query: URI.encode_query(query)
    }
    |> to_string()
  end
end
