#include "tulip/tink.hpp"

#include <erl_nif.h>

static ERL_NIF_TERM foo_nif(ErlNifEnv* env, int argc,
                            const ERL_NIF_TERM argv[]) {
  int x, ret;
  if (!enif_get_int(env, argv[0], &x)) {
    return enif_make_badarg(env);
  }
  ret = x + 1;
  return enif_make_int(env, ret);
}

static ERL_NIF_TERM bar_nif(ErlNifEnv* env, int argc,
                            const ERL_NIF_TERM argv[]) {
  int y, ret;
  if (!enif_get_int(env, argv[0], &y)) {
    return enif_make_badarg(env);
  }
  ret = y - 1;
  return enif_make_int(env, ret);
}

static ErlNifFunc nif_funcs[] = {{"foo", 1, foo_nif}, {"bar", 1, bar_nif}};

ERL_NIF_INIT(tulip, nif_funcs, NULL, NULL, NULL, NULL)
