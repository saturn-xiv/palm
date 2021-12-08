//
// OpsVpnLog.cpp
//
// This file has been generated from ../db/mappers/ops-vpn.xml. Do not edit.
//


#include "palm/models/OpsVpnLog.h"


using namespace std::string_literals;
using namespace Poco::Data::Keywords;


namespace palm {
namespace models {


OpsVpnLog::OpsVpnLog(ID id):
	Poco::ActiveRecord::ActiveRecord<Poco::Int64>(id)
{
}


OpsVpnLog::OpsVpnLog(const OpsVpnLog& other):
	Poco::ActiveRecord::ActiveRecord<Poco::Int64>(other),
	_version(other._version),
	_created_at(other._created_at),
	_updated_at(other._updated_at)
{
}


OpsVpnLog::Ptr OpsVpnLog::find(Poco::ActiveRecord::Context::Ptr pContext, const ID& id)
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(pContext->statementPlaceholderProvider());
	OpsVpnLog::Ptr pObject(new OpsVpnLog);

	pContext->session()
		<< "SELECT id, version, created_at, updated_at"
		<< "  FROM ops_vpn_logs"
		<< "  WHERE id = " << pSPP->next(),
		into(pObject->mutableID()),
		into(*pObject),
		bind(id),
		now;

	return withContext(pObject, pContext);
}


void OpsVpnLog::insert()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "INSERT INTO ops_vpn_logs (id, version, created_at, updated_at)"
		<< "  VALUES (NULL, " << pSPP->next() << ", " << pSPP->next() << ", " << pSPP->next() << ")",
		use(*this),
		now;
	updateID(context()->session());
}


void OpsVpnLog::update()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "UPDATE ops_vpn_logs"
		<< "  SET version = " << pSPP->next() << ", created_at = " << pSPP->next() << ", updated_at = " << pSPP->next()
		<< "  WHERE id = " << pSPP->next(),
		use(*this),
		bind(id()),
		now;
}


void OpsVpnLog::remove()
{
	Poco::ActiveRecord::StatementPlaceholderProvider::Ptr pSPP(context()->statementPlaceholderProvider());

	context()->session()
		<< "DELETE FROM ops_vpn_logs"
		<< "  WHERE id = " << pSPP->next(),
		bind(id()),
		now;
}


const std::vector<std::string>& OpsVpnLog::columns()
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


const std::string& OpsVpnLog::table()
{
	static const std::string t = "ops_vpn_logs";
	return t;
}


} } // namespace palm::models
