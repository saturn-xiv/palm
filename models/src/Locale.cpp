//
// Locale.cpp
//
// This file has been generated from ../db/mappers/locales.xml. Do not edit.
//


#include "palm/models/Locale.h"


using namespace std::string_literals;
using namespace Poco::Data::Keywords;


namespace palm {
namespace models {


Locale::Locale(ID id):
	Poco::ActiveRecord::ActiveRecord<Poco::Int64>(id)
{
}


Locale::Locale(const Locale& other):
	Poco::ActiveRecord::ActiveRecord<Poco::Int64>(other),
	_code(other._code),
	_lang(other._lang),
	_message(other._message),
	_version(other._version),
	_created_at(other._created_at),
	_updated_at(other._updated_at)
{
}


Locale::Ptr Locale::find(Poco::ActiveRecord::Context::Ptr pContext, const ID& id)
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(pContext->statementPlaceholderProvider());
	Locale::Ptr pObject(new Locale);

	pContext->session()
		<< "SELECT id, code, lang, message, version, created_at, updated_at"
		<< "  FROM locales"
		<< "  WHERE id = " << pSPP->next(),
		into(pObject->mutableID()),
		into(*pObject),
		bind(id),
		now;

	return withContext(pObject, pContext);
}


void Locale::insert()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "INSERT INTO locales (id, code, lang, message, version, created_at, updated_at)"
		<< "  VALUES (NULL, " << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ")",
		use(*this),
		now;
	updateID(context()->session());
}


void Locale::update()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "UPDATE locales"
		<< "  SET code = " << pSPP->next() << ", lang = " << pSPP->next() << ", message = " << pSPP->next() << ", version = " << pSPP->next() << ", created_at = " << pSPP->next() << ", updated_at = " << pSPP->next()
		<< "  WHERE id = " << pSPP->next(),
		use(*this),
		bind(id()),
		now;
}


void Locale::remove()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "DELETE FROM locales"
		<< "  WHERE id = " << pSPP->next(),
		bind(id()),
		now;
}


const std::vector<std::string>& Locale::columns()
{
	static const std::vector<std::string> cols =
	{
		"id"s,
		"code"s,
		"lang"s,
		"message"s,
		"version"s,
		"created_at"s,
		"updated_at"s,
	};

	return cols;
}


const std::string& Locale::table()
{
	static const std::string t = "locales";
	return t;
}


} } // namespace palm::models
