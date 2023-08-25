from quart import request, jsonify, Blueprint
from ...models import Settings, Theme

theme_blueprint = Blueprint('theme', __name__, url_prefix='/theme')

@theme_blueprint.route('/save', methods=['POST'])
async def save():
    request_data = await request.json
    theme_selected = request_data.get('theme_selected', 'auto')
    
    settings = await Settings.first()
    theme_selected_obj = await Theme.filter(name=theme_selected).first()
    theme_selected_id = theme_selected_obj.id if theme_selected_obj else 1
    settings.theme_selected_id = theme_selected_id
    await settings.save()

    return jsonify(success=True)


@theme_blueprint.route('/get', methods=['GET'])
async def get():
    settings = await Settings.first()
    theme_selected_id = settings.theme_selected_id if settings else 1
    theme_selected_obj = await Theme.filter(id=theme_selected_id).first()
    theme_selected = theme_selected_obj.name if theme_selected_obj else 'default_theme'
    return jsonify(theme_selected=theme_selected)

