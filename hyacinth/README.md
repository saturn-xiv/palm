# Usage

```bash
mvn clean && mvn package
java -Dlogback.configurationFile=logback.xml -jar target/hyacinth-2025.8.15-jar-with-dependencies.jar -p 8080 -c config.toml
java -cp 'target/hyacinth-2025.8.15-jar-with-dependencies.jar:libs/*' com.github.saturn_xiv.palm.hyacinth.App -p 8080 -c config.toml
```
