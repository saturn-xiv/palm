defmodule Rhododendron.Dao.EmailUser do
  require Logger

  def create(name, email, password) do
    {:ok, name} = Rhododendron.Dao.EmailUser.Validators.name(name)
    {:ok, email} = Rhododendron.Dao.EmailUser.Validators.email(email)
    {:ok, _password} = Rhododendron.Dao.EmailUser.Validators.password(password)

    Logger.warning("Create user #{email}<#{name}>.")
    # TODO
    {:ok}
  end

  defmodule Validators do
    @name_min_len 2
    @name_max_len 31
    @password_min_len 8
    @email_min_len 6
    @email_max_len 127
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
