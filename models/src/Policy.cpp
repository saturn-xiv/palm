//
// Policy.cpp
//
// This file has been generated from ../db/mappers/rbac.xml. Do not edit.
//


#include "palm/models/Policy.h"


using namespace std::string_literals;
using namespace Poco::Data::Keywords;


namespace palm {
namespace models {


Policy::Policy(ID id):
	Poco::ActiveRecord::ActiveRecord<Poco::Int64>(id)
{
}


Policy::Policy(const Policy& other):
	Poco::ActiveRecord::ActiveRecord<Poco::Int64>(other),
	_version(other._version),
	_created_at(other._created_at),
	_updated_at(other._updated_at)
{
}


Policy::Ptr Policy::find(Poco::ActiveRecord::Context::Ptr pContext, const ID& id)
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(pContext->statementPlaceholderProvider());
	Policy::Ptr pObject(new Policy);

	pContext->session()
		<< "SELECT id, version, created_at, updated_at"
		<< "  FROM polcies"
		<< "  WHERE id = " << pSPP->next(),
		into(pObject->mutableID()),
		into(*pObject),
		bind(id),
		now;

	return withContext(pObject, pContext);
}


void Policy::insert()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "INSERT INTO polcies (id, version, created_at, updated_at)"
		<< "  VALUES (NULL, " << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ")",
		use(*this),
		now;
	updateID(context()->session());
}


void Policy::update()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "UPDATE polcies"
		<< "  SET version = " << pSPP->next() << ", created_at = " << pSPP->next() << ", updated_at = " << pSPP->next()
		<< "  WHERE id = " << pSPP->next(),
		use(*this),
		bind(id()),
		now;
}


void Policy::remove()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "DELETE FROM polcies"
		<< "  WHERE id = " << pSPP->next(),
		bind(id()),
		now;
}


const std::vector<std::string>& Policy::columns()
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


const std::string& Policy::table()
{
	static const std::string t = "polcies";
	return t;
}


} } // namespace palm::models
