defmodule Bougainvillea.Portal.Release do
  @app :bougainvillea

  def db_seed do
    {:ok, _} = Application.ensure_all_started(@app)

    path = Application.app_dir(@app, "priv/repo/seeds.exs")

    if File.exists?(path) do
      Code.eval_file(path)
    end
  end
end
