package com.github.saturn_xiv.palm.marigold.plugins.portal.repositories;

import org.springframework.data.repository.ListCrudRepository;
import org.springframework.stereotype.Repository;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.Log;

@Repository("portal.r.log")
public interface LogRepository extends ListCrudRepository<Log, Integer> {

}
