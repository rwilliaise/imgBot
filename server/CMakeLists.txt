
set(IMGBOT_SERVER_COMPUTE_SOURCES
	compute_pthreads.c
)

add_executable(imgbot_server
	main.c
	image.c
)
target_link_libraries(imgbot_server websockets Threads::Threads)

if (${OpenCL_FOUND})
	target_link_libraries(imgbot_server OpenCL::OpenCL)

	set(IMGBOT_SERVER_COMPUTE_SOURCES
		compute_cl.c
	)
endif()

target_sources(imgbot_server ${IMGBOT_SERVER_COMPUTE_SOURCES})

