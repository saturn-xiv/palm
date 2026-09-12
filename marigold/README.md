# MARIGOLD - WechatPay services

- Usage

  ```bash
  # clean & build
  mvn clean
  mvn package -Dmaven.test.skip=true
  # running
  java -jar target/marigold-2026.7.28.jar --spring.config.name=staging
  ```

## Resources

- [Maven Git Versioning Extension](https://github.com/qoomon/maven-git-versioning-extension)
- [External Application Properties](https://docs.spring.io/spring-boot/reference/features/external-config.html#features.external-config.files)
