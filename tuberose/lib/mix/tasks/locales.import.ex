defmodule Mix.Tasks.Tuberose.Locales.Import do
  require Logger

  use Mix.Task

  @shortdoc "Import locales into from filesystem"

  @moduledoc """
  Usage:
  mix tuberose.locales.import --folder FOLDER
  """

  @requirements ["app.start"]
  @impl Mix.Task
  def run(args) do
    {parsed, _} = OptionParser.parse!(args, strict: [folder: :string])

    root = parsed[:folder]

    unless root do
      raise ArgumentError, message: "empty folder"
    end

    Path.wildcard("#{root}/*/*.json")
    |> Enum.map(fn it ->
      lang = Path.dirname(it) |> Path.basename()
      load_json_file(lang, it)
    end)

    Path.wildcard("#{parsed[:folder]}/*.json")
    |> Enum.map(fn it ->
      Logger.info("load locale from #{it}")
    end)

    # Tuberose.Repo.transaction(fn ->
    #   %Tuberose.Log{
    #     user_id: user.id,
    #     plugin: "core",
    #     ip: "localhost",
    #     level: "warning",
    #     resource_type: "user",
    #     message: "apply to role(#{parsed[:role]})"
    #   }
    #   |> Tuberose.Repo.insert()
    # end)
  end

  def load_json_file(lang, file) do
    Logger.info("load locale from #{file} for #{lang}")
    zone = Path.rootname(file) |> Path.basename()
    json = Poison.decode!(File.read!(file))

    Enum.each(json, fn {k, v} ->
      loop_json(lang, zone, k, v)
    end)
  end

  def loop_json(lang, zone, key, value) when is_binary(value) do
    code = "#{zone}.#{key}"

    unless Tuberose.Repo.get_by(Tuberose.Locale, lang: lang, code: code) do
      Logger.debug("save #{lang}.#{code}")
      %Tuberose.Locale{lang: lang, code: code, message: value} |> Tuberose.Repo.insert()
    end
  end

  def loop_json(lang, zone, key, value) when is_map(value) do
    Enum.each(value, fn {k, v} ->
      loop_json(lang, "#{zone}.#{key}", k, v)
    end)
  end
end
