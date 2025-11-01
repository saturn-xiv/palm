defmodule BambooWeb.HostsHTML do
  @moduledoc """
  This module contains pages rendered by PageController.

  See the `page_html` directory for all templates available.
  """
  use BambooWeb, :html

  embed_templates "hosts_html/*"
end
