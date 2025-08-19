extends Label
@export var scorer: Scorer

func _process(_delta: float) -> void:
	var score = scorer.score
	text = "%s" % score
