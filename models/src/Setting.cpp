//
// Setting.cpp
//
// This file has been generated from ../db/mappers/settings.xml. Do not edit.
//


#include "palm/models/Setting.h"


using namespace std::string_literals;
using namespace Poco::Data::Keywords;


namespace palm {
namespace models {


Setting::Setting(ID id):
	Poco::ActiveRecord::ActiveRecord<Poco::Int64>(id)
{
}


Setting::Setting(const Setting& other):
	Poco::ActiveRecord::ActiveRecord<Poco::Int64>(other),
	_user_id(other._user_id),
	_key(other._key),
	_value(other._value),
	_salt(other._salt),
	_version(other._version),
	_created_at(other._created_at),
	_updated_at(other._updated_at)
{
}


Setting::Ptr Setting::find(Poco::ActiveRecord::Context::Ptr pContext, const ID& id)
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(pContext->statementPlaceholderProvider());
	Setting::Ptr pObject(new Setting);

	pContext->session()
		<< "SELECT id, user_id, key, value, salt, version, created_at, updated_at"
		<< "  FROM settings"
		<< "  WHERE id = " << pSPP->next(),
		into(pObject->mutableID()),
		into(*pObject),
		bind(id),
		now;

	return withContext(pObject, pContext);
}


void Setting::insert()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "INSERT INTO settings (id, user_id, key, value, salt, version, created_at, updated_at)"
		<< "  VALUES (NULL, " << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ")",
		use(*this),
		now;
	updateID(context()->session());
}


void Setting::update()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "UPDATE settings"
		<< "  SET user_id = " << pSPP->next() << ", key = " << pSPP->next() << ", value = " << pSPP->next() << ", salt = " << pSPP->next() << ", version = " << pSPP->next() << ", created_at = " << pSPP->next() << ", updated_at = " << pSPP->next()
		<< "  WHERE id = " << pSPP->next(),
		use(*this),
		bind(id()),
		now;
}


void Setting::remove()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "DELETE FROM settings"
		<< "  WHERE id = " << pSPP->next(),
		bind(id()),
		now;
}


const std::vector<std::string>& Setting::columns()
{
	static const std::vector<std::string> cols =
	{
		"id"s,
		"user_id"s,
		"key"s,
		"value"s,
		"salt"s,
		"version"s,
		"created_at"s,
		"updated_at"s,
	};

	return cols;
}


const std::string& Setting::table()
{
	static const std::string t = "settings";
	return t;
}


} } // namespace palm::models
