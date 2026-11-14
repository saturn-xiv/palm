package com.github.saturn_xiv.palm.marigold.plugins.portal.repositories;

import java.util.List;
import java.util.Optional;

import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.ListCrudRepository;
import org.springframework.stereotype.Repository;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.Locale;

@Repository("portal.r.locale")
public interface LocaleRepository extends ListCrudRepository<Locale, Integer> {
    Optional<Locale> findByLangAndCode(String lang, String code);

    List<Locale> findByLang(String lang);

    @Query("SELECT DISTINCT p.lang FROM Locale p")
    List<String> findDistinctLanguages();

}
