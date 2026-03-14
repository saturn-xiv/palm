#include "lavender/application.hpp"
#include "lavender/logging.hpp"
#include "lavender/open-search.hpp"
#include "lavender/version.hpp"

#include <cstdlib>
#include <iostream>

#include <boost/log/core.hpp>
#include <boost/log/expressions.hpp>
#include <boost/log/trivial.hpp>
#include <boost/program_options.hpp>
#include <boost/property_tree/ini_parser.hpp>
#include <boost/property_tree/ptree.hpp>

int lavender::Application::launch(int argc, char** argv) const {
  std::vector<std::string> folders;
  std::string config_file;

  boost::program_options::options_description desc("Allowed options");
  desc.add_options()("help,h", "produce help message")(
      "version,v", "print version")("debug,d", "run on debug mode")(
      "config,c",
      boost::program_options::value<std::string>(&config_file)
          ->default_value("config.ini"),
      "load configuration from(default: config.ini)")(
      "folders,f",
      boost::program_options::value<std::vector<std::string>>(&folders)
          ->multitoken()
          ->composing(),
      "folders to watching");

  boost::program_options::variables_map vm;
  boost::program_options::store(
      boost::program_options::parse_command_line(argc, argv, desc), vm);
  boost::program_options::notify(vm);

  if (vm.count("help")) {
    std::cout << desc << std::endl;
    return EXIT_SUCCESS;
  }
  if (vm.count("version")) {
    std::cout << lavender::VERSION << std::endl;
    return EXIT_SUCCESS;
  }

  boost::log::core::get()->set_filter(
      boost::log::trivial::severity >=
      (vm.count("debug") ? boost::log::trivial::severity_level::debug
                         : boost::log::trivial::severity_level::debug));
  BOOST_LOG_TRIVIAL(debug) << "runing on debug mode";

  BOOST_LOG_TRIVIAL(debug) << "load configuration from file " << config_file;

  boost::property_tree::ptree tree;
  boost::property_tree::ini_parser::read_ini(config_file, tree);
  std::string opensearch_url = tree.get<std::string>("opensearch.url");
  std::string opensearch_namespace =
      tree.get<std::string>("opensearch.namespace");
  auto search = std::make_shared<lavender::OpenSearch>(opensearch_url,
                                                       opensearch_namespace);
  if (!search->index_exists<lavender::logging::filesystem::Message>()) {
    search->create_index<lavender::logging::filesystem::Message>(
        R"(
{
  "host": {
    "type": "text"
  },
  "file": {
    "type": "text"
  },
  "line": {
    "type": "text"
  },
  "created_at": {
    "type": "date_nanos",
    "format": "strict_date_optional_time_nanos"
  }
}
)"_json);
  }

  if (folders.empty()) {
    BOOST_LOG_TRIVIAL(warning) << "watching folders are empty";
  }

  return EXIT_SUCCESS;
}
