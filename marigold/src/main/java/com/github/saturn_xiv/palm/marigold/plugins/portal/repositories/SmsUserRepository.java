package com.github.saturn_xiv.palm.marigold.plugins.portal.repositories;

import org.springframework.data.repository.ListCrudRepository;
import org.springframework.stereotype.Repository;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.user.BySms;

@Repository("portal.r.user.by-sms")
public interface SmsUserRepository extends ListCrudRepository<BySms, Integer> {

}
