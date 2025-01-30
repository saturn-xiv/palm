-module(marguerite).
-export([jwt_sign/6, jwt_verify/3, aes_encrypt/1, aes_decrypt/1, hmac_sign/1, hmac_verify/2, version/0]).
-nifs([jwt_sign/6, jwt_verify/3, aes_encrypt/1, aes_decrypt/1, hmac_sign/1, hmac_verify/2, version/0]).
-on_load(init/0).

init() ->
    ok = erlang:load_nif("./libmarguerite", 0).

jwt_sign(_U, _V, _W, _X, _Y, _Z) ->
    erlang:nif_error(nif_library_not_loaded).

jwt_verify(_X, _Y, _Z) ->
    erlang:nif_error(nif_library_not_loaded).

aes_encrypt(_X) ->
    erlang:nif_error(nif_library_not_loaded).

aes_decrypt(_X) ->
    erlang:nif_error(nif_library_not_loaded).

hmac_sign(_X) ->
    erlang:nif_error(nif_library_not_loaded).

hmac_verify(_X, _Y) ->
    erlang:nif_error(nif_library_not_loaded).

version() ->
    erlang:nif_error(nif_library_not_loaded).
