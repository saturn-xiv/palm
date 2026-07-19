defmodule Rhododendron.Dao.Site do
  @title "site.title"
  @subhead "site.subhead"
  @keywords "site.keywords"
  @description "site.description"
  @author "site.author"
  @copyright "site.copyright"

  def title(lang) do
    Rhododendron.Dao.Locale.t(lang, @title)
  end

  def subhead(lang) do
    Rhododendron.Dao.Locale.t(lang, @subhead)
  end

  def description(lang) do
    Rhododendron.Dao.Locale.t(lang, @description)
  end

  def author() do
    try do
      Rhododendron.Dao.Setting.get(@title)
    rescue
      _ -> %{name: "", email: ""}
    end
  end

  def author(name, email) do
    Rhododendron.Dao.Setting.set(@author, %{name: name, email: email}, false)
  end

  def copyright() do
    try do
      Rhododendron.Dao.Setting.get(@copyright)
    rescue
      _ -> ""
    end
  end

  def copyright(s) do
    Rhododendron.Dao.Setting.set(@copyright, s, false)
  end

  def keywords(items) do
    Rhododendron.Dao.Setting.set(@keywords, items, false)
  end

  def keywords() do
    try do
      Rhododendron.Dao.Setting.get(@keywords)
    rescue
      _ -> []
    end
  end
end
