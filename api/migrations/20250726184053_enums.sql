CREATE TYPE alignment AS ENUM (
    'any',
    'chaotic_evil',
    'chaotic_good',
    'chaotic_neutral',
    'lawful_evil',
    'lawful_good',
    'lawful_neutral',
    'neutral',
    'neutral_evil',
    'neutral_good',
    'unaligned'
);

CREATE TYPE attribute AS ENUM (
    'strength', 
    'dexterity', 
    'constitution', 
    'intelligence', 
    'wisdom', 
    'charisma'
);

CREATE TYPE class AS ENUM (
    'artificer',
    'barbarian',
    'bard',
    'cleric',
    'druid',
    'fighter',
    'monk',
    'paladin',
    'ranger',
    'rogue',
    'sorcerer',
    'warlock',
    'wizard'
);

CREATE TYPE condition AS ENUM (
    'blinded',
    'charmed',
    'deafened',
    'frightened',
    'grappled',
    'incapacitated',
    'invisible',
    'paralyzed',
    'petrified',
    'poisoned',
    'prone',
    'restrained',
    'stunned',
    'unconscious',
    'exhaustion'
);

CREATE TYPE damage_type AS ENUM (
    'acid',
    'bludgeoning',
    'cold',
    'fire',
    'force',
    'lightning',
    'necrotic',
    'non_magical_bludgeoning',
    'non_magical_piercing',
    'non_magical_slashing',
    'piercing',
    'poison',
    'psychic',
    'radiant',
    'slashing',
    'thunder'
);

CREATE TYPE language AS ENUM (
    'abyssal',
    'celestial',
    'common',
    'deep_speech',
    'draconic',
    'dwarvish',
    'elvish',
    'giant',
    'gnomish',
    'goblin',
    'halfling',
    'infernal',
    'orc',
    'primordial',
    'sylvan',
    'undercommon'
);

CREATE TYPE magic_school AS ENUM (
    'abjuration',
    'conjuration',
    'divination',
    'enchantment',
    'evocation',
    'illusion',
    'necromancy',
    'transmutation'
);

CREATE TYPE monster_type AS ENUM (
    'aberration',
    'beast',
    'celestial',
    'construct',
    'dragon',
    'elemental',
    'fey',
    'fiend',
    'giant',
    'humanoid',
    'monstrosity',
    'ooze',
    'plant',
    'undead'
);

CREATE TYPE movement AS ENUM (
    'normal', 
    'burrow',
    'climb',
    'fly',
    'fly_hover', 
    'swim'
);

CREATE TYPE sight AS ENUM (
    'blindsight',
    'darkvision',
    'tremorsense',
    'truesight'
);

CREATE TYPE size AS ENUM (
    'tiny',
    'small',
    'medium',
    'large',
    'huge',
    'gargantuan'
);

CREATE TYPE skill AS ENUM (
    'acrobatics',
    'animal_handling',
    'arcana',
    'athletics',
    'deception',
    'history',
    'insight',
    'intimidation',
    'investigation',
    'medicine',
    'nature',
    'perception',
    'performance',
    'persuasion',
    'religion',
    'sleight_of_hand',
    'stealth',
    'survival'
);

CREATE TYPE spell_level AS ENUM (
    'cantrip',
    'first',
    'second',
    'third',
    'fourth',
    'fifth',
    'sixth',
    'seventh',
    'eighth',
    'ninth'
);
