//
// Log.h
//
// This file has been generated from ../db/mappers/auth.xml. Do not edit.
//


#ifndef palm_models_Log_INCLUDED
#define palm_models_Log_INCLUDED


#include "Poco/ActiveRecord/ActiveRecord.h"


namespace palm {
namespace models {


class Log: public Poco::ActiveRecord::ActiveRecord<Poco::Int64>
{
public:
	using Ptr = Poco::AutoPtr<Log>;

	explicit Log(ID id);
	Log() = default;
	Log(const Log& other);
	~Log() = default;

	const Poco::DateTime& created_at() const;
	Log& created_at(const Poco::DateTime& value);

	static Ptr find(Poco::ActiveRecord::Context::Ptr pContext, const ID& id);

	void insert();
	void update();
	void remove();

	static const std::vector<std::string>& columns();
	static const std::string& table();

private:
	Poco::DateTime _created_at;

	friend class Poco::Data::TypeHandler<Log>;
};


inline const Poco::DateTime& Log::created_at() const
{
	return _created_at;
}


inline Log& Log::created_at(const Poco::DateTime& value)
{
	_created_at = value;
	return *this;
}


} } // namespace palm::models


namespace Poco {
namespace Data {


template <>
class TypeHandler<palm::models::Log>
{
public:
	static std::size_t size()
	{
		return 1;
	}

	static void bind(std::size_t pos, const palm::models::Log& ar, AbstractBinder::Ptr pBinder, AbstractBinder::Direction dir)
	{
		TypeHandler<Poco::DateTime>::bind(pos++, ar._created_at, pBinder, dir);
}

	static void extract(std::size_t pos, palm::models::Log& ar, const palm::models::Log& deflt, AbstractExtractor::Ptr pExtr)
	{
		TypeHandler<Poco::DateTime>::extract(pos++, ar._created_at, deflt._created_at, pExtr);
}

	static void prepare(std::size_t pos, const palm::models::Log& ar, AbstractPreparator::Ptr pPrep)
	{
		TypeHandler<Poco::DateTime>::prepare(pos++, ar._created_at, pPrep);
	}
};


} } // namespace Poco::Data


#endif // palm_models_Log_INCLUDED
