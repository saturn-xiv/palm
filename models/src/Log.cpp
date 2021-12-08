//
// Log.cpp
//
// This file has been generated from ../db/mappers/auth.xml. Do not edit.
//


#include "palm/models/Log.h"


using namespace std::string_literals;
using namespace Poco::Data::Keywords;


namespace palm {
namespace models {


Log::Log(ID id):
	Poco::ActiveRecord::ActiveRecord<Poco::Int64>(id)
{
}


Log::Log(const Log& other):
	Poco::ActiveRecord::ActiveRecord<Poco::Int64>(other),
	_created_at(other._created_at)
{
}


Log::Ptr Log::find(Poco::ActiveRecord::Context::Ptr pContext, const ID& id)
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(pContext->statementPlaceholderProvider());
	Log::Ptr pObject(new Log);

	pContext->session()
		<< "SELECT id, created_at"
		<< "  FROM logs"
		<< "  WHERE id = " << pSPP->next(),
		into(pObject->mutableID()),
		into(*pObject),
		bind(id),
		now;

	return withContext(pObject, pContext);
}


void Log::insert()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "INSERT INTO logs (id, created_at)"
		<< "  VALUES (NULL, " << pSPP->next() << ")",
		use(*this),
		now;
	updateID(context()->session());
}


void Log::update()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "UPDATE logs"
		<< "  SET created_at = " << pSPP->next()
		<< "  WHERE id = " << pSPP->next(),
		use(*this),
		bind(id()),
		now;
}


void Log::remove()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "DELETE FROM logs"
		<< "  WHERE id = " << pSPP->next(),
		bind(id()),
		now;
}


const std::vector<std::string>& Log::columns()
{
	static const std::vector<std::string> cols =
	{
		"id"s,
		"created_at"s,
	};

	return cols;
}


const std::string& Log::table()
{
	static const std::string t = "logs";
	return t;
}


} } // namespace palm::models
