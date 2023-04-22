
add_executable(imgbot_discord
	main.c
	bot/auth.c
)
target_link_libraries(imgbot_discord websockets json-c curl)
