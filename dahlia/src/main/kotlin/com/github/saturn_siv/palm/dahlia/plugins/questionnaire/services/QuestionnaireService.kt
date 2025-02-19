package com.github.saturn_siv.palm.dahlia.plugins.questionnaire.services

import com.github.saturn_siv.palm.dahlia.plugins.questionnaire.repositories.FieldRepository
import com.github.saturn_siv.palm.dahlia.plugins.questionnaire.repositories.FormRepository
import com.github.saturn_siv.palm.dahlia.plugins.questionnaire.repositories.PollRepository
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.stereotype.Service

@Service("dahlia.questionnaire.s.questionnaire")
class QuestionnaireService {
    @Autowired
    private lateinit var fieldRepository: FieldRepository

    @Autowired
    private lateinit var pollRepository: PollRepository

    @Autowired
    private lateinit var formRepository: FormRepository
}