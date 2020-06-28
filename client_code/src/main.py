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


async def main():
    (_, writer) = await asyncio.open_connection(host="localhost", port=7777)
    json_data = json.dumps([row async for row in get_processes()])
    writer.write(json_data.encode())


asyncio.run(main())
