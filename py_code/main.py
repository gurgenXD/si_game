import asyncio
import json
from uuid import uuid4
from websockets.sync.client import connect
from pprint import pprint



def hello():
    game_uuid = str(uuid4())

    commands = {
        "1": {"Create": {"capacity": 3, "package_uuid": str(uuid4()), "presenter_uuid": str(uuid4())}},
        "2": {"Start": None},
    }

    with connect(f"ws://localhost:8080/ws/ce6b961c-6c45-48dd-b748-c90d37246bc3") as websocket:
        while True:
            a = input("Command: ")
            websocket.send(json.dumps(commands[a]))

            message = websocket.recv()
            print(f"Received: {message}")
            #     message = websocket.recv()
            #     print(f"Received: {message}")

hello()
