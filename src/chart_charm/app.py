import asyncio
from hypercorn.config import Config
from hypercorn.asyncio import serve
from quart import Quart
import threading

import toga
from toga.style import Pack
from toga.style.pack import COLUMN, ROW
from os import getenv
import logging

# Import blueprints
from .routes.index import index
import socket

def find_available_port():
    """Find an available port to bind to."""
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind(('', 0)) # Bind to any available port
        return s.getsockname()[1]


# Read LOGLEVEL environment variable, default to 'INFO'
log_level_str = getenv('LOGLEVEL', 'INFO')

# Convert log level string to logging constant
log_level = getattr(logging, log_level_str.upper(), logging.INFO)

# Configure logging with the determined log level
logging.basicConfig(level=log_level)

# Create a Quart app
app = Quart(__name__, static_folder='./resources', template_folder='./templates')

# Register Blueprint for home routes
app.register_blueprint(index)


class ChartCharm(toga.App):
    def startup(self):
        self.available_port = find_available_port()
        # Create an asyncio event loop for the server
        self.loop = asyncio.new_event_loop()
        asyncio.set_event_loop(self.loop)

        # Start the Quart server in a separate thread
        self.server_thread = threading.Thread(target=self.run_server)
        self.server_thread.start()

        self.on_exit = self.cleanup

        webview = toga.WebView(url=f'http://127.0.0.1:{self.available_port}', style=Pack(flex=1))
        main_box = toga.Box(children=[webview], style=Pack(direction=COLUMN))
        self.main_window = toga.MainWindow(title=self.formal_name)
        self.main_window.content = main_box
        self.main_window.show()

    def run_server(self):
        config = Config()
        config.bind = [f"127.0.0.1:{self.available_port}"]
        self.serve_task = self.loop.create_task(serve(app, config))
        try:
            self.loop.run_until_complete(self.serve_task)
        except asyncio.CancelledError:
            logging.info("Server has been shut down gracefully.")

    def cleanup(self, app, **kwargs):
        logging.info("Shutting down...")
        self.loop.call_soon_threadsafe(self.serve_task.cancel)
        self.server_thread.join()
        return True

def main():
    return ChartCharm()
