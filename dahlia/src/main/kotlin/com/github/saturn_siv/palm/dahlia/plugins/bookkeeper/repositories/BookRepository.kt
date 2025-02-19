package com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.repositories

import com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.models.Book
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("dahlia.bookkeeper.r.book.jpa")
interface BookRepository : CrudRepository<Book, Int> {
}