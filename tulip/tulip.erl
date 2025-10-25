-module(tulip).
-export([foo/1, bar/1]).
-nifs([foo/1, bar/1]).
-on_load(init/0).

init() ->
    ok = erlang:load_nif("./libtulip", 0).

foo(_X) ->
    erlang:nif_error(nif_library_not_loaded).

bar(_Y) ->
    erlang:nif_error(nif_library_not_loaded).
