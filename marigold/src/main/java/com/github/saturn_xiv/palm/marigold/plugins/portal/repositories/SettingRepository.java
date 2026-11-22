package com.github.saturn_xiv.palm.marigold.plugins.portal.repositories;

import java.time.Instant;
import java.util.Optional;

import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.ListCrudRepository;
import org.springframework.stereotype.Repository;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.Setting;
import com.github.saturn_xiv.palm.marigold.plugins.portal.models.user.Item;

@Repository("portal.r.setting")
public interface SettingRepository extends ListCrudRepository<Setting, Integer> {

    Optional<Setting> findByKeyAndUser(String key, Item user);

    @Query(value = "SELECT VERSION()", nativeQuery = true)
    String getDatabaseVersion();

    @Query(value = "SELECT CURRENT_TIMESTAMP", nativeQuery = true)
    Instant getDatabaseCurrentTimestamp();
}
