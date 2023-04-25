
#ifndef SERVER_IMAGE_H_
#define SERVER_IMAGE_H_

#include "state.h"

#define SERVER_IMAGE_IMAGE_HEADER \
	int width, height; \
	format_t format

typedef enum __attribute__((packed)) {
	IMAGE_FORMAT_STILL,
	IMAGE_FORMAT_ANIMATED_CONTAINER,
} format_t;

typedef struct {
	SERVER_IMAGE_IMAGE_HEADER;
} image_header_t;

// generic image
typedef struct {
	SERVER_IMAGE_IMAGE_HEADER;
	char data[1];
} image_t;

// one frame from an animated image
typedef struct {
	float delay; // length frame is displayed
	char data[1];
} animated_image_frame_t;

// animated imagery, i.e. APNG or GIF
typedef struct {
	SERVER_IMAGE_IMAGE_HEADER;
	int frame_count;
	animated_image_frame_t frames[1];
} animated_image_t;

typedef union {
	image_header_t header;
	image_t still;
	animated_image_t animated_container;
} any_image_t;

image_t *image_from_data (int width, int height, char *data);
any_image_t *image_from_url (state_t *state, const char *url);

image_t *image_convert_to_still (any_image_t *);
animated_image_t *image_convert_to_animated (any_image_t *);
animated_image_frame_t *image_convert_to_animated_frame (any_image_t *);

#endif // SERVER_IMAGE_H_
