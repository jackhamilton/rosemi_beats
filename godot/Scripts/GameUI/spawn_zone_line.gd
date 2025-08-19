extends Line2D

func _ready() -> void:
	var parent = get_parent()
	if parent is SpawnZone:
		var parent_zone = parent as SpawnZone
		position.x = parent_zone.line_x
