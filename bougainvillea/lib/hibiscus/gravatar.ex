defmodule Hibiscus.Gravatar do
  # https://docs.gravatar.com/sdk/images/
  def image(email) do
    "https://gravatar.com/avatar/#{hash(email)}"
  end

  # https://docs.gravatar.com/rest/hash/
  def hash(email) do
    email |> String.trim() |> String.downcase() |> Hibiscus.Sha256.compute()
  end
end
