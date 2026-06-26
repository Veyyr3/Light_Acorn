// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Light Acorn Framework: Game Tools
  Lord of the Framework: Veyyr3
*/

use crate::acorn_kernel::prelude::*;
use crate::acorn_tools::acorn_game_tools::prelude::*;

// ---------------------------- Acorn Functions Sets ----------------------------
const AGT_SIMPLE_COLLISION: [AcornFunction; 4] = 
    [
        agt_xz_grid_create,
        agt_xz_grid_do_simple_collision,
        agt_xz_grid_clear,
        agt_do_entities_move,
    ];

const AGT_SLIDE_COLLISION: [AcornFunction; 4] = 
    [
        agt_xz_grid_create,
        agt_xz_grid_do_slide_collision,
        agt_xz_grid_clear,
        agt_do_entities_move,
    ];