defmodule Hibiscus.Sha512 do
  def compute(data) do
    :crypto.hash(:sha512, data) |> Base.encode16(case: :lower)
  end
end
