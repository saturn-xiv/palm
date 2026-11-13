package com.github.saturn_xiv.palm.marigold.plugins.portal.repositories;

import java.util.Date;

import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.ListCrudRepository;
import org.springframework.stereotype.Repository;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.Setting;

@Repository("portal.r.setting")
public interface SettingRepository extends ListCrudRepository<Setting, Integer> {
    @Query(value = "SELECT VERSION()", nativeQuery = true)
    String getDatabaseVersion();

    @Query(value = "SELECT CURRENT_TIMESTAMP", nativeQuery = true)
    Date getDatabaseCurrentTimestamp();
}
