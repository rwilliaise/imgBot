
#include <stdlib.h>
#include <string.h>

#include "image.h"
#include "state.h"

// seems like the only ones that gets displayed by Discord(?)
#define STBI_ONLY_JPEG
#define STBI_ONLY_PNG // TODO(rwilliaise): APNG support (low priority)
#define STBI_ONLY_GIF

#define STB_IMAGE_IMPLEMENTATION
#include "stb_image.h"

static animated_image_t *image_stbi_load_gif (stbi__context *s) {

}

image_t *image_from_data (int width, int height, char *data) {
	image_t *img = malloc(sizeof(image_t) + width * height - 1);

	if (img == NULL) { return NULL; }

	img->width = width;
	img->height = height;
	img->format = IMAGE_FORMAT_STILL;
	memcpy(img->data, data, width * height);

	return img;
}

any_image_t *image_from_url (state_t *S, const char *url) {
	size_t size;
	int x, y, channels;
	uint8_t *data = state_geturl_async(S, url, &size);

	stbi__context s;
	stbi__start_mem(&s, data, size);

	if (stbi__gif_test(&s)) {
		
	} else {

	}

	return NULL;
}

