defmodule Rhododendron.Dao.EmailUser do
  require Logger
  import Ecto.Query
  import Ecto.Changeset

  def exists?(email) do
    {:ok, email} = Rhododendron.Dao.EmailUser.Validators.email(email)

    Rhododendron.Repo.one!(
      from t in Rhododendron.EmailUser, where: t.email == ^email, select: count()
    ) > 0
  end

  def confirm!(item) do
    Logger.info("Confirm user #{item.email}<#{item.name}>.")

    Rhododendron.Repo.update!(
      change(item, %{version: item.version + 1, confirmed_at: DateTime.utc_now(:microsecond)})
    )
  end

  def set_password!(item, password) do
    {:ok, password} = Rhododendron.PasswordHashing.sign(password, password_salt_length())
    Logger.warning("Update user #{item.email}<#{item.name}> password.")
    Rhododendron.Repo.update!(change(item, %{password: password, version: item.version + 1}))
  end

  def create!(name, email, password) do
    {:ok, password} = Rhododendron.PasswordHashing.sign(password, password_salt_length())

    Logger.warning("Create user #{email}<#{name}>.")
    uid = Ecto.UUID.generate()
    Logger.debug("Create user #{uid}")
    %Rhododendron.User{name: name, uid: uid} |> Rhododendron.Repo.insert!()
    user = Rhododendron.Repo.get_by(Rhododendron.User, uid: uid)

    %Rhododendron.EmailUser{
      user_id: user.id,
      name: name,
      email: email,
      password: password,
      avatar: Rhododendron.Gravatar.avatar(email)
    }
    |> Rhododendron.Repo.insert!()
  end

  def password_salt_length do
    16
  end

  defmodule Validators do
    @name_min_len 2
    @name_max_len 31
    @password_min_len 8
    @email_min_len 6
    @email_max_len 63
    @email_regex ~r/^[a-z0-9._-]+@[a-z0-9.-]+\.[a-z]{2,5}$/

    def name(s) do
      s = s |> String.trim()
      l = String.length(s)

      if l >= @name_min_len and l <= @name_max_len do
        {:ok, s}
      else
        {:error, "Username must be between #{@name_min_len} and #{@name_max_len} characters"}
      end
    end

    def email(s) do
      s = s |> String.trim() |> String.downcase()
      l = String.length(s)

      if l >= @email_min_len and l <= @email_max_len do
        if s =~ @email_regex do
          {:ok, s}
        else
          {:error, "#{s} isn't a valid email address"}
        end
      else
        {:error, "Email must be between #{@email_min_len} and #{@email_max_len} characters"}
      end
    end

    def password(s) do
      if String.length(s) >= @password_min_len do
        {:ok, s}
      else
        {:error, "Password must be at least #{@password_min_len} characters"}
      end
    end
  end
end
