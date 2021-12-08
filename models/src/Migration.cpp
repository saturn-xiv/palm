//
// Migration.cpp
//
// This file has been generated from ../db/mappers/schema_migration.xml. Do not edit.
//


#include "palm/models/Migration.h"


using namespace std::string_literals;
using namespace Poco::Data::Keywords;


namespace palm {
namespace models {


Migration::Migration(const Migration& other):
	_version(other._version),
	_name(other._name),
	_up(other._up),
	_down(other._down),
	_run_on(other._run_on),
	_created_at(other._created_at)
{
}


void Migration::insert()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "INSERT INTO schema_migrations (version, name, up, down, run_on, created_at)"
		<< "  VALUES (" << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ")",
		use(*this),
		now;
}


void Migration::update()
{
	throw Poco::NotImplementedException("update not implemented for keyless class", "Migration");
}


void Migration::remove()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "DELETE FROM schema_migrations"
		<< "  WHERE version = " << pSPP->next() << " AND name = " << pSPP->next() << " AND up = " << pSPP->next() << " AND down = " << pSPP->next() << " AND run_on = " << pSPP->next() << " AND created_at = " << pSPP->next(),
		use(*this),
		now;
}


const std::vector<std::string>& Migration::columns()
{
	static const std::vector<std::string> cols =
	{
		"version"s,
		"name"s,
		"up"s,
		"down"s,
		"run_on"s,
		"created_at"s,
	};

	return cols;
}


const std::string& Migration::table()
{
	static const std::string t = "schema_migrations";
	return t;
}


} } // namespace palm::models
