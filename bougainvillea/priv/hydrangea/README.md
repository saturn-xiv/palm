# HYDRANGEA CLIENT

```bash
# CREATE TOKEN
./bin/bougainvillea eval 'Bougainvillea.Release.help'
./bin/bougainvillea eval 'Bougainvillea.Release.create_user_by_email("Who am I", "change-me@gmail.com", "password")'
./bin/bougainvillea list-user
./bin/bougainvillea eval 'Bougainvillea.Release.add_role_for_user("xxx-xxx-xxx-xxx", "bougainvillea")'
./bin/bougainvillea eval 'Bougainvillea.Release.generate_token_by_email("change-me@gmail.com", ["sign-in"], 7)'

./hydrangea.sh echo 'Hello,Palm!'
./hydrangea.sh version
```
