import tortoise
from tortoise import fields, models


class Theme(models.Model):
    id = fields.IntField(pk=True)
    name = fields.CharField(max_length=25, unique=True)

class Settings(models.Model):
    __tablename__ = "settings"
    id = fields.IntField(pk=True)
    theme_selected = fields.ForeignKeyField("models.Theme", related_name="settings")


class Project(models.Model):
    __tablename__ = "projects"
    id = fields.IntField(pk=True)
    name = fields.CharField(max_length=255)
    description = fields.TextField(null=True)
    created_at = fields.DatetimeField(auto_now_add=True)
    updated_at = fields.DatetimeField(auto_now=True)

    def __str__(self):
        return self.name

class DataPoint(models.Model):
    __tablename__ = "data_points"
    id = fields.IntField(pk=True)
    project = fields.ForeignKeyField("models.Project", related_name="data_points")
    data = fields.FloatField()
    created_at = fields.DatetimeField(auto_now_add=True)
    updated_at = fields.DatetimeField(auto_now=True)

    def __str__(self):
        return f"{self.data} for {self.project.name}"

async def check_empty():
    theme_count = await Theme.all().count()
    if not theme_count:
        return True
    return False

async def populate():
    # Add the default themes
    await Theme(name="auto").save()
    await Theme(name="light").save()
    await Theme(name="dark").save()

    # Set the default theme to auto
    await Settings(theme_selected_id=1).save()
