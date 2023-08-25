"""Blueprint for the index route."""
from quart import Blueprint
from quart import render_template

index_blueprint = Blueprint('index', __name__)

@index_blueprint.route('/')
async def home():
    return await render_template('index.html')