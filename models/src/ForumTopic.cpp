//
// ForumTopic.cpp
//
// This file has been generated from ../db/mappers/forum.xml. Do not edit.
//


#include "palm/models/ForumTopic.h"


using namespace std::string_literals;
using namespace Poco::Data::Keywords;


namespace palm {
namespace models {


ForumTopic::ForumTopic(ID id):
	Poco::ActiveRecord::ActiveRecord<Poco::Int64>(id)
{
}


ForumTopic::ForumTopic(const ForumTopic& other):
	Poco::ActiveRecord::ActiveRecord<Poco::Int64>(other),
	_version(other._version),
	_created_at(other._created_at),
	_updated_at(other._updated_at)
{
}


ForumTopic::Ptr ForumTopic::find(Poco::ActiveRecord::Context::Ptr pContext, const ID& id)
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(pContext->statementPlaceholderProvider());
	ForumTopic::Ptr pObject(new ForumTopic);

	pContext->session()
		<< "SELECT id, version, created_at, updated_at"
		<< "  FROM forum_topics"
		<< "  WHERE id = " << pSPP->next(),
		into(pObject->mutableID()),
		into(*pObject),
		bind(id),
		now;

	return withContext(pObject, pContext);
}


void ForumTopic::insert()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "INSERT INTO forum_topics (id, version, created_at, updated_at)"
		<< "  VALUES (NULL, " << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ")",
		use(*this),
		now;
	updateID(context()->session());
}


void ForumTopic::update()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "UPDATE forum_topics"
		<< "  SET version = " << pSPP->next() << ", created_at = " << pSPP->next() << ", updated_at = " << pSPP->next()
		<< "  WHERE id = " << pSPP->next(),
		use(*this),
		bind(id()),
		now;
}


void ForumTopic::remove()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "DELETE FROM forum_topics"
		<< "  WHERE id = " << pSPP->next(),
		bind(id()),
		now;
}


const std::vector<std::string>& ForumTopic::columns()
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


const std::string& ForumTopic::table()
{
	static const std::string t = "forum_topics";
	return t;
}


} } // namespace palm::models
