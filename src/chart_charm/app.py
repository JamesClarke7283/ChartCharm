from quart import Quart, Blueprint
from hypercorn.config import Config
from hypercorn.asyncio import serve
import asyncio
import logging
import toga
from toga.style import Pack
from toga.style.pack import COLUMN
import multiprocessing
import socket
from .routes.index import index
import os

log_level_str = os.getenv('LOGLEVEL', 'INFO')
log_level = getattr(logging, log_level_str.upper(), logging.INFO)
logging.basicConfig(level=log_level)

class ChartCharmServer:
    def __init__(self, port, shutdown_event):
        self.app = Quart(__name__, static_folder='./resources', template_folder='./templates')
        self.port = port
        self.shutdown_event = shutdown_event

        # Register Blueprints
        self.app.register_blueprint(index)

    async def run_server(self):
        config = Config()
        config.bind = [f"127.0.0.1:{self.port}"]

        self.server_task = asyncio.ensure_future(serve(self.app, config))

        while not self.shutdown_event.is_set():
            await asyncio.sleep(1)

        self.server_task.cancel()
        try:
            await self.server_task
        except asyncio.CancelledError:
            logging.info("Server has been shut down gracefully.")

class ChartCharmClient(toga.App):
    def startup(self):
        self.shutdown_event = multiprocessing.Event()
        available_port = self.find_available_port()
        self.server_process = multiprocessing.Process(target=self.run_server, args=(available_port, self.shutdown_event))
        self.server_process.start()
        self.on_exit = self.cleanup

        webview = toga.WebView(url=f'http://127.0.0.1:{available_port}', style=Pack(flex=1))
        main_box = toga.Box(children=[webview], style=Pack(direction=COLUMN))
        self.main_window = toga.MainWindow(title=self.formal_name, size=(414, 736))
        self.main_window.content = main_box
        self.main_window.show()

    def find_available_port(self):
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.bind(('', 0))
            return s.getsockname()[1]

    def run_server(self, port, shutdown_event):
        server = ChartCharmServer(port, shutdown_event)
        asyncio.run(server.run_server())

    def cleanup(self, app, **kwargs):
        self.shutdown_event.set()
        self.server_process.join()
        return True

def main():
    return ChartCharmClient()
