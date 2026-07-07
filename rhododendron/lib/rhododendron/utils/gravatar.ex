defmodule Rhododendron.Gravatar do
  # https://docs.gravatar.com/sdk/images/
  def avatar(email) do
    "https://gravatar.com/avatar/#{hash(email)}"
  end

  # https://docs.gravatar.com/rest/hash/
  defp hash(email) do
    it = email |> String.trim() |> String.downcase()
    :crypto.hash(:sha256, it) |> Base.encode16(case: :lower)
  end
end
