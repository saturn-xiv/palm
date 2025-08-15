# Usage

```bash
mvn clean && mvn package

java -jar target/hyacinth-2025.8.15.jar -h

java -Dlogback.configurationFile=logback.xml -cp 'target/hyacinth-2025.8.15.jar:libs/*' com.github.saturn_xiv.palm.hyacinth.App -p 8080 -c config.toml
```
