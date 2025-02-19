package com.github.saturn_siv.palm.dahlia.plugins.cms.repositories

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.wechat.MiniProgramUser
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("jpa.dahlia.cms.r.wechat.mini-program-user.jpa")
interface WechatMiniProgramUserRepository : CrudRepository<MiniProgramUser, Int> {
}