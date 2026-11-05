defmodule Bougainvillea.Portal.Services.EmailUser do
  require Logger

  @password_min_length 6
  @password_max_length 6
  @email_max_length 31
  @name_min_length 2
  @name_max_length 31

  def set_password(email, password) do
    {:ok, email} = Hibiscus.Validator.email(email)

    {:ok, password} =
      Hibiscus.Validator.password(password, @password_min_length, @password_max_length)

    it = Bougainvillea.Repo.get_by(Bougainvillea.Portal.Models.EmailUser, email: email)
    Logger.warning("set password for #{it.name}<#{it.email}>")
    {:ok, _} = Bougainvillea.Portal.Dao.EmailUser.set_password(it.id, password)
    {:ok, it}
  end

  def create(
        name,
        email,
        password,
        lang \\ Bougainvillea.Portal.default_lang(),
        timezone \\ Bougainvillea.Portal.default_timezone()
      ) do
    {:ok, email} = Hibiscus.Validator.email(email, @email_max_length)
    {:ok, name} = Hibiscus.Validator.name(name, @name_min_length, @name_max_length)

    {:ok, password} =
      Hibiscus.Validator.password(password, @password_min_length, @password_max_length)

    # TODO validate lang timezone

    Logger.warning("create user for #{name}<#{email}>")

    if Bougainvillea.Portal.Dao.EmailUser.exist?(email) do
      {:error, "Email user #{email} already exists"}
    else
      {:ok, password} = Loquat.Argon2.sign(password)
      sn = Hibiscus.uuid()
      {:ok, _} = Bougainvillea.Portal.Dao.User.create(sn, name, lang, timezone)
      user = Bougainvillea.Repo.get_by(Bougainvillea.Portal.Models.User, sn: sn)
      {:ok, _} = Bougainvillea.Portal.Dao.EmailUser.create(user.id, name, email, password)

      email_user = Bougainvillea.Repo.get_by(Bougainvillea.Portal.Models.EmailUser, email: email)
      {:ok, email_user}
    end
  end
end
