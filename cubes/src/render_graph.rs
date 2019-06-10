use amethyst::{
    ecs::{ReadExpect, Resources, SystemData},
    renderer::{
        pass,
        rendy::{
            factory::Factory,
            graph::{
                present::PresentNode,
                render::{RenderGroupDesc, SubpassBuilder},
                GraphBuilder,
            },
            hal::{command, format::Format, image},
            mesh::PosTex,
        },
        types, GraphCreator,
    },
    window::{ScreenDimensions, Window},
};

use std::ops::Deref;

use std::sync::Arc;

#[derive(Default)]
pub struct MyRenderGraphCreator {
    screen_dimensions: Option<ScreenDimensions>,
    window_surface_format: Option<Format>,
}

impl GraphCreator<types::DefaultBackend> for MyRenderGraphCreator {
    // Need rebuild?
    fn rebuild(&mut self, res: &Resources) -> bool {
        // Read the screen dimensions to see if I've to rebuild the graph
        let actual_sd = res.try_fetch::<ScreenDimensions>();
        /*if self.screen_dimensions.as_ref() != actual_sd.as_ref().map(|d| d.deref() ){
            self.screen_dimensions = actual_sd.map(|d| d.clone());
        }
        false*/
        self.screen_dimensions.as_ref() != actual_sd.as_ref().map(|d| d.deref())
    }

    fn builder(
        &mut self,
        factory: &mut Factory<types::DefaultBackend>,
        res: &Resources,
    ) -> GraphBuilder<types::DefaultBackend, Resources> {
        let actual_sd = res.try_fetch::<ScreenDimensions>();
        self.screen_dimensions = actual_sd.map(|d| d.clone());

        let dimensions = self.screen_dimensions.as_ref().unwrap();
        let window_image_kind =
            image::Kind::D2(dimensions.width() as u32, dimensions.height() as u32, 1, 1);

        // Retrieve a reference to the target window,
        // which is created by the WindowBundle
        let window = <ReadExpect<'_, Arc<Window>>>::fetch(res);

        // Now create new surface
        let surface = factory.create_surface(&window);

        // Get cached surface
        let surface_format = self
            .window_surface_format
            .get_or_insert_with(|| factory.get_surface_format(&surface));

        let mut graph_builder = GraphBuilder::new();

        let color_image = graph_builder.create_image(
            window_image_kind,
            1,
            *surface_format,
            Some(command::ClearValue::Color([0., 0., 0., 1.].into())),
        );

        let depth = graph_builder.create_image(
            window_image_kind,
            1,
            Format::D32Sfloat,
            Some(command::ClearValue::DepthStencil(
                command::ClearDepthStencil(1.0, 0),
            )),
        );

        let pass1 = graph_builder.add_node(
            // Creates a render pass
            SubpassBuilder::new()
                .with_group(pass::DrawPbrDesc::default().builder())
                //.with_group(pass::DrawFlatDesc::default().builder())
                .with_color(color_image)
                .with_depth_stencil(depth)
                .into_pass(),
        );

        // Finally, add the pass to the graph.
        // The PresentNode takes its input and applies it to the surface.
        graph_builder
            .add_node(PresentNode::builder(&factory, surface, color_image).with_dependency(pass1));

        graph_builder
    }
}
