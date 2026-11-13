package com.github.saturn_xiv.palm.marigold.plugins.hydrangea.repositories;

import org.springframework.data.repository.ListCrudRepository;
import org.springframework.stereotype.Repository;

import com.github.saturn_xiv.palm.marigold.plugins.hydrangea.models.JobLog;

@Repository("hydrangea.r.job-log")
public interface JobLogRepository extends ListCrudRepository<JobLog, Integer> {

}
