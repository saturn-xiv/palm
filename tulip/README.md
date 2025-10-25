# Usage

- Testing NIFs

```erlang
$ erl
> c(tulip).
> tulip:version().
> HELLO = <<"Hello, tulip!">>.
> HI = <<"Hi, tulip!">>.

> CODE = tulip:aes_encrypt(HELLO).
> PLAIN = tulip:aes_decrypt(CODE).
> PLAIN == HELLO.

> PASSWORD = tulip:hmac_sign(HELLO).
> tulip:hmac_verify(PASSWORD, HELLO).
> tulip:hmac_verify(PASSWORD, HI).

> ISSUER = <<"who-am-i">>.
> TOKEN = tulip:jwt_sign(ISSUER, HELLO, 3600).
> {SUBJECT} = tulip:jwt_verify(TOKEN, ISSUER).
> SUBJECT == HELLO.
> SUBJECT == HI.
```
