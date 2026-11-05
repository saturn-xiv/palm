defmodule Bougainvillea.Controllers.Api do
  def health_check() do
    %{
      version: Application.spec(:bougainvillea, :vsn) |> to_string,
      created_at: DateTime.utc_now(:microsecond)
    }
  end
end
