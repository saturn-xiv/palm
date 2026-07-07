defmodule Rhododendron.Ssha512 do
  # https://mad9scientist.com/dovecot-password-creation-php/
  # https://doc.dovecot.org/2.3/configuration_manual/howto/convert_password_schemes/
  def sign(data, salt_len) do
    salt = :crypto.strong_rand_bytes(salt_len)
    header() <> ((:crypto.hash(:sha512, data <> salt) <> salt) |> Base.encode64())
  end

  def verify(hashed, data) do
    if(String.starts_with?(hashed, header())) do
      {:ok, <<tmp::binary-size(64), salt::binary>>} =
        String.trim_leading(hashed, header()) |> Base.decode64()

      if :crypto.hash(:sha512, data <> salt) == tmp do
        {:ok}
      else
        {:error, "invalid salt"}
      end
    else
      {:error, "invalid ssha-512 message"}
    end
  end

  defp header do
    "{SSHA512}"
  end
end
