#define BOOST_TEST_MODULE lua
#include <boost/test/unit_test.hpp>

#include <iostream>

#include <lua.hpp>

BOOST_AUTO_TEST_CASE(plus) {
  lua_State* state = luaL_newstate();
  luaL_openlibs(state);

  const char lua_script[] = "function sum(a, b) return a+b; end";
  int load_stat =
      luaL_loadbuffer(state, lua_script, strlen(lua_script), lua_script);
  lua_pcall(state, 0, 0, 0);

  lua_getglobal(state, "sum");
  if (lua_isfunction(state, -1)) {
    lua_pushnumber(state, 5.0);
    lua_pushnumber(state, 6.0);
    lua_pcall(state, 2, 1, 0);
    double sumval = 0.0;
    if (!lua_isnil(state, -1)) {
      sumval = lua_tonumber(state, -1);
      lua_pop(state, 1);
    }
    std::cout << "sum=" << sumval << std::endl;
  }

  lua_close(state);
}
