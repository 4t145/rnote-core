pub mod strike;
pub mod render;
pub mod unit;
pub mod app;
pub mod pad;
pub mod geometry;
pub mod measure;


async fn dev() {
    use wgpu::util::DeviceExt;
    use wgpu::InstanceDescriptor;
    // 创建wgpu实例
    let instance = wgpu::Instance::new(InstanceDescriptor::default());
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

    // 创建缓冲区
    let data = vec![1.0, 2.0, 3.0, 4.0]; // 示例数据
    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Example Buffer"),
        contents: bytemuck::cast_slice(&data),
        usage: wgpu::BufferUsages::VERTEX,
    });
    buffer.destroy()
}