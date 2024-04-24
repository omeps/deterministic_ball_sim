use rand::RngCore;
use wgpu::util::DeviceExt;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ft = async {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let (device, queue) = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .unwrap()
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_limits: wgpu::Limits::default(),
                    required_features: wgpu::Features::MAPPABLE_PRIMARY_BUFFERS,
                },
                None,
            )
            .await
            .unwrap();

        let mut noisy_data = [5; 4096 * 4];
        rand::thread_rng().fill_bytes(&mut noisy_data);
        let rand_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("to sort"),
            contents: &noisy_data,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::MAP_READ,
        });
        queue.submit([]);
        let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
        rand_buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });
        device.poll(wgpu::Maintain::Wait);
        rx.receive().await.unwrap().unwrap();
        let slicea = rand_buffer.slice(..).get_mapped_range();
        let slice: &[u32] = bytemuck::cast_slice(&slicea);
        let mut rgb = [0; 2048 * 3 * 100];
        for i in 0..100 {
            for j in 0..2048 {
                rgb[3 * (j + 2048 * i)] = (slice[2 * j] / 16777216) as u8;
                rgb[3 * (j + 2048 * i) + 1] = (slice[2 * j] / 16777216) as u8;
                rgb[3 * (j + 2048 * i) + 2] = (slice[2 * j] / 16777216) as u8;
            }
        }
        let rgb: image::RgbImage = image::RgbImage::from_raw(2048, 100, Vec::from(rgb)).unwrap();
        rgb.save("start.png").unwrap();
        drop(slicea);
        rand_buffer.unmap();
        let rand_buffer =
            deterministic_ball_sim::MergeSort::new(&device).sort(rand_buffer, &device, &queue);
        let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
        rand_buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });
        device.poll(wgpu::Maintain::Wait);
        rx.receive().await.unwrap().unwrap();
        let slice = rand_buffer.slice(..).get_mapped_range();
        let slice: &[u32] = bytemuck::cast_slice(&slice);
        let mut rgb = [0; 2048 * 3 * 100];
        for i in 0..100 {
            for j in 0..2048 {
                rgb[3 * (j + 2048 * i)] = (slice[2 * j] / 16777216) as u8;
                rgb[3 * (j + 2048 * i) + 1] = (slice[2 * j] / 16777216) as u8;
                rgb[3 * (j + 2048 * i) + 2] = (slice[2 * j] / 16777216) as u8;
            }
        }

        let rgb: image::RgbImage = image::RgbImage::from_raw(2048, 100, Vec::from(rgb)).unwrap();
        rgb.save("sorted.png").unwrap();
    };
    pollster::block_on(ft);

    Ok(())
}
