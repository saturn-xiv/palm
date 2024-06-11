defmodule TuberoseWeb.Resolvers.EmailUser do
  require Logger
  import Ecto.Query
  import Ecto.Changeset

  def confirm(_parent, %{user: user, home: home}, %{context: context}) do
    user = String.trim(user) |> String.downcase()
    {:ok, home} = Tuberose.Validation.url(home)

    it =
      from(p in Tuberose.EmailUser,
        where: p.nickname == ^user or p.email == ^user,
        select: map(p, [:email, :real_name, :deleted_at, :confirmed_at])
      )
      |> first
      |> Tuberose.Repo.one()

    unless it do
      raise ArgumentError, message: "User is not exists"
    end

    if it.deleted_at do
      raise ArgumentError, message: "User is disabled"
    end

    if it.confirmed_at do
      raise ArgumentError, message: "User is active"
    end

    send_email(it.email, home, context.locale, :confirm)
    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def confirm_by_token(_parent, %{token: token}, %{context: context}) do
    {subject, _} =
      Tuberose.Atropa.Client.jwt_verify(token, to_string(:tuberose), to_string(:confirm))

    it = Tuberose.Repo.get_by(Tuberose.EmailUser, nickname: subject)

    unless it do
      raise ArgumentError, message: "User is not exists"
    end

    if it.deleted_at do
      raise ArgumentError, message: "User is disabled"
    end

    if it.confirmed_at do
      raise ArgumentError, message: "User is active"
    end

    Tuberose.Repo.transaction(fn ->
      Tuberose.Repo.update(change(it, %{confirmed_at: DateTime.utc_now()}))

      %Tuberose.Log{
        user_id: it.user_id,
        plugin: "core",
        ip: context.client_ip,
        level: "info",
        resource_type: "email_user",
        message: "active by email."
      }
      |> Tuberose.Repo.insert()
    end)

    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def unlock(_parent, %{user: user, home: home}, %{context: context}) do
    user = String.trim(user) |> String.downcase()
    {:ok, home} = Tuberose.Validation.url(home)

    it =
      from(p in Tuberose.EmailUser,
        where: p.nickname == ^user or p.email == ^user,
        select: map(p, [:user_id, :email, :real_name, :deleted_at, :confirmed_at])
      )
      |> first
      |> Tuberose.Repo.one()

    unless it do
      raise ArgumentError, message: "User isn't exists"
    end

    if it.deleted_at do
      raise ArgumentError, message: "User is disabled"
    end

    unless it.confirmed_at do
      raise ArgumentError, message: "User is inactive"
    end

    ur = Tuberose.Repo.get(Tuberose.User, it.user_id)

    if ur.deleted_at do
      raise ArgumentError, message: "User is disabled"
    end

    unless ur.locked_at do
      raise ArgumentError, message: "User isn't locked"
    end

    send_email(it.email, home, context.locale, :unlock)
    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def unlock_by_token(_parent, %{token: token}, %{context: context}) do
    {subject, _} =
      Tuberose.Atropa.Client.jwt_verify(token, to_string(:tuberose), to_string(:unlock))

    it = Tuberose.Repo.get_by(Tuberose.EmailUser, nickname: subject)

    unless it do
      raise ArgumentError, message: "User isn't exists"
    end

    if it.deleted_at do
      raise ArgumentError, message: "User is disabled"
    end

    unless it.confirmed_at do
      raise ArgumentError, message: "User is inactive"
    end

    ur = Tuberose.Repo.get(Tuberose.User, it.user_id)

    if ur.deleted_at do
      raise ArgumentError, message: "User is disabled"
    end

    unless ur.locked_at do
      raise ArgumentError, message: "User isn't locked"
    end

    Tuberose.Repo.transaction(fn ->
      Tuberose.Repo.update(change(ur, %{locked_at: nil}))

      %Tuberose.Log{
        user_id: ur.id,
        plugin: "core",
        ip: context.client_ip,
        level: "info",
        resource_type: "email_user",
        message: "unlocked by email."
      }
      |> Tuberose.Repo.insert()
    end)

    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def forgot_password(_parent, %{user: user, home: home}, %{context: context}) do
    user = String.trim(user) |> String.downcase()
    {:ok, home} = Tuberose.Validation.url(home)

    it =
      from(p in Tuberose.EmailUser,
        where: p.nickname == ^user or p.email == ^user,
        select: map(p, [:user_id, :email, :real_name, :deleted_at, :confirmed_at])
      )
      |> first
      |> Tuberose.Repo.one()

    unless it do
      raise ArgumentError, message: "User isn't exists"
    end

    if it.deleted_at do
      raise ArgumentError, message: "User is disabled"
    end

    unless it.confirmed_at do
      raise ArgumentError, message: "User is inactive"
    end

    ur = Tuberose.Repo.get(Tuberose.User, it.user_id)

    if ur.deleted_at do
      raise ArgumentError, message: "User is disabled"
    end

    if ur.locked_at do
      raise ArgumentError, message: "User is locked"
    end

    send_email(it.email, home, context.locale, :"reset-password")
    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def reset_password(_parent, %{token: token, password: password, home: home}, %{context: context}) do
    {:ok, password} = Tuberose.Validation.password(password)
    {:ok, home} = Tuberose.Validation.url(home)

    {subject, _} =
      Tuberose.Atropa.Client.jwt_verify(token, to_string(:tuberose), "reset-password")

    it = Tuberose.Repo.get_by(Tuberose.EmailUser, nickname: subject)

    unless it do
      raise ArgumentError, message: "User isn't exists"
    end

    if it.deleted_at do
      raise ArgumentError, message: "User is disabled"
    end

    unless it.confirmed_at do
      raise ArgumentError, message: "User is inactive"
    end

    ur = Tuberose.Repo.get(Tuberose.User, it.user_id)

    if ur.deleted_at do
      raise ArgumentError, message: "User is disabled"
    end

    if ur.locked_at do
      raise ArgumentError, message: "User is locked"
    end

    {password, salt} = Tuberose.Atropa.Client.hmac_sign(password, 16)

    Tuberose.Repo.transaction(fn ->
      Tuberose.Repo.update(change(it, %{password: password, salt: salt}))

      %Tuberose.Log{
        user_id: ur.id,
        plugin: "core",
        ip: context.client_ip,
        level: "info",
        resource_type: "email_user",
        message: "password has been reset by email."
      }
      |> Tuberose.Repo.insert()
    end)

    send_email(it.email, home, context.locale, :"password-changed")
    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def sign_up(
        _parent,
        %{
          real_name: real_name,
          nickname: nickname,
          email: email,
          password: password,
          home: home,
          timezone: timezone
        },
        %{context: context}
      ) do
    {:ok, nickname} = Tuberose.Validation.code(nickname, 3)
    {:ok, real_name} = Tuberose.Validation.label(real_name, 2)
    {:ok, email} = Tuberose.Validation.email(email)
    {:ok, password} = Tuberose.Validation.password(password)
    {:ok, timezone} = Tuberose.Validation.timezone(timezone)
    {:ok, home} = Tuberose.Validation.url(home)

    avatar = Tuberose.EmailUser.gravatar(email)
    {password, salt} = Tuberose.Atropa.Client.hmac_sign(password, 16)

    Logger.info("create user #{real_name}<#{email}>")

    Tuberose.Repo.transaction(fn ->
      if Tuberose.Repo.get_by(Tuberose.EmailUser, email: email) do
        raise ArgumentError, message: "Email already exists"
      end

      if Tuberose.Repo.get_by(Tuberose.EmailUser, nickname: nickname) do
        raise ArgumentError, message: "Nickname already exists"
      end

      uid = Ecto.UUID.generate()
      %Tuberose.User{uid: uid, lang: context.locale, timezone: timezone} |> Tuberose.Repo.insert()
      user = Tuberose.Repo.get_by(Tuberose.User, uid: uid)

      %Tuberose.EmailUser{
        user_id: user.id,
        email: email,
        password: password,
        salt: salt,
        nickname: nickname,
        real_name: real_name,
        avatar: avatar
      }
      |> Tuberose.Repo.insert()

      %Tuberose.Log{
        user_id: user.id,
        plugin: "core",
        ip: context.client_ip,
        level: "info",
        resource_type: "email_user",
        message: "signed up by email"
      }
      |> Tuberose.Repo.insert()
    end)

    send_email(email, home, context.locale, :confirm)
    {:ok, %{:created_at => DateTime.utc_now()}}
  end

  defp send_email(email, home, locale, action) do
    email_user = Tuberose.Repo.get_by(Tuberose.EmailUser, email: email)
    subject = Tuberose.I18N.t(locale, "users.mailer.#{action}.subject")

    args =
      cond do
        action == :confirm or action == :unlock or action == :"reset-password" ->
          not_before = %Google.Protobuf.Timestamp{
            seconds: (DateTime.utc_now() |> DateTime.to_unix()) - 1
          }

          expires_at = %Google.Protobuf.Timestamp{
            seconds: DateTime.utc_now() |> Timex.shift(hours: 2) |> DateTime.to_unix()
          }

          %{
            home: home,
            token:
              Tuberose.Atropa.Client.jwt_sign(
                to_string(:tuberose),
                email_user.nickname,
                [to_string(action)],
                not_before,
                expires_at
              )
          }

        action == :"password-changed" ->
          %{}

        action == :"change-email" or action == :"email-changed" ->
          %{email: email}
      end

    body =
      Mustache.render(
        Tuberose.I18N.t(locale, "users.mailer.#{action}.body"),
        Map.put(args, :recipient, email_user.real_name)
      )

    Logger.info("send a #{action} email to #{email_user.real_name}<#{email_user.email}>")
    Logger.debug("#{subject}:\n#{body}")

    task = %Palm.Atropa.V1.EmailSendRequest{
      subject: subject,
      body: %Palm.Atropa.V1.EmailSendRequest.Body{text: body, html: true},
      cc: [],
      bcc: [],
      attachments: []
    }

    Tuberose.Amqp.Client.produce(:emails, :protobuf, Palm.Atropa.V1.EmailSendRequest.encode(task))
  end
end
