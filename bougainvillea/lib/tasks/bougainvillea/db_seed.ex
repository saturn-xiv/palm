defmodule Mix.Tasks.Bougainvillea.DbSeed do
  use Mix.Task
  require Logger
  import SweetXml

  @shortdoc "Load the seed data from priv"

  @impl true
  def run(_args) do
    Mix.Task.run("app.start")

    priv_dir = :code.priv_dir(:bougainvillea)

    Bougainvillea.Repo.transaction(fn ->
      load_locales(Path.join(priv_dir, "locales"))
    end)
    Bougainvillea.Repo.transaction(fn ->
      if Bougainvillea.Portal.Dao.Currency.count() == 0 do
        load_iso4217(Path.join([priv_dir, "iso4217", "list-one.xml"]))
        count = Bougainvillea.Portal.Dao.Currency.count()
        Logger.info("totally inserted #{count} items")
      else
        Logger.warning("table currencies isn't empty")
      end
    end)

    Logger.info("done.")
  end

  def load_iso4217(file) do
    Logger.info("loading iso4217 from #{file}")
    {:ok, doc} = File.read(file)
    items = doc |> xpath(
      ~x"//CcyTbl/CcyNtry"l,
      country: ~x"./CtryNm/text()",
      name: ~x"./CcyNm/text()",
      code: ~x"./Ccy/text()",
      number: ~x"./CcyNbr/text()",
      units: ~x"./CcyMnrUnts/text()",
      fund: ~x"./CcyNm/@IsFund"
    )

    Enum.each(items, fn it ->
      Logger.info("found #{it[:name]}(#{it[:code]},#{it[:number]})")
      if it[:number] do
        units = case it[:units] do
          nil -> nil
          ~c"N.A." -> nil
          v -> List.to_integer(v)
        end
        {:ok, fund} = case it[:fund] do
          nil -> {:ok, nil}
          ~c"true" -> {:ok, true}
          v -> {:error, "invalid value #{v}"}
        end
        {:ok, _} = Bougainvillea.Portal.Dao.Currency.create(List.to_string(it[:name]), List.to_string(it[:code]), List.to_string(it[:country]), List.to_string(it[:number]), units, fund)
      end
    end)
  end

  def load_locales(lang, key, value) when is_binary(value) do
    Logger.debug("found #{lang}.#{key}")
    if Bougainvillea.Portal.Dao.Locale.exist?(lang, key) do
      {:ok, _} =  Bougainvillea.Portal.Dao.Locale.create(lang, key, value)
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
