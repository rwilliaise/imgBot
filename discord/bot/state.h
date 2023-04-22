
#ifndef DISCORD_BOT_STATE_H_
#define DISCORD_BOT_STATE_H_

#include "../bot.h"

#include <curl/curl.h>
#include <libwebsockets.h>

struct bot { // private definition
	CURL *curl_state;
};

#endif // DISCORD_BOT_STATE_H_
