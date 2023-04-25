
#ifndef SERVER_STATE_H_
#define SERVER_STATE_H_

#include <stddef.h>

typedef struct state state_t;

state_t *state_new ();
void state_free (state_t *S);

char *state_geturl_async (state_t *S, const char *url, size_t *size);

#endif // SERVER_STATE_H_
