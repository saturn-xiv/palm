defmodule Dahlia.Rbac do
  defmodule Roles do
    defstruct administrator: false, root: false, others: []

    def administrator do
      "administrator"
    end

    def root do
      "root"
    end
  end

  defimpl String.Chars, for: Roles do
    def to_string(%Roles{administrator: administrator, root: root, others: others}) do
      "#{Roles.administrator()}: #{administrator}, #{Roles.root()}: #{root}, others: [#{Enum.join(others, ", ")}]"
    end
  end
end
