package com.github.saturn_siv.palm.dahlia.config

import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.security.authentication.dao.DaoAuthenticationProvider
import org.springframework.security.config.annotation.web.builders.HttpSecurity
import org.springframework.security.config.annotation.web.configuration.EnableWebSecurity
import org.springframework.security.config.annotation.web.invoke
import org.springframework.security.core.userdetails.User
import org.springframework.security.core.userdetails.UserDetailsService
import org.springframework.security.crypto.argon2.Argon2PasswordEncoder
import org.springframework.security.crypto.password.PasswordEncoder
import org.springframework.security.provisioning.InMemoryUserDetailsManager
import org.springframework.security.web.SecurityFilterChain

@Configuration
@EnableWebSecurity
class SecurityConfig {
    @Bean
    fun filterChain(http: HttpSecurity): SecurityFilterChain {
        http {
            authorizeHttpRequests {
                authorize("/vendors/**", permitAll)
                authorize("/user/**", hasAuthority("ROLE_USER"))
            }
            formLogin {
                loginPage = "/users/sign-in"
            }
            httpBasic { }
        }
        return http.build()
    }

    @Bean
    fun authProvider(): DaoAuthenticationProvider {
        val provider = DaoAuthenticationProvider()
        provider.setUserDetailsService(userDetailsService())
        provider.setPasswordEncoder(passwordEncoder())
        return provider
    }

    @Bean
    fun userDetailsService(): UserDetailsService {
        val users = User.builder()
        val manager = InMemoryUserDetailsManager()
        manager.createUser(users.username("user").password("change-me").roles("USER").build())
        manager.createUser(users.username("admin").password("change-me").roles("USER", "ADMIN").build())
        return manager
    }

    @Bean
    fun passwordEncoder(): PasswordEncoder {
        return Argon2PasswordEncoder.defaultsForSpringSecurity_v5_8();
    }

}