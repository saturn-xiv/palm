package com.github.saturn_siv.palm.dahlia.plugins.cms.models

import org.springframework.security.core.GrantedAuthority
import org.springframework.security.core.userdetails.UserDetails
import java.time.Instant


class CurrentUser : UserDetails {
    private lateinit var user: User
    private lateinit var expiresAt: Instant

    override fun getAuthorities(): MutableCollection<out GrantedAuthority> {
//        TODO
        val items = mutableListOf<GrantedAuthority>()
        return items
    }

    override fun getPassword(): String {
        return "change-me"
    }

    override fun getUsername(): String {
        return this.user.uid
    }

    override fun isAccountNonExpired(): Boolean {
        return true
    }

    override fun isAccountNonLocked(): Boolean {
        return this.user.lockedAt != null
    }

    override fun isCredentialsNonExpired(): Boolean {
        return this.expiresAt.isAfter(Instant.now())
    }

    override fun isEnabled(): Boolean {
        return this.user.deletedAt != null
    }
}