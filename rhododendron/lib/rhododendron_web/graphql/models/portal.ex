defmodule RhododendronWeb.Portal do
  def ok() do
    %{created_at: DateTime.utc_now(:microsecond)}
  end
end
