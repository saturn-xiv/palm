package com.github.saturn_siv.palm.dahlia.models

import org.springframework.security.core.GrantedAuthority
import org.springframework.security.core.userdetails.UserDetails

class Session : UserDetails {
    private lateinit var uid: String


    override fun getAuthorities(): MutableCollection<out GrantedAuthority> {
//        TODO
        val items = mutableListOf<GrantedAuthority>()
        return items
    }

    override fun getPassword(): String {
        return "change-me"
    }

    override fun getUsername(): String {
        return this.uid
    }

    override fun isAccountNonExpired(): Boolean {
//        TODO
        return true
    }

    override fun isAccountNonLocked(): Boolean {
//        TODO
        return true
    }

    override fun isCredentialsNonExpired(): Boolean {
//        TODO
        return true
    }

    override fun isEnabled(): Boolean {
//        TODO
        return true
    }
}