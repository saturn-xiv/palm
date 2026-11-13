package com.github.saturn_xiv.palm.marigold.plugins.portal.repositories;

import org.springframework.data.repository.ListCrudRepository;
import org.springframework.stereotype.Repository;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.user.ByWechatMiniProgram;

@Repository("portal.r.user.by-wechat.mini-program")
public interface WechatMiniProgramUserRepository extends ListCrudRepository<ByWechatMiniProgram, Integer> {

}
