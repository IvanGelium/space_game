extends RefCounted
class_name PlanetConfig


var radius: float = 70.0
var type: String = "earthlike"

func set_radius(r: float) -> PlanetConfig:
	radius = r
	return self

func set_type(t: String) -> PlanetConfig:
	type = t
	return self
