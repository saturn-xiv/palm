defmodule Hibiscus.Sha256 do
  def compute(data) do
    :crypto.hash(:sha256, data) |> Base.encode16(case: :lower)
  end
end
