package com.github.saturn_xiv.palm.marigold.plugins.portal.repositories;

import org.springframework.data.repository.ListCrudRepository;
import org.springframework.stereotype.Repository;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.user.ByWechatOauth2;

@Repository("portal.r.user.by-wechat.oauth2")
public interface WechatOauth2UserRepository extends ListCrudRepository<ByWechatOauth2, Integer> {

}
