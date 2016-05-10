#version 150 core

in vec2 v_Uv;
out vec4 o_Color;

uniform sampler2D t_Texture;

void main() {
  vec4 alpha = texture(t_Texture, v_Uv).aaaa;
  vec4 color = alpha * texture(t_Texture, v_Uv);

  if(color.a < 0.1)
    discard;

  o_Color = color;
}
