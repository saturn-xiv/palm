//
// User.cpp
//
// This file has been generated from ../db/mappers/auth.xml. Do not edit.
//


#include "palm/models/User.h"


using namespace std::string_literals;
using namespace Poco::Data::Keywords;


namespace palm {
namespace models {


User::User(ID id):
	Poco::ActiveRecord::ActiveRecord<Poco::Int64>(id)
{
}


User::User(const User& other):
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


User::Ptr User::find(Poco::ActiveRecord::Context::Ptr pContext, const ID& id)
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(pContext->statementPlaceholderProvider());
	User::Ptr pObject(new User);

	pContext->session()
		<< "SELECT id, user_id, key, value, salt, version, created_at, updated_at"
		<< "  FROM users"
		<< "  WHERE id = " << pSPP->next(),
		into(pObject->mutableID()),
		into(*pObject),
		bind(id),
		now;

	return withContext(pObject, pContext);
}


void User::insert()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "INSERT INTO users (id, user_id, key, value, salt, version, created_at, updated_at)"
		<< "  VALUES (NULL, " << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ")",
		use(*this),
		now;
	updateID(context()->session());
}


void User::update()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "UPDATE users"
		<< "  SET user_id = " << pSPP->next() << ", key = " << pSPP->next() << ", value = " << pSPP->next() << ", salt = " << pSPP->next() << ", version = " << pSPP->next() << ", created_at = " << pSPP->next() << ", updated_at = " << pSPP->next()
		<< "  WHERE id = " << pSPP->next(),
		use(*this),
		bind(id()),
		now;
}


void User::remove()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "DELETE FROM users"
		<< "  WHERE id = " << pSPP->next(),
		bind(id()),
		now;
}


const std::vector<std::string>& User::columns()
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


const std::string& User::table()
{
	static const std::string t = "users";
	return t;
}


} } // namespace palm::models
