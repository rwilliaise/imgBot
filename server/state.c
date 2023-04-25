
#include "state.h"

#include <curl/curl.h>
#include <stdlib.h>
#include <string.h>

struct state {
    CURL *curl;
};

struct memory {
    uint8_t *str;
    size_t size;
};

static size_t state_curl_writefunction (void *ptr, size_t size, size_t nmemb, struct memory *ud) {
    uint8_t *new_str = realloc(ud->str, ud->size += size * nmemb);
    if (new_str == NULL) {
        return 0;
    }

    memcpy(&new_str[size * nmemb], ptr, size);

    ud->str = new_str;
    return size * nmemb;
}

state_t *state_new () {
    state_t *S = malloc(sizeof(state_t));
    if (S == NULL) {
        return NULL;
    }

    S->curl = curl_easy_init();
    if (S->curl == NULL) {
        free(S);
        return NULL;
    }
    return S;
}

void state_free (state_t *S) {
   curl_easy_cleanup(S->curl); 
   free(S);
}

uint8_t *state_geturl_async(state_t *S, const char *url, size_t *size) {
    struct memory mem = { NULL, 0 };
    curl_easy_setopt(S->curl, CURLOPT_URL, url);
    curl_easy_setopt(S->curl, CURLOPT_MAXREDIRS, 10L);

    curl_easy_setopt(S->curl, CURLOPT_WRITEFUNCTION, state_curl_writefunction);
    curl_easy_setopt(S->curl, CURLOPT_WRITEDATA, &mem);

	if (size != NULL) {
		*size = mem.size;
	}

 	return mem.str;   
}
