
#ifndef DISCORD_BOT_H_
#define DISCORD_BOT_H_

typedef struct bot bot_t;

bot_t *bot_create (char *token);

void bot_enter_context (bot_t *B);


#endif // DISCORD_BOT_H_

