//
// Migration.h
//
// This file has been generated from ../db/mappers/schema_migration.xml. Do not edit.
//


#ifndef palm_models_Migration_INCLUDED
#define palm_models_Migration_INCLUDED


#include "Poco/ActiveRecord/ActiveRecord.h"


namespace palm {
namespace models {


class Migration: public Poco::ActiveRecord::KeylessActiveRecord
{
public:
	using Ptr = Poco::AutoPtr<Migration>;

	Migration() = default;
	Migration(const Migration& other);
	~Migration() = default;

	const std::string& version() const;
	Migration& version(const std::string& value);

	const std::string& name() const;
	Migration& name(const std::string& value);

	const std::string& up() const;
	Migration& up(const std::string& value);

	const std::string& down() const;
	Migration& down(const std::string& value);

	const Poco::Nullable<Poco::DateTime>& run_on() const;
	Migration& run_on(const Poco::Nullable<Poco::DateTime>& value);

	const Poco::DateTime& created_at() const;
	Migration& created_at(const Poco::DateTime& value);

	void insert();
	void update();
	void remove();

	static const std::vector<std::string>& columns();
	static const std::string& table();

private:
	std::string _version;
	std::string _name;
	std::string _up;
	std::string _down;
	Poco::Nullable<Poco::DateTime> _run_on;
	Poco::DateTime _created_at;

	friend class Poco::Data::TypeHandler<Migration>;
};


inline const std::string& Migration::version() const
{
	return _version;
}


inline Migration& Migration::version(const std::string& value)
{
	_version = value;
	return *this;
}


inline const std::string& Migration::name() const
{
	return _name;
}


inline Migration& Migration::name(const std::string& value)
{
	_name = value;
	return *this;
}


inline const std::string& Migration::up() const
{
	return _up;
}


inline Migration& Migration::up(const std::string& value)
{
	_up = value;
	return *this;
}


inline const std::string& Migration::down() const
{
	return _down;
}


inline Migration& Migration::down(const std::string& value)
{
	_down = value;
	return *this;
}


inline const Poco::Nullable<Poco::DateTime>& Migration::run_on() const
{
	return _run_on;
}


inline Migration& Migration::run_on(const Poco::Nullable<Poco::DateTime>& value)
{
	_run_on = value;
	return *this;
}


inline const Poco::DateTime& Migration::created_at() const
{
	return _created_at;
}


inline Migration& Migration::created_at(const Poco::DateTime& value)
{
	_created_at = value;
	return *this;
}


} } // namespace palm::models


namespace Poco {
namespace Data {


template <>
class TypeHandler<palm::models::Migration>
{
public:
	static std::size_t size()
	{
		return 6;
	}

	static void bind(std::size_t pos, const palm::models::Migration& ar, AbstractBinder::Ptr pBinder, AbstractBinder::Direction dir)
	{
		TypeHandler<std::string>::bind(pos++, ar._version, pBinder, dir);
		TypeHandler<std::string>::bind(pos++, ar._name, pBinder, dir);
		TypeHandler<std::string>::bind(pos++, ar._up, pBinder, dir);
		TypeHandler<std::string>::bind(pos++, ar._down, pBinder, dir);
		TypeHandler<Poco::Nullable<Poco::DateTime>>::bind(pos++, ar._run_on, pBinder, dir);
		TypeHandler<Poco::DateTime>::bind(pos++, ar._created_at, pBinder, dir);
}

	static void extract(std::size_t pos, palm::models::Migration& ar, const palm::models::Migration& deflt, AbstractExtractor::Ptr pExtr)
	{
		TypeHandler<std::string>::extract(pos++, ar._version, deflt._version, pExtr);
		TypeHandler<std::string>::extract(pos++, ar._name, deflt._name, pExtr);
		TypeHandler<std::string>::extract(pos++, ar._up, deflt._up, pExtr);
		TypeHandler<std::string>::extract(pos++, ar._down, deflt._down, pExtr);
		TypeHandler<Poco::Nullable<Poco::DateTime>>::extract(pos++, ar._run_on, deflt._run_on, pExtr);
		TypeHandler<Poco::DateTime>::extract(pos++, ar._created_at, deflt._created_at, pExtr);
}

	static void prepare(std::size_t pos, const palm::models::Migration& ar, AbstractPreparator::Ptr pPrep)
	{
		TypeHandler<std::string>::prepare(pos++, ar._version, pPrep);
		TypeHandler<std::string>::prepare(pos++, ar._name, pPrep);
		TypeHandler<std::string>::prepare(pos++, ar._up, pPrep);
		TypeHandler<std::string>::prepare(pos++, ar._down, pPrep);
		TypeHandler<Poco::Nullable<Poco::DateTime>>::prepare(pos++, ar._run_on, pPrep);
		TypeHandler<Poco::DateTime>::prepare(pos++, ar._created_at, pPrep);
	}
};


} } // namespace Poco::Data


#endif // palm_models_Migration_INCLUDED
