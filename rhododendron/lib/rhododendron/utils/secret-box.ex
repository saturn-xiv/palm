defmodule Rhododendron.SecretBox do
  @algorithm :aes_256_gcm

  def encrypt(plain, iv_len \\ 16, aad \\ "") do
    iv = :crypto.strong_rand_bytes(iv_len)

    {cipher, tag} =
      :crypto.crypto_one_time_aead(
        @algorithm,
        key(),
        iv,
        plain,
        aad,
        true
      )

    {cipher, tag, iv}
  end

  def decrypt(cipher, tag, iv, aad \\ "") do
    :crypto.crypto_one_time_aead(@algorithm, key(), iv, cipher, aad, tag, false)
  end

  defp key do
    Application.fetch_env!(:rhododendron, :secret_box_key) |> Base.decode64!()
  end
end
