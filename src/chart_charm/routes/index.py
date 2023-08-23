"""Blueprint for the index route."""
from quart import Blueprint
from quart import render_template

index = Blueprint('index', __name__)

@index.route('/')
async def home():
    return await render_template('index.html')