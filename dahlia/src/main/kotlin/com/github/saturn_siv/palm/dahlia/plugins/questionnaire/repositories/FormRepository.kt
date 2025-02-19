package com.github.saturn_siv.palm.dahlia.plugins.questionnaire.repositories

import com.github.saturn_siv.palm.dahlia.plugins.questionnaire.models.Form
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("dahlia.questionnaire.r.form.jpa")
interface FormRepository : CrudRepository<Form, Int> {
}