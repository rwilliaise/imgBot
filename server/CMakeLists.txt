
set(IMGBOT_SERVER_COMPUTE_SOURCES
	compute_pthreads.c
)

add_executable(imgbot_server
	main.c
	image.c
	state.c
)
target_link_libraries(imgbot_server websockets CURL::libcurl Threads::Threads m)

if (${OpenCL_FOUND})
	target_link_libraries(imgbot_server OpenCL::OpenCL)

	set(IMGBOT_SERVER_COMPUTE_SOURCES
		compute_cl.c
	)
endif()

target_sources(imgbot_server PRIVATE ${IMGBOT_SERVER_COMPUTE_SOURCES})

