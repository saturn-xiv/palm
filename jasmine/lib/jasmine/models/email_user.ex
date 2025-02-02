defmodule Jasmine.Models.EmailUser do
  require Logger
  require Ecto.Query
  require Ecto.Changeset
  require Ecto.UUID

  # https://docs.gravatar.com/api/avatars/hash/
  def gravatar(email) do
    email = email |> String.trim() |> String.downcase()
    hash = :sha256 |> :crypto.hash(email) |> Base.encode16(case: :lower)
    "https://gravatar.com/avatar/#{hash}"
  end

  def create(user_id, name, email, password) do
    %Jasmine.EmailUser{
      user_id: user_id,
      name: name,
      email: email,
      password: password,
      avatar: gravatar(email),
      updated_at: DateTime.utc_now()
    }
    |> Jasmine.Repo.insert()

    Jasmine.Repo.get_by(Jasmine.EmailUser, email: email)
  end

  def email?(email) do
    Jasmine.Repo.one!(
      Ecto.Query.from(p in Jasmine.EmailUser, where: p.email == ^email, select: count())
    ) > 0
  end

  def confirm(id) do
    now = DateTime.utc_now()
    it = Jasmine.Repo.get!(Jasmine.EmailUser, id)
    Jasmine.Repo.update(Ecto.Changeset.change(it, %{confirmed_at: now, updated_at: now}))
  end
end
