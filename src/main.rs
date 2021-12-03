mod visuals;

fn main()
{
    let mut a = visuals::visualizer::Visualizer::new();

    while a.window.is_open() {
        a.draw_circle((200, 200), 300, visuals::colours::CERISE);
        a.draw_circle((400, 400), 200, visuals::colours::CERISE);
        a.window.update_with_buffer(&a.buffer, 800, 600).unwrap();
    }
}