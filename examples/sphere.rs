use t3::geometry::Sphere;
use t3::light::Light;
use t3::math::Vec3;

fn main() {
    let mut app = t3::App::new();

    app.scene
        .add_geometry(Sphere::new(6.5, Vec3::new(0.0, 0.0, 0.0)));
    app.scene.add_light(Light::new(Vec3::new(30.0, 80.0, 0.0)));

    app.render(|app, time| {
        let lights = &mut app.scene.lights;
        let light = &mut lights[0];
        let light_pos = light.get_pos();
        light.set_pos(Vec3::new(
            20.0 * (time as f32 / 1000.0).sin(),
            light_pos.y,
            20.0 * (time as f32 / 1000.0).cos(),
        ));
    });
}
