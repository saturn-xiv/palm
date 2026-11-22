package com.github.saturn_xiv.palm.marigold.plugins.portal.repositories;

import java.util.Optional;

import org.springframework.data.repository.ListCrudRepository;
import org.springframework.stereotype.Repository;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.user.ByGoogleOauth2;

@Repository("portal.r.user.by-google-oauth2")
public interface GoogleOauth2UserRepository extends ListCrudRepository<ByGoogleOauth2, Integer> {
    Optional<ByGoogleOauth2> findByClientIdAndSub(String clientId, String sub);
}
