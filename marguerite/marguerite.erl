-module(marguerite).
-export([hmac_sign/1, hmac_verify/2, version/0]).
-nifs([hmac_sign/1, hmac_verify/2, version/0]).
-on_load(init/0).

init() ->
    ok = erlang:load_nif("./libmarguerite", 0).

hmac_sign(_X) ->
    erlang:nif_error(nif_library_not_loaded).

hmac_verify(_X, _Y) ->
    erlang:nif_error(nif_library_not_loaded).

version() ->
    erlang:nif_error(nif_library_not_loaded).
