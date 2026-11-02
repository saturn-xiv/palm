package com.github.saturn_xiv.palm.hibiscus.repositories;

import org.springframework.data.repository.ListCrudRepository;
import org.springframework.stereotype.Repository;

import com.github.saturn_xiv.palm.hibiscus.models.Log;

@Repository("r.setting")
public interface LogRepository extends ListCrudRepository<Log, Integer> {

}
