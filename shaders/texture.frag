void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    vec2 uv = fragCoord/iResolution.xy;
    uv-=.5; // center the pixel coordinates origin
    uv.x *= iResolution.x / iResolution.y; // adjust aspect ratio

    uv+=vec2(.5*sin(iTime), .25*sin(iTime*2.0));// look around by moving the origin

    float d = 1.0 / length(uv); // inverse of the pixel distance from the origin
    float a = atan(uv.x, uv.y) / 3.14; // angle of the current pixel relative to the origin x axis

    float dShift = iTime * 1.0; // shifting the distance will make the tunnel slide forwards/backwards
    float aShift = iTime * 0.1; // shifting the angle will make the tunnel rotate

    vec4 texCol = texture(iChannel0, vec2(d + dShift, a+ aShift));
    vec3 dCol = vec3(length(uv));  // used to darken the center of the tunnel

    // Output to screen
    fragColor = vec4(texCol.xyz * dCol, 1.0);
}