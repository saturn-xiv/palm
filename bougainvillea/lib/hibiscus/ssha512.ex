defmodule Hibiscus.Ssha512 do
  @prefix "{SSHA512}"

  # https://mad9scientist.com/dovecot-password-creation-php/
  # https://doc.dovecot.org/2.3/configuration_manual/howto/convert_password_schemes/
  def sign(password, salt_len) do
    salt = Hibiscus.Random.bytes(salt_len)
    hash = :crypto.hash(:sha512, password <> salt)
    "#{@prefix}#{(hash <> salt) |> Base.encode64(padding: false)}"
  end

  def verify(@prefix <> code, password) do
    {:ok, buf} = Base.decode64(code)
    <<hash::binary-size(64)>> <> salt = buf
    :crypto.hash(:sha512, password <> salt) == hash
  end

  def verify(_code, _password) do
    false
  end
end
