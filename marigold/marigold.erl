-module(marigold).
-export([version/0]).
-nifs([version/0]).
-on_load(init/0).

init() ->
    ok = erlang:load_nif("libmarigold", 0).

version() ->
    erlang:nif_error(nif_library_not_loaded).
