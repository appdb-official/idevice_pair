#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod backend;
mod known_apps;
mod logging;
mod ui;

use eframe::egui;

fn main() -> eframe::Result {
    let logs = logging::init();

    eframe::run_native(
        &format!("idevice pair v{}", env!("CARGO_PKG_VERSION")),
        native_options(),
        Box::new(|cc| {
            ui::setup(&cc.egui_ctx);
            Ok(Box::new(app::App::new(&cc.egui_ctx, logs)))
        }),
    )
}

fn native_options() -> eframe::NativeOptions {
    let viewport = egui::ViewportBuilder::default().with_inner_size([760.0, 620.0]);

    #[cfg(target_os = "macos")]
    let viewport = viewport.with_icon(std::sync::Arc::new(egui::IconData::default()));

    #[cfg(not(target_os = "macos"))]
    let viewport = {
        let icon = eframe::icon_data::from_png_bytes(include_bytes!("../icon.png"))
            .expect("bad icon data");
        viewport.with_icon(std::sync::Arc::new(icon))
    };

    eframe::NativeOptions {
        viewport,
        renderer: pick_renderer(),
        ..Default::default()
    }
}

/// Fall back to OpenGL when Metal fails on Macs patched with OpenCore.
#[cfg(target_os = "macos")]
fn pick_renderer() -> eframe::Renderer {
    use eframe::wgpu;

    let mut descriptor = wgpu::InstanceDescriptor::new_without_display_handle();
    descriptor.backends = wgpu::Backends::METAL;
    let instance = wgpu::Instance::new(descriptor);

    let result = pollster::block_on(async {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .map_err(|e| e.to_string())?;
        adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .map_err(|e| e.to_string())
    });

    match result {
        Ok(_) => eframe::Renderer::Wgpu,
        Err(error) => {
            tracing::warn!("Metal initialization failed; falling back to OpenGL: {error}");
            eframe::Renderer::Glow
        }
    }
}

#[cfg(not(target_os = "macos"))]
fn pick_renderer() -> eframe::Renderer {
    eframe::Renderer::Wgpu
}
