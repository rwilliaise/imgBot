
#include <concord/discord.h>
#include <concord/discord_codecs.h>
#include <stdlib.h>

void bot_on_ready(struct discord *client, const struct discord_ready *event) {
    struct discord_create_global_application_command params = {
        .name = "caption",
        .description = "put text on an image, ifunny GIF caption/esmBot style",
    };
    discord_create_global_application_command(client, event->application->id, &params, NULL);
}

int main(void) {
	const char *bot_token = getenv("DISCORD_BOT_TOKEN");

	if (bot_token == NULL) {
		return 1;
	}

	struct discord *client = discord_init(bot_token);
    
    discord_set_on_ready(client, bot_on_ready);
	discord_run(client);

	return 0;
}
