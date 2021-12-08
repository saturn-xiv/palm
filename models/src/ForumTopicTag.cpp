//
// ForumTopicTag.cpp
//
// This file has been generated from ../db/mappers/forum.xml. Do not edit.
//


#include "palm/models/ForumTopicTag.h"


using namespace std::string_literals;
using namespace Poco::Data::Keywords;


namespace palm {
namespace models {


ForumTopicTag::ForumTopicTag(ID id):
	Poco::ActiveRecord::ActiveRecord<Poco::Int64>(id)
{
}


ForumTopicTag::ForumTopicTag(const ForumTopicTag& other):
	Poco::ActiveRecord::ActiveRecord<Poco::Int64>(other),
	_version(other._version),
	_created_at(other._created_at),
	_updated_at(other._updated_at)
{
}


ForumTopicTag::Ptr ForumTopicTag::find(Poco::ActiveRecord::Context::Ptr pContext, const ID& id)
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(pContext->statementPlaceholderProvider());
	ForumTopicTag::Ptr pObject(new ForumTopicTag);

	pContext->session()
		<< "SELECT id, version, created_at, updated_at"
		<< "  FROM forum_topics_tags"
		<< "  WHERE id = " << pSPP->next(),
		into(pObject->mutableID()),
		into(*pObject),
		bind(id),
		now;

	return withContext(pObject, pContext);
}


void ForumTopicTag::insert()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "INSERT INTO forum_topics_tags (id, version, created_at, updated_at)"
		<< "  VALUES (NULL, " << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ")",
		use(*this),
		now;
	updateID(context()->session());
}


void ForumTopicTag::update()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "UPDATE forum_topics_tags"
		<< "  SET version = " << pSPP->next() << ", created_at = " << pSPP->next() << ", updated_at = " << pSPP->next()
		<< "  WHERE id = " << pSPP->next(),
		use(*this),
		bind(id()),
		now;
}


void ForumTopicTag::remove()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "DELETE FROM forum_topics_tags"
		<< "  WHERE id = " << pSPP->next(),
		bind(id()),
		now;
}


const std::vector<std::string>& ForumTopicTag::columns()
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


const std::string& ForumTopicTag::table()
{
	static const std::string t = "forum_topics_tags";
	return t;
}


} } // namespace palm::models
