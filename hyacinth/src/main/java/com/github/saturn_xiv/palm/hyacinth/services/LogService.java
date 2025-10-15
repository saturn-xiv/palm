package com.github.saturn_xiv.palm.hyacinth.services;

import jakarta.annotation.Resource;
import jakarta.persistence.EntityManager;
import jakarta.persistence.PersistenceContext;
import jakarta.persistence.TypedQuery;

import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Service;

import com.github.saturn_xiv.palm.hyacinth.repositories.LogRepository;

@Service("palm.hyacinth.log-service")
public class LogService {

    public String version() {
        String hql;
        switch (this.driver()) {
            case "org.hsqldb.jdbc.JDBCDriver":
                hql = "SELECT DISTINCT DATABASE_VERSION()";
                break;
            default:
                hql = "SELECT VERSION()";
                break;
        }
        TypedQuery<String> query = this.entityManager.createQuery(hql, String.class);
        return query.getSingleResult();
    }

    public String driver() {
        return this.driverName;
    }

    @PersistenceContext
    EntityManager entityManager;
    @Resource
    LogRepository logRepository;
    @Value("${spring.datasource.driver-class-name}")
    String driverName;
}
