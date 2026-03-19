// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Light Acorn Framework: Game Tools
  Lord of the Framework: Veyyr3
*/
pub const VERTEX_SHADER_SRC: &str = r#"#version 330 core
layout(location = 0) in vec3 aPos;
layout(location = 1) in vec2 aTex;
layout(location = 2) in vec4 aCol;
layout(location = 3) in vec4 aNormal;
layout(location = 4) in mat4 aModelMat; 

uniform mat4 uViewProjection;
out vec2 vTex;
out vec4 vCol;

void main() {
    vTex = aTex;
    vCol = aCol;
    gl_Position = uViewProjection * aModelMat * vec4(aPos, 1.0);
}
"#;

pub const FRAGMENT_SHADER_SRC: &str = r#"#version 330 core
in vec2 vTex;
in vec4 vCol;
out vec4 fragColor;
uniform sampler2D uTexture;

void main() {
    fragColor = vCol;
}
"#;