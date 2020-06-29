import json
import asyncio
import io


async def get_processes():
    proc = await asyncio.create_subprocess_shell(
        "ps aux", stdout=asyncio.subprocess.PIPE
    )
    stdout, _ = await proc.communicate()

    input_data = io.StringIO(stdout.decode("utf8"))

    # Headers
    line = input_data.readline()
    cmd_idx = line.find("COMMAND")
    keys = [i for i in line[:cmd_idx].split(" ") if i]

    for line in input_data.readlines():
        values = [i for i in line[:cmd_idx].split(" ") if i]

        # since py37 all dicts are ordered on key insert order
        output = {keys[i]: values[i] for i in range(len(keys))}
        output["COMMAND"] = line[cmd_idx:]

        yield output


async def show_processes():
    # Perpare the data
    print("Getting process data")
    json_data = json.dumps([row async for row in get_processes()])
    json_data = json_data.encode()
    print(f"Sending {len(json_data)} bytes")

    # Send the data
    (_, writer) = await asyncio.open_connection(host="localhost", port=7777)
    writer.write(json_data)
    print("Sent to server")


async def main():
    print("Starting client")
    while True:
        print("Starting")
        await show_processes()
        await asyncio.sleep(5)


asyncio.run(main())
