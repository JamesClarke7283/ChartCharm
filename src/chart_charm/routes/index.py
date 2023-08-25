"""Blueprint for the index route."""
from quart import Blueprint
from quart import render_template
from ..models import Project, Settings, Theme

index_blueprint = Blueprint('index', __name__)

@index_blueprint.route('/')
async def home():
    projects = await Project.all()
    settings = await Settings.all().first()
    theme_id = settings.theme_selected_id
    theme = await Theme.filter(id=theme_id).first()
    theme = theme.name
    """
    if theme == "auto":
        theme = "dark"
    elif theme == "light":
        theme = "dark"
    else:
        theme = "light"
    """
    return await render_template('index.html', projects=projects, tile_theme=theme)