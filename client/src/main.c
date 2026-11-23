#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <stdatomic.h>

#define UART_BUFFER_POSITION 0xCAFFEE 
#define LOG_BUFFER 0xBEEF
#define LOG_BUFFER_POS 0xCEE

static int volatile uart_buffer;
static volatile bool recv_data = 0;

void log_function(char* who, char* str)
{
    char* log_buffer = (char*) (volatile char*) LOG_BUFFER;
    volatile int* log_buffer_pos = (volatile int*) LOG_BUFFER_POS;

    int who_len = strlen(who);
    int str_len = strlen(str);
    int msg_size = who_len + 1 + str_len; // who:str

    *log_buffer_pos += msg_size;

    //INFO: I am using sprintf instead of snprintf only because this is a dummy code
    sprintf(log_buffer + msg_size, "%s:%s", who, str);
}

// Simulated ISR
void isr_recv(void)
{
    recv_data = true;
    log_function("client", "recv interrupt triggered");
}

// Dummy UART read
int uart_read(int *data)
{
    static char temp_buffer[1024];

    if (data == NULL)
    {
        return -1;
    }

    if (uart_buffer < 0)
    {
        return -1;
    }

    *data = uart_buffer;
    snprintf(temp_buffer, sizeof(temp_buffer), "uart read: %d", uart_buffer);
    log_function("client", temp_buffer);

    return sizeof(uart_buffer);
}

int proc_main(void)
{
    int msg = 0;
    static char log_msg[1024] = {0};

    snprintf(log_msg, sizeof(log_msg), "starting loop");
    log_function("client", log_msg);

    while(!recv_data)
    {
        log_function("client", "waiting message");
        sleep(1);
    }

    if (recv_data)
    {
        snprintf(log_msg, sizeof(log_msg), "received msg");
        log_function("client", log_msg);

        // Clear request
        recv_data = false;

        if ( uart_read(&msg) < 0 )
        {
            log_function("client", "uart read error");
        }

        snprintf(log_msg, sizeof(log_msg), "recv payload: %d", msg);
        log_function("client", log_msg);
    }

    return 0;
}

