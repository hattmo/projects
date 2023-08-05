import asyncio
import socket


async def main():
    async def handle_connection(
        reader: asyncio.StreamReader, writer: asyncio.StreamWriter
    ):
        res = await reader.readline()
        print("from client:", res)
        server.close()

    sock = socket.socket(fileno=0)
    server = await asyncio.start_unix_server(handle_connection, None, sock=sock)
    await server.serve_forever()


if __name__ == "__main__":
    asyncio.run(main())
