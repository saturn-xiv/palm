//
// ForumTopicTag.h
//
// This file has been generated from ../db/mappers/forum.xml. Do not edit.
//


#ifndef palm_models_ForumTopicTag_INCLUDED
#define palm_models_ForumTopicTag_INCLUDED


#include "Poco/ActiveRecord/ActiveRecord.h"


namespace palm {
namespace models {


class ForumTopicTag: public Poco::ActiveRecord::ActiveRecord<Poco::Int64>
{
public:
	using Ptr = Poco::AutoPtr<ForumTopicTag>;

	explicit ForumTopicTag(ID id);
	ForumTopicTag() = default;
	ForumTopicTag(const ForumTopicTag& other);
	~ForumTopicTag() = default;

	Poco::Int64 version() const;
	ForumTopicTag& version(Poco::Int64 value);

	const Poco::DateTime& created_at() const;
	ForumTopicTag& created_at(const Poco::DateTime& value);

	const Poco::DateTime& updated_at() const;
	ForumTopicTag& updated_at(const Poco::DateTime& value);

	static Ptr find(Poco::ActiveRecord::Context::Ptr pContext, const ID& id);

	void insert();
	void update();
	void remove();

	static const std::vector<std::string>& columns();
	static const std::string& table();

private:
	Poco::Int64 _version = 0;
	Poco::DateTime _created_at;
	Poco::DateTime _updated_at;

	friend class Poco::Data::TypeHandler<ForumTopicTag>;
};


inline Poco::Int64 ForumTopicTag::version() const
{
	return _version;
}


inline ForumTopicTag& ForumTopicTag::version(Poco::Int64 value)
{
	_version = value;
	return *this;
}


inline const Poco::DateTime& ForumTopicTag::created_at() const
{
	return _created_at;
}


inline ForumTopicTag& ForumTopicTag::created_at(const Poco::DateTime& value)
{
	_created_at = value;
	return *this;
}


inline const Poco::DateTime& ForumTopicTag::updated_at() const
{
	return _updated_at;
}


inline ForumTopicTag& ForumTopicTag::updated_at(const Poco::DateTime& value)
{
	_updated_at = value;
	return *this;
}


} } // namespace palm::models


namespace Poco {
namespace Data {


template <>
class TypeHandler<palm::models::ForumTopicTag>
{
public:
	static std::size_t size()
	{
		return 3;
	}

	static void bind(std::size_t pos, const palm::models::ForumTopicTag& ar, AbstractBinder::Ptr pBinder, AbstractBinder::Direction dir)
	{
		TypeHandler<Poco::Int64>::bind(pos++, ar._version, pBinder, dir);
		TypeHandler<Poco::DateTime>::bind(pos++, ar._created_at, pBinder, dir);
		TypeHandler<Poco::DateTime>::bind(pos++, ar._updated_at, pBinder, dir);
}

	static void extract(std::size_t pos, palm::models::ForumTopicTag& ar, const palm::models::ForumTopicTag& deflt, AbstractExtractor::Ptr pExtr)
	{
		TypeHandler<Poco::Int64>::extract(pos++, ar._version, deflt._version, pExtr);
		TypeHandler<Poco::DateTime>::extract(pos++, ar._created_at, deflt._created_at, pExtr);
		TypeHandler<Poco::DateTime>::extract(pos++, ar._updated_at, deflt._updated_at, pExtr);
}

	static void prepare(std::size_t pos, const palm::models::ForumTopicTag& ar, AbstractPreparator::Ptr pPrep)
	{
		TypeHandler<Poco::Int64>::prepare(pos++, ar._version, pPrep);
		TypeHandler<Poco::DateTime>::prepare(pos++, ar._created_at, pPrep);
		TypeHandler<Poco::DateTime>::prepare(pos++, ar._updated_at, pPrep);
	}
};


} } // namespace Poco::Data


#endif // palm_models_ForumTopicTag_INCLUDED
