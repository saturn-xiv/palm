-module(tulip).
-export([aes_encrypt/1, aes_decrypt/1, hmac_sign/1, hmac_verify/2, jwt_sign/3, jwt_sign/4, jwt_verify/2, jwt_verify/3, version/0]).
-nifs([aes_encrypt/1, aes_decrypt/1, hmac_sign/1, hmac_verify/2, jwt_sign/3, jwt_sign/4, jwt_verify/2, jwt_verify/3, version/0]).
-on_load(init/0).

init() ->
    ok = erlang:load_nif("libtulip", 0).

aes_encrypt(_PLAIN) ->
    erlang:nif_error(nif_library_not_loaded).

aes_decrypt(_CODE) ->
    erlang:nif_error(nif_library_not_loaded).

hmac_sign(_PLAIN) ->
    erlang:nif_error(nif_library_not_loaded).

hmac_verify(_CODE, _PLAIN) ->
    erlang:nif_error(nif_library_not_loaded).

jwt_sign(_ISSUER, _SUBJECT, _TTL_IN_SECONDS) ->
    erlang:nif_error(nif_library_not_loaded).

jwt_sign(_ISSUER, _SUBJECT, _AUDIENCES, _TTL_IN_SECONDS) ->
    erlang:nif_error(nif_library_not_loaded).

jwt_verify(_TOKEN, _ISSUER) ->
    erlang:nif_error(nif_library_not_loaded).

jwt_verify(_TOKEN, _ISSUER, _AUDIENCE) ->
    erlang:nif_error(nif_library_not_loaded).


version() ->
    erlang:nif_error(nif_library_not_loaded).
