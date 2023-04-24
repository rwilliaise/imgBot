
#include <stdlib.h>
#include <string.h>

#include "image.h"
#include "state.h"

#define STB_IMAGE_IMPLEMENTATION
#include "stb_image.h"

image_t *image_from_data (int width, int height, char *data) {
	image_t *img = malloc(sizeof(image_t) + width * height - 1);

	if (img == NULL) { return NULL; }

	img->width = width;
	img->height = height;
	img->format = IMAGE_FORMAT_STILL;
	memcpy(img->data, data, width * height);

	return img;
}

any_image_t *image_from_url(state_t *state, const char *url) {


	return NULL;
}

