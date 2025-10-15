package com.github.saturn_xiv.palm.hyacinth.services;

import jakarta.annotation.Resource;
import jakarta.persistence.EntityManager;
import jakarta.persistence.PersistenceContext;
import jakarta.persistence.TypedQuery;

import org.springframework.stereotype.Service;

import com.github.saturn_xiv.palm.hyacinth.repositories.LogRepository;

@Service("palm.hyacinth.log-service")
public class LogService {

    public String version() {
        String hql = "SELECT VERSION()";
        TypedQuery<String> query = entityManager.createQuery(hql, String.class);
        return query.getSingleResult();
    }

    @PersistenceContext
    EntityManager entityManager;
    @Resource
    LogRepository logRepository;
}
