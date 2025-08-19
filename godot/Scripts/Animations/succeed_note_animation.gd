extends Node2D

@export var timeToFade = 0.2
var timePassed = 0

func _enter_tree() -> void:
	self.modulate.r = 0
	self.modulate.b = 0

func _process(delta: float) -> void:
	timePassed += delta
	self.modulate.a = 1 - (timePassed / timeToFade) * 1
	if self.modulate.a <= 0:
		self.queue_free()
