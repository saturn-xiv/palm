defmodule Tuberose.I18n do
  import Ecto.Query

  def t(lang, code) do
    it =
      from(p in Tuberose.Locale,
        where: [lang: ^lang, code: ^code],
        select: map(p, [:message])
      )
      |> first
      |> Tuberose.Repo.one()

    if it, do: it[:message], else: "#{lang}.#{code}"
  end
end
