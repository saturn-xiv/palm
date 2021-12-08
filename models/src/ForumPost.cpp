//
// ForumPost.cpp
//
// This file has been generated from ../db/mappers/forum.xml. Do not edit.
//


#include "palm/models/ForumPost.h"


using namespace std::string_literals;
using namespace Poco::Data::Keywords;


namespace palm {
namespace models {


ForumPost::ForumPost(ID id):
	Poco::ActiveRecord::ActiveRecord<Poco::Int64>(id)
{
}


ForumPost::ForumPost(const ForumPost& other):
	Poco::ActiveRecord::ActiveRecord<Poco::Int64>(other),
	_version(other._version),
	_created_at(other._created_at),
	_updated_at(other._updated_at)
{
}


ForumPost::Ptr ForumPost::find(Poco::ActiveRecord::Context::Ptr pContext, const ID& id)
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(pContext->statementPlaceholderProvider());
	ForumPost::Ptr pObject(new ForumPost);

	pContext->session()
		<< "SELECT id, version, created_at, updated_at"
		<< "  FROM forum_posts"
		<< "  WHERE id = " << pSPP->next(),
		into(pObject->mutableID()),
		into(*pObject),
		bind(id),
		now;

	return withContext(pObject, pContext);
}


void ForumPost::insert()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "INSERT INTO forum_posts (id, version, created_at, updated_at)"
		<< "  VALUES (NULL, " << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ")",
		use(*this),
		now;
	updateID(context()->session());
}


void ForumPost::update()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "UPDATE forum_posts"
		<< "  SET version = " << pSPP->next() << ", created_at = " << pSPP->next() << ", updated_at = " << pSPP->next()
		<< "  WHERE id = " << pSPP->next(),
		use(*this),
		bind(id()),
		now;
}


void ForumPost::remove()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "DELETE FROM forum_posts"
		<< "  WHERE id = " << pSPP->next(),
		bind(id()),
		now;
}


const std::vector<std::string>& ForumPost::columns()
{
	static const std::vector<std::string> cols =
	{
		"id"s,
		"version"s,
		"created_at"s,
		"updated_at"s,
	};

	return cols;
}


const std::string& ForumPost::table()
{
	static const std::string t = "forum_posts";
	return t;
}


} } // namespace palm::models
