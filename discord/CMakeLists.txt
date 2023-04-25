
add_executable(imgbot_discord
	main.c
)
target_link_libraries(imgbot_discord PRIVATE websockets json-c curl concord_lib)
