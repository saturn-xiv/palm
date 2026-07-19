defmodule RhododendronWeb.Resolvers.Portal.EmailUser do
  require Logger

  def sign_in(
        _parent,
        %{email: email, password: password},
        %{
          context: %RhododendronWeb.Session{client_ip: client_ip}
        }
      ) do
    {:ok, email} = Rhododendron.Dao.EmailUser.Validators.email(email)

    e_user =
      Rhododendron.Repo.get_by!(Rhododendron.EmailUser, email: email)
      |> Rhododendron.Repo.preload(:user)

    {:ok} = Rhododendron.PasswordHashing.verify(e_user.password, password)

    if e_user.deleted_at do
      raise ArgumentError, "User #{email} is disabled."
    end

    if e_user.locked_at do
      raise ArgumentError, "User #{email} is locked."
    end

    unless e_user.confirmed_at do
      raise ArgumentError, "User #{email} isn't confirmed yet."
    end

    Rhododendron.Repo.transact(fn ->
      Rhododendron.Dao.User.sign_in(e_user.user, :email, e_user.email, client_ip)
      {:ok, true}
    end)

    {:ok, Rhododendron.Dao.User.build_sign_in_response(e_user.user, :email, e_user.email)}
  end

  def sign_up(
        _parent,
        %{name: name, email: email, password: password, lang: lang, timezone: timezone},
        %{
          context: %RhododendronWeb.Session{client_ip: client_ip}
        }
      ) do
    {:ok, lang} = Rhododendron.Dao.User.Validators.locale(lang)
    {:ok, timezone} = Rhododendron.Dao.User.Validators.timezone(timezone)
    {:ok, name} = Rhododendron.Dao.EmailUser.Validators.name(name)
    {:ok, email} = Rhododendron.Dao.EmailUser.Validators.email(email)
    {:ok, password} = Rhododendron.Dao.EmailUser.Validators.password(password)

    if Rhododendron.Dao.EmailUser.exists?(email) do
      {:error, "User #{email} already exists"}
    else
      Logger.warning("create user #{email}<#{name}>")

      Rhododendron.Repo.transact(fn ->
        Rhododendron.Dao.EmailUser.create!(name, email, password, lang, timezone)

        it =
          Rhododendron.Repo.get_by!(Rhododendron.EmailUser, email: email)
          |> Rhododendron.Repo.preload(:user)

        Rhododendron.Dao.Log.info!(it.user_id, :auth, client_ip, %{}, "Sign up.")
        {:ok, RhododendronWeb.Portal.ok()}
      end)
    end
  end
end
