package com.github.saturn_siv.palm.dahlia.plugins.cms.repositories

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.GoogleOauth2User
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("jpa.dahlia.cms.r.google-oauth2-user.jpa")
interface GoogleOauth2UserRepository : CrudRepository<GoogleOauth2User, Int> {
}