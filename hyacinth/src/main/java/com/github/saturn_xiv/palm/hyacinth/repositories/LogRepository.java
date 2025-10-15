package com.github.saturn_xiv.palm.hyacinth.repositories;

import java.util.List;

import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.CrudRepository;
import org.springframework.data.repository.query.Param;
import org.springframework.stereotype.Repository;

import com.github.saturn_xiv.palm.hyacinth.models.Log;

@Repository("palm.hyacinth.log-repository")
public interface LogRepository extends CrudRepository<Log, Integer> {
    @Query("SELECT i FROM Log i WHERE i.timeTaking > :value ORDER BY i.createdAt DESC")
    List<Log> findTimeTakingLagerThan(@Param("value") int value);
}
