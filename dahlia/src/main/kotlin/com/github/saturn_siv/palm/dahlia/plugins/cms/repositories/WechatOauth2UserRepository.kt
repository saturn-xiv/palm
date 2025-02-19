package com.github.saturn_siv.palm.dahlia.plugins.cms.repositories

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.wechat.Oauth2User
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("jpa.dahlia.cms.r.wechat.oauth2-user.jpa")
interface WechatOauth2UserRepository : CrudRepository<Oauth2User, Int> {
}