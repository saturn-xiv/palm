import com.google.protobuf.gradle.id

plugins {
	java
	id("org.springframework.boot") version "4.0.5"
	id("io.spring.dependency-management") version "1.1.7"
	id("com.google.protobuf") version "0.9.5"
}

group = "com.github.saturn_xiv.palm"
version = "2026.4.21"

java {
	toolchain {
		languageVersion = JavaLanguageVersion.of(25)
	}
}

repositories {
	mavenCentral()
}

extra["springGrpcVersion"] = "1.0.2"

dependencies {
	implementation("org.springframework.boot:spring-boot-starter-data-jpa")
	implementation("org.springframework.boot:spring-boot-starter-validation")
	implementation("io.grpc:grpc-services")
	implementation("org.springframework.grpc:spring-grpc-server-spring-boot-starter")
	
	// Source: https://mvnrepository.com/artifact/org.casbin/jcasbin
	implementation("org.casbin:jcasbin:1.99.0")
	// Source: https://mvnrepository.com/artifact/org.casbin/jcasbin-rabbitmq-watcher
	implementation("org.casbin:jcasbin-rabbitmq-watcher:1.3.0")
	// Source: https://mvnrepository.com/artifact/org.casbin/hibernate-adapter
	implementation("org.casbin:hibernate-adapter:1.5.0")
	// Source: https://mvnrepository.com/artifact/com.google.crypto.tink/tink
	implementation("com.google.crypto.tink:tink:1.21.0")
	// Source: https://mvnrepository.com/artifact/io.minio/minio
	implementation("io.minio:minio:9.0.0")
	// Source: https://mvnrepository.com/artifact/com.github.wechatpay-apiv3/wechatpay-java
	implementation("com.github.wechatpay-apiv3:wechatpay-java:0.2.17")
	// Source: https://mvnrepository.com/artifact/com.twilio.sdk/twilio
	implementation("com.twilio.sdk:twilio:11.4.0")

	runtimeOnly("com.mysql:mysql-connector-j")
	runtimeOnly("org.postgresql:postgresql")
	testImplementation("org.springframework.boot:spring-boot-starter-data-jpa-test")
	testImplementation("org.springframework.boot:spring-boot-starter-validation-test")
	testImplementation("org.springframework.grpc:spring-grpc-test")
	testRuntimeOnly("org.junit.platform:junit-platform-launcher")
}

dependencyManagement {
	imports {
		mavenBom("org.springframework.grpc:spring-grpc-dependencies:${property("springGrpcVersion")}")
	}
}

protobuf {
	protoc {
		artifact = "com.google.protobuf:protoc"
	}
	plugins {
		id("grpc") {
			artifact = "io.grpc:protoc-gen-grpc-java"
		}
	}
	generateProtoTasks {
		all().forEach {
			it.plugins {
				id("grpc") {
					option("@generated=omit")
				}
			}
		}
	}
}

tasks.withType<Test> {
	useJUnitPlatform()
}
