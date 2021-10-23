/**
 * \file hello_world.c
 * \author Graham Riches
 * \brief A sample hello world application to show the usage of the binary-tools application
 * \version 0.1
 * \date 2021-10-23
 * 
 * @copyright Copyright (c) 2021
 * 
 */

/****************************** Includes ******************************/
#include <stdio.h>

/****************************** Local Variables ******************************/
static char print_buffer[1000];

/****************************** Local Function Declarations ******************************/
static void print_hello_message( void );

/****************************** Function Definitions ******************************/
/**
 * \brief Main sample application
 * 
 * \param argc Number of CLI arguments
 * \param argv CLI string arguments list
 * \retval int Program result code
 */
int main(int argc, char* argv[]) {
    print_hello_message();
    return 0;
}

/**
 * \brief A really weird and inefficient way of printing hello world that creates a more interesting
 *        symbol table for the example.
 */
static void print_hello_message( void ) {
    int bytesWritten = snprintf(print_buffer, 1000, "%s %s\r\n", "Hello", "World!");
    print_buffer[++bytesWritten] = '\0';
    printf("%s", print_buffer);
}
