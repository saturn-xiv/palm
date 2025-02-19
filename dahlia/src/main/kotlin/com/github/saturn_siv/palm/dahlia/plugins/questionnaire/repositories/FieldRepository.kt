package com.github.saturn_siv.palm.dahlia.plugins.questionnaire.repositories

import com.github.saturn_siv.palm.dahlia.plugins.questionnaire.models.Field
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("dahlia.questionnaire.r.field.jpa")
interface FieldRepository : CrudRepository<Field, Int> {
}