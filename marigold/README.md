# MARIGOLD - WechatPay services

- Usage

  ```bash
  # clean & build
  mvn clean
  mvn package -Dmaven.test.skip=true
  # running on staging mode
  java -jar target/marigold-$(git describe --tags --abbrev=7 --always | sed 's/-g/-/')*.jar --spring.config.name=staging
  ```

## Resources

- [Maven Git Versioning Extension](https://github.com/qoomon/maven-git-versioning-extension)
- [External Application Properties](https://docs.spring.io/spring-boot/reference/features/external-config.html#features.external-config.files)
