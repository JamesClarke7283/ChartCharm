from quart import Blueprint
from .theme import theme_blueprint

settings_blueprint = Blueprint('settings', __name__, url_prefix='/settings')

# Register the blueprints
settings_blueprint.register_blueprint(theme_blueprint)