extends Camera3D

## Target to look at (x, 0., y)
@export var to_target: Vector2 = Vector2(0., 0.);
## Zoom - How far is the camera from the ground (y = 0.)
@export var to_zoom: float = 15.;
## Rotation - specifies the rotation around y. (rad)
@export var to_rotation: float = 0.;

@export var MOVE_SPEED: float = 3.;
@export var ZOOM_SPEED: float = 2.;
@export var MIN_ZOOM: float = 15.;
@export var MAX_ZOOM: float = 150.;
@export var ROTATION_SPEED: float = PI/6.;
@export var SMOOTHNESS: float = 5.;



var s_to_target: Vector2;
var s_to_zoom: float;
var s_to_rotation: float;

enum Direction {
	Backward = 0,
	Right = 1,
	Forward = 2,
	Left = 3,
}

func _ready() -> void:
	rotation.x = deg_to_rad(-45.);
	s_to_target = to_target;
	s_to_zoom = to_zoom;
	s_to_rotation = to_rotation;
	calculate_position_and_rotation(1.0);

func _process(delta: float) -> void:
	handle_inputs(delta);
	calculate_position_and_rotation(SMOOTHNESS * delta);
	
	
func handle_inputs(delta: float) -> void:
	if Input.is_action_pressed("move_forward"):
		to_target += get_movement(Direction.Forward) * MOVE_SPEED * to_zoom * delta;
		
	if Input.is_action_pressed("move_backward"):
		to_target += get_movement(Direction.Backward) * MOVE_SPEED * to_zoom * delta;
		
	if Input.is_action_pressed("move_left"):
		to_target += get_movement(Direction.Left) * MOVE_SPEED * to_zoom * delta;
	
	if Input.is_action_pressed("move_right"):
		to_target += get_movement(Direction.Right) * MOVE_SPEED * to_zoom * delta;
		
	if Input.is_action_just_pressed("zoom_in"):
		to_zoom = max(to_zoom - ZOOM_SPEED, MIN_ZOOM)
		
	if Input.is_action_just_pressed("zoom_out"):
		to_zoom = min(to_zoom + ZOOM_SPEED, MAX_ZOOM)
	
	if Input.is_action_just_pressed("rotation_clockwise"):
		to_rotation += ROTATION_SPEED;
		 
	if Input.is_action_just_pressed("otation_anticlockwise"):
		to_rotation -= ROTATION_SPEED;
		
func get_movement(direction: Direction) -> Vector2:
		var angle = direction * (PI / 2.) + to_rotation;
		return Vector2(sin(angle), cos(angle));

func calculate_position_and_rotation(smoothness_delta: float) -> void:
	s_to_target = lerp(s_to_target, to_target, smoothness_delta);
	s_to_zoom = lerp(s_to_zoom, to_zoom, smoothness_delta);
	s_to_rotation = lerp_angle(s_to_rotation, to_rotation, smoothness_delta);
	position = Vector3(
		s_to_target.x + sin(s_to_rotation) * s_to_zoom,
		s_to_zoom,
		s_to_target.y + cos(s_to_rotation) * s_to_zoom,
	);
	rotation.y = s_to_rotation;
