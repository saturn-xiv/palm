import com.google.protobuf.gradle.id

plugins {
	java
	id("org.springframework.boot") version "4.0.5"
	id("io.spring.dependency-management") version "1.1.7"
	id("com.google.protobuf") version "0.9.5"
}

group = "com.github.saturn_xiv.palm"
version = "0.0.1-SNAPSHOT"

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
