//
// User.h
//
// This file has been generated from ../db/mappers/auth.xml. Do not edit.
//


#ifndef palm_models_User_INCLUDED
#define palm_models_User_INCLUDED


#include "Poco/ActiveRecord/ActiveRecord.h"


namespace palm {
namespace models {


class User: public Poco::ActiveRecord::ActiveRecord<Poco::Int64>
{
public:
	using Ptr = Poco::AutoPtr<User>;

	explicit User(ID id);
	User() = default;
	User(const User& other);
	~User() = default;

	Poco::Int64 user_id() const;
	User& user_id(Poco::Int64 value);

	const std::string& key() const;
	User& key(const std::string& value);

	const std::string& value() const;
	User& value(const std::string& value);

	const std::string& salt() const;
	User& salt(const std::string& value);

	Poco::Int64 version() const;
	User& version(Poco::Int64 value);

	const Poco::DateTime& created_at() const;
	User& created_at(const Poco::DateTime& value);

	const Poco::DateTime& updated_at() const;
	User& updated_at(const Poco::DateTime& value);

	static Ptr find(Poco::ActiveRecord::Context::Ptr pContext, const ID& id);

	void insert();
	void update();
	void remove();

	static const std::vector<std::string>& columns();
	static const std::string& table();

private:
	Poco::Int64 _user_id = 0;
	std::string _key;
	std::string _value;
	std::string _salt;
	Poco::Int64 _version = 0;
	Poco::DateTime _created_at;
	Poco::DateTime _updated_at;

	friend class Poco::Data::TypeHandler<User>;
};


inline Poco::Int64 User::user_id() const
{
	return _user_id;
}


inline User& User::user_id(Poco::Int64 value)
{
	_user_id = value;
	return *this;
}


inline const std::string& User::key() const
{
	return _key;
}


inline User& User::key(const std::string& value)
{
	_key = value;
	return *this;
}


inline const std::string& User::value() const
{
	return _value;
}


inline User& User::value(const std::string& value)
{
	_value = value;
	return *this;
}


inline const std::string& User::salt() const
{
	return _salt;
}


inline User& User::salt(const std::string& value)
{
	_salt = value;
	return *this;
}


inline Poco::Int64 User::version() const
{
	return _version;
}


inline User& User::version(Poco::Int64 value)
{
	_version = value;
	return *this;
}


inline const Poco::DateTime& User::created_at() const
{
	return _created_at;
}


inline User& User::created_at(const Poco::DateTime& value)
{
	_created_at = value;
	return *this;
}


inline const Poco::DateTime& User::updated_at() const
{
	return _updated_at;
}


inline User& User::updated_at(const Poco::DateTime& value)
{
	_updated_at = value;
	return *this;
}


} } // namespace palm::models


namespace Poco {
namespace Data {


template <>
class TypeHandler<palm::models::User>
{
public:
	static std::size_t size()
	{
		return 7;
	}

	static void bind(std::size_t pos, const palm::models::User& ar, AbstractBinder::Ptr pBinder, AbstractBinder::Direction dir)
	{
		TypeHandler<Poco::Int64>::bind(pos++, ar._user_id, pBinder, dir);
		TypeHandler<std::string>::bind(pos++, ar._key, pBinder, dir);
		TypeHandler<std::string>::bind(pos++, ar._value, pBinder, dir);
		TypeHandler<std::string>::bind(pos++, ar._salt, pBinder, dir);
		TypeHandler<Poco::Int64>::bind(pos++, ar._version, pBinder, dir);
		TypeHandler<Poco::DateTime>::bind(pos++, ar._created_at, pBinder, dir);
		TypeHandler<Poco::DateTime>::bind(pos++, ar._updated_at, pBinder, dir);
}

	static void extract(std::size_t pos, palm::models::User& ar, const palm::models::User& deflt, AbstractExtractor::Ptr pExtr)
	{
		TypeHandler<Poco::Int64>::extract(pos++, ar._user_id, deflt._user_id, pExtr);
		TypeHandler<std::string>::extract(pos++, ar._key, deflt._key, pExtr);
		TypeHandler<std::string>::extract(pos++, ar._value, deflt._value, pExtr);
		TypeHandler<std::string>::extract(pos++, ar._salt, deflt._salt, pExtr);
		TypeHandler<Poco::Int64>::extract(pos++, ar._version, deflt._version, pExtr);
		TypeHandler<Poco::DateTime>::extract(pos++, ar._created_at, deflt._created_at, pExtr);
		TypeHandler<Poco::DateTime>::extract(pos++, ar._updated_at, deflt._updated_at, pExtr);
}

	static void prepare(std::size_t pos, const palm::models::User& ar, AbstractPreparator::Ptr pPrep)
	{
		TypeHandler<Poco::Int64>::prepare(pos++, ar._user_id, pPrep);
		TypeHandler<std::string>::prepare(pos++, ar._key, pPrep);
		TypeHandler<std::string>::prepare(pos++, ar._value, pPrep);
		TypeHandler<std::string>::prepare(pos++, ar._salt, pPrep);
		TypeHandler<Poco::Int64>::prepare(pos++, ar._version, pPrep);
		TypeHandler<Poco::DateTime>::prepare(pos++, ar._created_at, pPrep);
		TypeHandler<Poco::DateTime>::prepare(pos++, ar._updated_at, pPrep);
	}
};


} } // namespace Poco::Data


#endif // palm_models_User_INCLUDED
