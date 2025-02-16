plugins {
    kotlin("jvm") version "1.9.25"
    kotlin("plugin.spring") version "1.9.25"
    id("org.springframework.boot") version "3.4.2"
    id("io.spring.dependency-management") version "1.1.7"
    kotlin("plugin.jpa") version "1.9.25"

    id("com.gorylenko.gradle-git-properties") version "2.4.2"
}

group = "com.github.saturn_siv.palm"
version = "2025.2.15-SNAPSHOT"

java {
    toolchain {
        languageVersion = JavaLanguageVersion.of(21)
    }
}

repositories {
    mavenCentral()
}

extra["springShellVersion"] = "3.4.0"

dependencies {
    implementation("org.springframework.boot:spring-boot-starter-amqp")
    implementation("org.springframework.boot:spring-boot-starter-cache")
    implementation("org.springframework.boot:spring-boot-starter-data-jpa")
    implementation("org.springframework.boot:spring-boot-starter-freemarker")
    implementation("org.springframework.boot:spring-boot-starter-graphql")
    implementation("org.springframework.boot:spring-boot-starter-mail")
    implementation("org.springframework.boot:spring-boot-starter-security")
    implementation("org.springframework.boot:spring-boot-starter-validation")
    implementation("org.springframework.boot:spring-boot-starter-web")
    implementation("com.fasterxml.jackson.module:jackson-module-kotlin")
    implementation("org.flywaydb:flyway-core")
    implementation("org.flywaydb:flyway-database-postgresql")
    implementation("org.flywaydb:flyway-mysql")
    implementation("org.jetbrains.kotlin:kotlin-reflect")
    implementation("org.springframework.session:spring-session-core")
    implementation("org.springframework.shell:spring-shell-starter")

    runtimeOnly("io.grpc:grpc-netty-shaded:1.70.0")
    implementation("io.grpc:grpc-protobuf:1.70.0")
    implementation("io.grpc:grpc-stub:1.70.0")
    compileOnly("org.apache.tomcat:annotations-api:6.0.53") // necessary for Java 9+

    // https://mvnrepository.com/artifact/com.google.crypto.tink/tink
    implementation("com.google.crypto.tink:tink:1.16.0")
    // https://mvnrepository.com/artifact/com.twilio.sdk/twilio
    implementation("com.twilio.sdk:twilio:10.6.9")
    // https://mvnrepository.com/artifact/com.github.wechatpay-apiv3/wechatpay-java
    implementation("com.github.wechatpay-apiv3:wechatpay-java:0.2.16")
    // https://mvnrepository.com/artifact/io.minio/minio
    implementation("io.minio:minio:8.5.17")
    // https://mvnrepository.com/artifact/org.opensearch.client/opensearch-java
    implementation("org.opensearch.client:opensearch-java:2.21.0")

    // https://mvnrepository.com/artifact/org.casbin/jcasbin
    implementation("org.casbin:jcasbin:1.79.0")
    // https://mvnrepository.com/artifact/org.casbin/jdbc-adapter
    implementation("org.casbin:jdbc-adapter:2.10.0")
    // https://mvnrepository.com/artifact/org.casbin/jcasbin-redis-watcher
    implementation("org.casbin:jcasbin-redis-watcher:1.8.0")

    runtimeOnly("com.mysql:mysql-connector-j")
    runtimeOnly("org.postgresql:postgresql")
    testImplementation("org.springframework.boot:spring-boot-starter-test")
    testImplementation("org.jetbrains.kotlin:kotlin-test-junit5")
    testImplementation("org.springframework:spring-webflux")
    testImplementation("org.springframework.amqp:spring-rabbit-test")
    testImplementation("org.springframework.graphql:spring-graphql-test")
    testImplementation("org.springframework.security:spring-security-test")
    testImplementation("org.springframework.shell:spring-shell-starter-test")
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
}

dependencyManagement {
    imports {
        mavenBom("org.springframework.shell:spring-shell-dependencies:${property("springShellVersion")}")
    }
}

kotlin {
    compilerOptions {
        freeCompilerArgs.addAll("-Xjsr305=strict")
    }
}

allOpen {
    annotation("jakarta.persistence.Entity")
    annotation("jakarta.persistence.MappedSuperclass")
    annotation("jakarta.persistence.Embeddable")
}

tasks.withType<Test> {
    useJUnitPlatform()
}
