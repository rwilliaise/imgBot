cmake_minimum_required(VERSION 3.25)
project(imgbot)

set(THREADS_PREFER_PTHREADS_FLAG ON)

include(ExternalProject)

find_program(MAKE_EXE
	NAMES gmake mingw32-make make 
	NAMES_PER_DIR
	DOC "GNU Make"
)

find_program(PATCH_EXE
	NAMES patch
	NAMES_PER_DIR
	DOC "Unix patch"
)

externalproject_add(concord
	URL 			  https://github.com/Cogmasters/concord/archive/refs/tags/v2.2.0.tar.gz 
	URL_MD5 		  bc154a91e9625db199fa623ce1620953
	CONFIGURE_COMMAND ""
	BUILD_COMMAND 	  ${MAKE_EXE} -C <SOURCE_DIR>
	INSTALL_COMMAND   ${MAKE_EXE} -C <SOURCE_DIR> install PREFIX=<INSTALL_DIR>
)

externalproject_get_property(concord INSTALL_DIR)

set(concord_LIBRARY ${INSTALL_DIR}/lib/${CMAKE_STATIC_LIBRARY_PREFIX}discord${CMAKE_STATIC_LIBRARY_SUFFIX})

add_library(concord_lib STATIC IMPORTED)
set_target_properties(concord_lib PROPERTIES IMPORTED_LOCATION ${concord_LIBRARY})
target_include_directories(concord_lib INTERFACE ${INSTALL_DIR}/include)

unset(concord_LIBRARY)
unset(INSTALL_DIR)

find_package(OpenCL)
find_package(libwebsockets)
find_package(CURL REQUIRED)
find_package(json-c REQUIRED)
find_package(Threads REQUIRED)

add_subdirectory(discord)
add_subdirectory(server)
