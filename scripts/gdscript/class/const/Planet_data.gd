# Data.gd
extends RefCounted
class_name Planet_data

enum Layer {
	SPACE,
	ATMO20,
	ATMO40,
	ATMO60,
	ATMO80,
	ATMO100,
	MOUNTIN,
	SURFACE,
	DIRT,
	STONE,
	CORE,
}

enum BiomeType { 
	SPACE,
	AIR20, 
	AIR40, 
	AIR60, 
	AIR80, 
	AIR100, 
	ICE,
	SNOW, 
	SAND, 
	GRASS, 
	DIRT,
	ROCK,
	LAVA, 
	 }

const BIOMES = [
	{
		"enum":BiomeType.SPACE,
		"color":Color(0.0,0.0,0.0),
	},
	 {
		"enum":BiomeType.AIR100,
		"color":Color(0.6, 0.6, 0.9, 1),
	},
	 {
		"enum":BiomeType.GRASS,
		"color": Color(0.2, 0.5, 0.2),
	},
	 {
		"enum":BiomeType.DIRT,
		"color":Color(0.4, 0.3, 0.1),
	},
	 {
		"enum":BiomeType.SNOW,
		"color": Color(0.8, 0.8, 0.9),
	},
	 {
		"enum":BiomeType.ICE,
		"color":Color(0.6, 0.6, 0.9),
	},
	 {
		"enum":BiomeType.SAND,
		"color":Color(0.9, 0.9, 0.5),
	},
	 {
		"enum":BiomeType.ROCK,
		"color":Color(0.3,0.3,0.3),
	},
	 {
		"enum":BiomeType.LAVA,
		"color":Color(1.0, 0.3, 0.1),
	}
]


	
