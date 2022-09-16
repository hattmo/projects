#include <dbus/dbus.h>
#include <stdio.h>
int main(int argc, char **argv)
{
    DBusError err;
    DBusConnection *conn;
    int ret;
    unsigned int serial = 0;
    char *sigvalue = "Hello, world!";
    char *sigvalue2 = "Hello, world!";

    dbus_error_init(&err);

    conn = dbus_bus_get(DBUS_BUS_SESSION, &err);
    if (dbus_error_is_set(&err))
    {
        fprintf(stderr, "Connection Error (%s)\n", err.message);
        dbus_error_free(&err);
    }
    if (NULL == conn)
    {
        exit(1);
    }

    ret = dbus_bus_request_name(conn, "test.signal.sink", DBUS_NAME_FLAG_REPLACE_EXISTING, &err);
    if (dbus_error_is_set(&err))
    {
        fprintf(stderr, "Name Error (%s)\n", err.message);
        dbus_error_free(&err);
    }
    if (DBUS_REQUEST_NAME_REPLY_PRIMARY_OWNER != ret)
    {
        exit(1);
    }

    dbus_bus_add_match(conn, "type='signal',interface='test.signal.Type'", &err); // see signals from the given interface
    dbus_connection_flush(conn);
    if (dbus_error_is_set(&err))
    {
        fprintf(stderr, "Match Error (%s)\n", err.message);
        exit(1);
    }
    printf("Match rule sent\n");

    while (true)
    {
        DBusMessage *msg;

        dbus_connection_read_write(conn, 0);
        msg = dbus_connection_pop_message(conn);

        if (NULL == msg)
        {
            sleep(1);
            continue;
        }

        if (dbus_message_is_signal(msg, "test.signal.Type", "Test"))
        {
            printf("Got Signal with value: %s\n", dbus_message_get_signature(msg));
            if (!dbus_message_iter_init(msg, &args))
            {
                fprintf(stderr, "Message Has no arguments!\n");
            }
            else if (DBUS_TYPE_STRING != dbus_message_iter_get_arg_type(&args))
            {
                fprintf(stderr, "Argument is not string!\n");
            }
            else
            {
                dbus_message_iter_get_basic(&args, &sigvalue2);
                printf("Got String: %s\n", sigvalue2);
            }
        }

        dbus_message_unref(msg);
    }

    return 0;
}