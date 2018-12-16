/*
 * Copyright (c) 2018. Anthony Callahan.
 */

/*

    This is just an experiment getting to see what rust can really do.
    We use Three-rs (its three.js but in rust.
    Don't expect a lot of comments
    Also just keep in mind this file will be uhhh not the greatest in terms of
    organization and readability.


*/
//imports the crate..
extern crate three;
use three::Object;

//plane will use pbr just to test.
// TODO add pbr support / custom material loader & mesh importer.

fn make_square() -> three::Geometry {
    three::Geometry::plane(1.0, 1.0)
}

//programs main entry point.
fn main()
{
    let title = "Toy thing";

    let mut window = three::Window::new(title); //crates a mutable window from the Three crate using the title variable.

    let material_1 = three::material::Basic {
        color: 0xFFFF00,
        .. Default::default()
    };

    let mut mesh = window.factory.mesh(make_square(), material_1);

    window.scene.add(&mesh);

    window.scene.background = three::Background::Color(0xC6FF0FF);

    //game loop
    let center = [0.0, 0.0];
    let yextent = 1.0;
    let zrange  = -1.0 .. 1.0;
    let camera = window.factory.orthographic_camera(center, yextent, zrange);
    while window.update() {
        window.render(&camera);
    }
}