# MARIGOLD - An online education solution

## Development

- Database schemes

```bash
# add postgresql driver
$ ~/local/liquibase-5.0.4/liquibase lpm add postgresql

$ ~/local/liquibase-5.0.4/liquibase update
$ ~/local/liquibase-5.0.4/liquibase rollback-count --count=1
$ ~/local/liquibase-5.0.4/liquibase validate
$ ~/local/liquibase-5.0.4/liquibase status
```

## Deployment

```bash
# clean & build
mvn clean
mvn package -Dmaven.test.skip=true

# running on staging mode
java -Xmx2048m -Xms256m -jar target/marigold-*.jar --spring.config.name=staging
```

## Resources

- [Maven Git Versioning Extension](https://github.com/qoomon/maven-git-versioning-extension)
- [External Application Properties](https://docs.spring.io/spring-boot/reference/features/external-config.html#features.external-config.files)
