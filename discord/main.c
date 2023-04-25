
#include <concord/discord.h>
#include <stdlib.h>

int main(void) {
	const char *bot_token = getenv("DISCORD_BOT_TOKEN");

	if (bot_token == NULL) {
		return 1;
	}

	struct discord *client = discord_init(bot_token);
	discord_run(client);

	return 0;
}
