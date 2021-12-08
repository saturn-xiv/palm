//
// Locale.h
//
// This file has been generated from ../db/mappers/locales.xml. Do not edit.
//


#ifndef palm_models_Locale_INCLUDED
#define palm_models_Locale_INCLUDED


#include "Poco/ActiveRecord/ActiveRecord.h"


namespace palm {
namespace models {


class Locale: public Poco::ActiveRecord::ActiveRecord<Poco::Int64>
{
public:
	using Ptr = Poco::AutoPtr<Locale>;

	explicit Locale(ID id);
	Locale() = default;
	Locale(const Locale& other);
	~Locale() = default;

	const std::string& code() const;
	Locale& code(const std::string& value);

	const std::string& lang() const;
	Locale& lang(const std::string& value);

	const std::string& message() const;
	Locale& message(const std::string& value);

	Poco::Int64 version() const;
	Locale& version(Poco::Int64 value);

	const Poco::DateTime& created_at() const;
	Locale& created_at(const Poco::DateTime& value);

	const Poco::DateTime& updated_at() const;
	Locale& updated_at(const Poco::DateTime& value);

	static Ptr find(Poco::ActiveRecord::Context::Ptr pContext, const ID& id);

	void insert();
	void update();
	void remove();

	static const std::vector<std::string>& columns();
	static const std::string& table();

private:
	std::string _code;
	std::string _lang;
	std::string _message;
	Poco::Int64 _version = 0;
	Poco::DateTime _created_at;
	Poco::DateTime _updated_at;

	friend class Poco::Data::TypeHandler<Locale>;
};


inline const std::string& Locale::code() const
{
	return _code;
}


inline Locale& Locale::code(const std::string& value)
{
	_code = value;
	return *this;
}


inline const std::string& Locale::lang() const
{
	return _lang;
}


inline Locale& Locale::lang(const std::string& value)
{
	_lang = value;
	return *this;
}


inline const std::string& Locale::message() const
{
	return _message;
}


inline Locale& Locale::message(const std::string& value)
{
	_message = value;
	return *this;
}


inline Poco::Int64 Locale::version() const
{
	return _version;
}


inline Locale& Locale::version(Poco::Int64 value)
{
	_version = value;
	return *this;
}


inline const Poco::DateTime& Locale::created_at() const
{
	return _created_at;
}


inline Locale& Locale::created_at(const Poco::DateTime& value)
{
	_created_at = value;
	return *this;
}


inline const Poco::DateTime& Locale::updated_at() const
{
	return _updated_at;
}


inline Locale& Locale::updated_at(const Poco::DateTime& value)
{
	_updated_at = value;
	return *this;
}


} } // namespace palm::models


namespace Poco {
namespace Data {


template <>
class TypeHandler<palm::models::Locale>
{
public:
	static std::size_t size()
	{
		return 6;
	}

	static void bind(std::size_t pos, const palm::models::Locale& ar, AbstractBinder::Ptr pBinder, AbstractBinder::Direction dir)
	{
		TypeHandler<std::string>::bind(pos++, ar._code, pBinder, dir);
		TypeHandler<std::string>::bind(pos++, ar._lang, pBinder, dir);
		TypeHandler<std::string>::bind(pos++, ar._message, pBinder, dir);
		TypeHandler<Poco::Int64>::bind(pos++, ar._version, pBinder, dir);
		TypeHandler<Poco::DateTime>::bind(pos++, ar._created_at, pBinder, dir);
		TypeHandler<Poco::DateTime>::bind(pos++, ar._updated_at, pBinder, dir);
}

	static void extract(std::size_t pos, palm::models::Locale& ar, const palm::models::Locale& deflt, AbstractExtractor::Ptr pExtr)
	{
		TypeHandler<std::string>::extract(pos++, ar._code, deflt._code, pExtr);
		TypeHandler<std::string>::extract(pos++, ar._lang, deflt._lang, pExtr);
		TypeHandler<std::string>::extract(pos++, ar._message, deflt._message, pExtr);
		TypeHandler<Poco::Int64>::extract(pos++, ar._version, deflt._version, pExtr);
		TypeHandler<Poco::DateTime>::extract(pos++, ar._created_at, deflt._created_at, pExtr);
		TypeHandler<Poco::DateTime>::extract(pos++, ar._updated_at, deflt._updated_at, pExtr);
}

	static void prepare(std::size_t pos, const palm::models::Locale& ar, AbstractPreparator::Ptr pPrep)
	{
		TypeHandler<std::string>::prepare(pos++, ar._code, pPrep);
		TypeHandler<std::string>::prepare(pos++, ar._lang, pPrep);
		TypeHandler<std::string>::prepare(pos++, ar._message, pPrep);
		TypeHandler<Poco::Int64>::prepare(pos++, ar._version, pPrep);
		TypeHandler<Poco::DateTime>::prepare(pos++, ar._created_at, pPrep);
		TypeHandler<Poco::DateTime>::prepare(pos++, ar._updated_at, pPrep);
	}
};


} } // namespace Poco::Data


#endif // palm_models_Locale_INCLUDED
