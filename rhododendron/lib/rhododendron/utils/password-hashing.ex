defmodule Rhododendron.PasswordHashing do
  def sign(password, salt_len) do
    salt = :crypto.strong_rand_bytes(salt_len)
    {:ok, _, hashed} = :jargon.hash(password, salt, :argon2id, 32, 12, 1, salt_len)
    {:ok, hashed}
  end

  def verify(hashed, password) do
    {:ok, true} = :jargon.verify(hashed, password)
    {:ok}
  end
end
