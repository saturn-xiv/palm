defmodule Mix.Tasks.Bougainvillea.DbSeed do
  use Mix.Task
  require Logger
  import Ecto.Query

  @shortdoc "Load the seed data from priv"

  @impl true
  def run(_args) do
    Mix.Task.run("app.start")

    priv_dir = :code.priv_dir(:bougainvillea)

    Bougainvillea.Repo.transaction(fn ->
      load_iso4217(Path.join([priv_dir, "iso4217", "list-one.xml"]))
    end)

    load_locales(Path.join(priv_dir, "locales"))

    Logger.info("done.")
  end

  def load_iso4217(file) do
    Logger.info("loading iso4217 from #{file}")
  end

  def load_locales(lang, key, value) when is_binary(value) do
    Logger.debug("found #{lang}.#{key}")
    now = DateTime.utc_now(:microsecond)
    if Bougainvillea.Portal.Models.Locale |> where([x], x.lang == ^lang and x.code == ^key) |> Bougainvillea.Repo.aggregate(:count) == 0 do
      %Bougainvillea.Portal.Models.Locale{lang: lang, code: key, message: value, updated_at: now} |> Bougainvillea.Repo.insert()
      {1, 1}
    else
      {1, 0}
    end
  end

  def load_locales(lang, key, value) when is_map(value) do
    Enum.reduce(value, {0, 0}, fn {k, v}, {found, inserted} ->
      {f, i} = load_locales(lang, "#{key}.#{k}", v)
      {found + f, inserted + i}
    end)
  end

  def load_locales(lang, file) do
    Logger.debug("load #{file} for #{lang}")
    {:ok, items} = YamlElixir.read_from_file(file)
    load_locales(lang, file |> Path.basename() |> Path.rootname(), items)
  end

  def load_locales(root) do
    Logger.info("loading locales from #{root}")
    {:ok, items} = File.ls(root)
    languages = Enum.filter(items, fn x -> File.dir?(Path.join(root, x)) end)

    globals =
      Enum.filter(items, fn x ->
        file = Path.join(root, x)
        ext = Path.extname(x)
        File.regular?(file) and ext in [".yaml", ".yml"]
      end)

    {found, inserted} =
      Enum.reduce(languages, {0, 0}, fn lang, {ft, it} ->
        {ft, it} =
          Enum.reduce(globals, {ft, it}, fn x, {fx, ix} ->
            {f, i} = load_locales(lang, Path.join(root, x))
            {fx + f, ix + i}
          end)

        {:ok, files} = File.ls(Path.join(root, lang))

        Enum.reduce(files, {ft, it}, fn x, {fx, ix} ->
          file = Path.join([root, lang, x])

          if File.regular?(file) do
            {f, i} = load_locales(lang, file)
            {fx + f, ix + i}
          else
            {fx, ix}
          end
        end)
      end)

    Logger.info("totally found #{found}, inserted #{inserted}")
  end
end
