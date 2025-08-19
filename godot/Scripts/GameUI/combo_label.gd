extends Label
@export var scorer: Scorer

func _process(_delta: float) -> void:
	var combo = scorer.combo
	text = "%sx" % combo
