from quart import Blueprint, request, jsonify, url_for, redirect
from ..models import Project

project_blueprint = Blueprint('project', __name__, url_prefix='/project')

@project_blueprint.route('/query', methods=['GET'])
async def query():
    request_data = await request.form
    project_id = request_data.get('project_id', None)
    project_name = request_data.get('project_name', None)

    if project_id:
        project = await Project.filter(id=project_id).first()
    elif project_name:
        project = await Project.filter(name=project_name).first()
    else:
        projects = await Project.all()
        return redirect(url_for('index.home', projects=projects))
    
    if project:
        return redirect(url_for('index.home', project=project))
    return redirect(url_for('index.home'))


@project_blueprint.route('/create', methods=['POST'])
async def create():
    # this is a post form
    request_data = await request.form
    project_name = request_data.get('project_name', None)
    project_description = request_data.get('project_description', None)

    if project_description == "":
        project_description = None

    project = await Project.create(name=project_name, description=project_description)
    return redirect(url_for('index.home'))


@project_blueprint.route('/update', methods=['POST'])
async def update():
    request_data = await request.form
    project_id = request_data.get('project_id', None)
    project_name = request_data.get('project_name', None)

    project = await Project.filter(id=project_id).first()
    project.name = project_name
    project.updated_at = datetime.datetime.now()
    await project.save()
    return jsonify(project=project.to_dict())


@project_blueprint.route('/delete', methods=['POST'])
async def delete():
    request_data = await request.form
    project_id = request_data.get('project_id', None)
    project_name = request_data.get('project_name', None)

    if project_id:
        await Project.filter(id=project_id).delete()
    elif project_name:
        await Project.filter(name=project_name).delete()
    return jsonify(success=True)