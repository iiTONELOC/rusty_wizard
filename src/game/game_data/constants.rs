pub const NORTH: &'static str = "North";
pub const SOUTH: &'static str = "South";
pub const EAST: &'static str = "East";
pub const WEST: &'static str = "West";

pub const NUMBERS: [i8; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

pub const NULL: &'static str = "null";

pub const ROOM_NAMES: [&'static str; 13] = [
    "Meeting Room",
    "Gallery",
    "Conservatory",
    "Sitting Room",
    "Trophy Hall",
    "Study",
    "Summoning Room",
    "Court Yard",
    "Workshop",
    "Foundry",
    "Master's Chamber",
    "Bath",
    "Kitchen",
];

pub const ROOM_ITEMS: [&'static str; 13] = [
    &NULL,
    &NULL,
    "Maps of the Echo Lands",
    &NULL,
    &NULL,
    "Locket of Reflection",
    "Ring of Dispel",
    &NULL,
    "Mermaid Tears",
    "Tablet of Destiny",
    &NULL,
    "Flaming Pearl",
    &NULL,
];

pub const ROOM_DESCRIPTIONS: [&'static str; 13] = [
    "\nYou are in a meeting room. There is a large table in the center of the room.\n",
    "\nYou are in a gallery. There is a collection of paintings on the walls.\n",
    "\nYou are in a conservatory. There is a large collection of plants on the walls.\n",
    "\nYou are in a sitting room. There is a large table in the center of the room.\n",
    "\nYou are in a trophy hall. There is a collection of trophies on the walls.\n",
    "\nYou are in a study. There is a large desk in the center of the room.\n",
    "\nYou are in a summoning room. There is a large table in the center of the room.\n",
    "\nYou are in a court yard. There is a large table in the center of the room.\n",
    "\nYou are in a workshop. There is a large table in the center of the room.\n",
    "\nYou are in a foundry. There is a large table in the center of the room.\n",
    "\nYou are in a master's chamber. There is a large table in the center of the room.\n",
    "\nYou are in a bath. There is a large table in the center of the room.\n",
    "\nYou are in a kitchen. There is a large table in the center of the room.\n",
];

pub const INVALID_DIRECTION: &'static str =
    "\n\nNot a valid direction! Type \"help\" for a list of valid directions.";

pub const DIRECTION_NOT_AVAILABLE: &'static str = "\n\nYou cannot go that way from here.";

pub const ITEM_NOT_FOUND: &'static str = "\n\nImpossible, that item is not in this room!";
