<?php

require dirname(__FILE__) . '/vendor/autoload.php';

function markdown2html($server, $payload, $sanitize)
{
    global $logger;

    $client = new Palm\Morus\V1\MarkdownClient($server, [
        'credentials' => Grpc\ChannelCredentials::createInsecure(),
    ]);
    $request = new Palm\Morus\V1\MarkdownToHtmlRequest();
    $request->setPayload($payload);
    $request->setSanitize($sanitize);
    list($response, $status) = $client->ToHtml($request)->wait();
    if ($status->code !== Grpc\STATUS_OK) {
        $logger->error("markdown to html", ['code' => $status->code, 'details' => $status->details]);
        exit(1);
    }
    $logger->info("markdown to html", ['body' => $response->getPayload()]);
}

$logger = new Monolog\Logger('palm');
$logger->pushHandler(new Monolog\Handler\StreamHandler('php://stdout', Monolog\Level::Debug));


if (empty($argv[1])) {
    $logger->error("USAGE: " . $argv[0] . " CONFIG_FILE");
    exit(1);
}

$logger->debug("load config from", ['file' => $argv[1]]);
$config = json_decode(file_get_contents($argv[1]));

$server = $config->{'server'}->{'host'} . ":" . $config->{'server'}->{'port'};
$logger->info("connect to", ['server' => $server]);

markdown2html($server, "# Hi, Palm!", true);
