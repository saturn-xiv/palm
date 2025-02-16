package com.github.saturn_siv.palm.dahlia.config

import org.casbin.jcasbin.main.Enforcer
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.http.HttpMethod
import org.springframework.security.authentication.AuthenticationManager
import org.springframework.security.authentication.ProviderManager
import org.springframework.security.authentication.dao.DaoAuthenticationProvider
import org.springframework.security.config.annotation.method.configuration.EnableMethodSecurity
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
@EnableMethodSecurity(prePostEnabled = true, securedEnabled = true, jsr250Enabled = true)
class SecurityConfiguration {
    @Bean
    fun filterChain(http: HttpSecurity): SecurityFilterChain {
        http {
            authorizeHttpRequests {
//                authorize(HttpMethod.GET, "/users/sign-in", permitAll)
//                authorize(HttpMethod.POST, "/users/sign-in", permitAll)
//                authorize(HttpMethod.POST, "/graphql", permitAll)
//
//                authorize(HttpMethod.GET, "/cms/**", permitAll)
//                authorize(HttpMethod.GET, "/bbs/**", permitAll)
//                authorize(HttpMethod.GET, "/accounting/**", permitAll)
//                authorize(HttpMethod.GET, "/monitor/**", permitAll)

                authorize(HttpMethod.GET, "/vendors/**", permitAll)
                authorize(HttpMethod.GET, "/assets/**", permitAll)

//                authorize(HttpMethod.GET, "/rss/*", permitAll)
//                authorize(HttpMethod.GET, "/sitemap/*", permitAll)
//                authorize(HttpMethod.GET, "/sitemap.xml", permitAll)
//                authorize(HttpMethod.GET, "/robots.txt", permitAll)

//                TODO
//                authorize("/admin/**", hasAuthority("ROLE_ADMINISTRATOR"))

//                authorize(HttpMethod.GET, "/", permitAll)
            }
            formLogin {
                loginPage = "/users/sign-in"
                loginProcessingUrl = "/users/sign-in"
                permitAll = true
            }
            logout {
                logoutUrl = "/users/sign-out"
                logoutSuccessUrl = "/users/sign-in"
                permitAll = true
            }
            httpBasic { }
        }
        return http.build()
    }

    @Bean
    fun authProvider(): AuthenticationManager {
        val provider = DaoAuthenticationProvider()
        provider.setUserDetailsService(userDetailsService())
        provider.setPasswordEncoder(passwordEncoder())
        return ProviderManager(provider)
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

    @Autowired
    private lateinit var enforcer: Enforcer;
}