-module(loquat).
-export([foo/1, bar/1]).

foo(X) ->
  call_cnode({foo, X}).
bar(Y) ->
  call_cnode({bar, Y}).

call_cnode(Msg) ->
  {any, 'loquat@palm.change-me.org'} ! {call, self(), Msg},
  receive
    {FromPid, Msg} ->
      io:format("Received ~p from ~p~n", [Msg, FromPid])
  end.
