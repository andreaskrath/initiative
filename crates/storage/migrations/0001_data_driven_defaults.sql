-- These are "sensible" defaults for the options table, but they can be removed
-- by the user at a later point, if they should wish to.
INSERT INTO options (id, variant, value, sort_order, protected) 
VALUES 
    -- Variant: school
    (x'646fd7ba77dc49ea9e8c3ed2ea46eb44', 'school', 'Abjuration',    0, 1),
    (x'2a695ca1987841d0b6dea4ce36ad9fd0', 'school', 'Conjuration',   1, 1),
    (x'2797bb238513494f8a4a6d3627126be9', 'school', 'Divination',    2, 1),
    (x'bc1f402e510f4ad6aa4bb651a2c11e2e', 'school', 'Enchantment',   3, 1),
    (x'e0077f26cb0449eab96824ebdcd547a9', 'school', 'Evocation',     4, 1),
    (x'b7d450860c214915bc7c9800e548f577', 'school', 'Illusion',      5, 1),
    (x'372ac2e0ea33480ebece915a1ab32f66', 'school', 'Necromancy',    6, 1),
    (x'76ec57679b0a4d4db0b95f128a7a7eec', 'school', 'Transmutation', 7, 1),

    -- Variant: level
    (x'26e2e95f94804322b2358cdedb56b2c0', 'level', 'Cantrip', 0, 1),
    (x'4eba4c6534e14d97b3eee4bc32bad4a5', 'level', 'First',   1, 1),
    (x'1493e731748a4eda90b0e5be0f963b13', 'level', 'Second',  2, 1),
    (x'6966e7be17b34a8ba2a628f66b830de9', 'level', 'Third',   3, 1),
    (x'd43361be1add4c48a22d2f702c811278', 'level', 'Fourth',  4, 1),
    (x'fc66835e031e467485c295776c056488', 'level', 'Fifth',   5, 1),
    (x'9194a50c109b4c5dbbf755c6a8d47f17', 'level', 'Sixth',   6, 1),
    (x'73c64214276946109290475d7d9f4ad1', 'level', 'Seventh', 7, 1),
    (x'e72ac9c427404db382d6c3edabd54e33', 'level', 'Eighth',  8, 1),
    (x'5292724c53c443f5b3509aa6619719c4', 'level', 'Ninth',   9, 1),

    -- Variant: casting_time
    (x'c38e2b82fd8946f8af412726fdac45ca', 'casting_time', '1 action',        0, 1),
    (x'59a5828779114143bc02cb2dbc963e20', 'casting_time', '1 bonus action',  1, 1),
    (x'4ac6bf8668564099a2a33c4b2c1f73f4', 'casting_time', '1 reaction',      2, 1),
    (x'5b53a22b1f6242c2a6bdc2d507b0f46a', 'casting_time', '1 minute',        3, 1),
    (x'8d6b0ee97fbc4642b6d6f06f8e9ed7f3', 'casting_time', '10 minutes',      4, 1),
    (x'7627df65bdee4615bdaf849aedf3a924', 'casting_time', '1 hour',          5, 1),
    (x'56166b279cac4f11bd6c9c23ced1a1ba', 'casting_time', '8 hours',         6, 1),
    (x'e6f244ab0cf24bceadf0616931c1ba05', 'casting_time', '1 day',           7, 1),

    -- Variant: duration
    (x'dc564ed4a5a54a979a257a67cd216d8b', 'duration', 'Instantaneous',   0, 1),
    (x'fc3988dcd6fd47d38609974be7b3166d', 'duration', '1 round',         1, 1),
    (x'3a72b0ffcd374a91b7378ddea1fdcd64', 'duration', '1 minute',        2, 1),
    (x'bc8110ad47a141fc83b2fdf31c6fa325', 'duration', '10 minutes',      3, 1),
    (x'2cb5e46e45da4a949ef607c1ee53a4ac', 'duration', '1 hour',          4, 1),
    (x'f6c89fafe854419c90534a3e9409999e', 'duration', '8 hours',         5, 1),
    (x'fe1285647cb54a548c8d1f7f755a5bea', 'duration', '1 day',           6, 1),
    (x'c9e7bf9e90ab4b1db34fbbee77d0ebd0', 'duration', '7 days',          7, 1),
    (x'4fde410be4f74afa9a25c740aee78797', 'duration', 'Permanent',       8, 1),
    (x'38cf57c403ad4d8c83f8a783f67ab522', 'duration', 'Until dispelled', 9, 1),

    -- Variant: range
    (x'ab364c8e15f34cef836f7bc36fa0b82d', 'range', 'Self',      0, 1),
    (x'356cc828e9324457b693e3bff977e9be', 'range', 'Touch',     1, 1),
    (x'fefe05bc3eb44ed39a0d52d7d96dd279', 'range', 'Sight',     2, 1),
    (x'561f5306f95a46cb8be1f04dae41a6bd', 'range', '30 ft.',    3, 1),
    (x'1c7a4a004b7c4d4fb7a59ae7d1786d90', 'range', '60 ft.',    4, 1),
    (x'9e979de895944b93a75e262f7b3a0677', 'range', '90 ft.',    5, 1),
    (x'6f43bfde443d4b4ca7cf9b851bd552ba', 'range', '120 ft.',   6, 1),
    (x'0df42c5a59cc446086ce2643b98c06ce', 'range', '150 ft.',   7, 1),
    (x'cbdceb4d9d22422c8fb37950c93b869c', 'range', '300 ft.',   8, 1),
    (x'974bf51b35f24b8792e6e6811bd3c2f7', 'range', '500 ft.',   9, 1),
    (x'6463e9941a2144f0b62f979738d5c50b', 'range', '1 mile',    10, 1),
    (x'4c49d0f8d431419e916f7e7e3fb074c6', 'range', 'Unlimited', 11, 1),

    -- Variant: area
    (x'0088614f532b4de2a7ae7647819309ba', 'area', 'Single target',          0, 1),
    (x'2953546813d144db8d7e7ac7f1044b3d', 'area', 'Multiple targets',       1, 1),
    (x'3221b9300c074ed89bf86a62dd7201a4', 'area', 'All creatures in range', 2, 1),
    (x'c2a6ba2035e540408047fc5196a8b4e4', 'area', '5 ft.',                  3, 1),
    (x'cd14157277414052bba4343cfe3e3f8a', 'area', '10 ft.',                 4, 1),
    (x'f8e42f49b6d94f98809a6873f0e4a95d', 'area', '15 ft.',                 5, 1),
    (x'b1edfa29e9ad41da94535c8097f1d50d', 'area', '20 ft.',                 6, 1),
    (x'fb260f37272e4654bb074b93af92e838', 'area', '25 ft.',                 7, 1),
    (x'8e8f6123f674468481465e4e4a401a8d', 'area', '30 ft.',                 8, 1),
    (x'aa008fbab1af4ba5ac6ca1a895c9891e', 'area', '35 ft.',                 9, 1),
    (x'f34114db7e2b4a17965df8f430e079a4', 'area', '40 ft.',                 10, 1),
    (x'e756aa15a9194d3695d4f08d32b4d376', 'area', 'Unlimited',              11, 1),

    -- Variant: source
    (x'3db2c5debd754e84a2a8c1144f7a31e7', 'source', 'Player''s Handbook',              0, 1),
    (x'0dee673504c845a68ac1d63a35cf0916', 'source', 'Dungeon Master''s Guide',         1, 1),
    (x'69c7b1359967448cbe6ce6e4fc38be41', 'source', 'Monster Manual',                  2, 1),
    (x'fad874f7c6e54cd188aeaa48b315a28c', 'source', 'Xanathar''s Guide to Everything', 3, 1),
    (x'411ca88eb0b64772a5967c94e43cf58c', 'source', 'Tasha''s Cauldron of Everything', 4, 1),
    (x'9c8144e73e414cc2b79ecae18fb35ab5', 'source', 'Homebrew',                        5, 1);
