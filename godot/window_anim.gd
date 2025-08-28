extends AnimatedSprite2D
class_name WindowAnim

@export var odds = 100

var delta = 0

func _process(_delta: float) -> void:
	self.delta += _delta
	if self.delta > 1:
		self.delta = 0
	else:
		return
	if is_playing():
		return
	else:
		visible = false
	if randf() < 1.0/odds:
		if randi() % 4 == 1:
			visible = true
			pick_anim()

func pick_anim():
	if is_playing():
		return

	match randi() % 5:
		0:
			self.play("anju")
		1:
			self.play("sanbk")
		2:
			self.play("lize")
		3:
			self.play("inui")
