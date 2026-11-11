defmodule Bougainvillea.Portal.Models do
  defimpl String.Chars, for: Bougainvillea.Portal.Models.EmailUser do
    def to_string(%Bougainvillea.Portal.Models.EmailUser{name: name, email: email}) do
      "#{name}<#{email}>"
    end
  end
end
