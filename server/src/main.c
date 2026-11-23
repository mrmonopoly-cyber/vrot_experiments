#include <string.h>
#include <stdio.h>
#include <stdbool.h>
#include <stdint.h>
#include <unistd.h>
#include <stdatomic.h>

#define UART_BUFFER_POSITION 0xCAFFEE 
#define UART_TRIGGER 0xBEEF

#define LOG_BUFFER 0x00F
#define LOG_BUFFER_POS 0x00E

static atomic_int volatile *uart_buffer = (atomic_int volatile*) UART_BUFFER_POSITION;
static atomic_bool volatile *send_data = (atomic_bool volatile *) UART_TRIGGER;
static bool volatile trigger = (bool volatile *) 0x6767;

void log_function(char* who, char* str)
{
    char* log_buffer = (char*) (volatile char*) LOG_BUFFER;
    volatile int* log_buffer_pos = (volatile int*) LOG_BUFFER_POS;

    int who_len = strlen(who);
    int str_len = strlen(str);
    int msg_size = who_len + 1 + str_len + 1; // who:str\n

    *log_buffer_pos += msg_size;

    //INFO: I am using sprintf instead of snprintf only because this is a dummy code
    sprintf(log_buffer + msg_size, "%s:%s\n", who, str);
}

// Simulated ISR
void isr_send(void)
{
    atomic_store(send_data, true);
    log_function("server", "triggered send interrupt");
}

int uart_init(void)
{
    atomic_init(uart_buffer, 69);
    atomic_init(send_data, false);
    return 0;
}

// Dummy UART write
int uart_write(int msg)
{
    static char temp_bu[1024] = {0};

    snprintf(temp_bu, sizeof(temp_bu), "original write: %p", uart_write);

    log_function("server", temp_bu);
    if (msg < 0)
        return -1;

    atomic_store(uart_buffer, msg);
    trigger = true;

    return sizeof(msg);
}

int proc_main(void)
{
    int payload = 0;
    static char log_buffer[1024] = {0};

    uart_init();

    while(!atomic_load(send_data))
    {
        log_function("server", "waiting interrupt");
        sleep(1);
    }
    atomic_store(send_data, false);

    payload=42;
    snprintf(log_buffer, sizeof(log_buffer), "sending data: %d", payload);
    log_function("server", log_buffer);

    uart_write(payload);
    snprintf(log_buffer, sizeof(log_buffer), "data sent: %d", payload);
    log_function("server", log_buffer);
    sleep(1);

    while(!atomic_load(send_data))
    {
        log_function("server", "waiting interrupt");
        sleep(1);
    }

    payload=42;
    snprintf(log_buffer, sizeof(log_buffer), "sending data: %d", payload);
    log_function("server", log_buffer);

    uart_write(payload);
    snprintf(log_buffer, sizeof(log_buffer), "data sent: %d", payload);
    log_function("server", log_buffer);

    for(;;);

    return 0;
}
