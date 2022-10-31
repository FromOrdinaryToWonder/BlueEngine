#[derive(Debug, Clone, Copy)]
pub struct Operation {
    pub translation: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
}

#[derive(Debug, Clone)]
pub struct Animation {
    pub keyframes: Vec<(u64, Operation)>,
    pub time: std::time::Instant,
    pub target: u128,
    pub difference_translate: (f32, f32, f32),
    pub progressed_translation: (f32, f32, f32),
    pub difference_rotation: (f32, f32, f32),
    pub current_frame: usize,
    pub object: &'static str,
}

impl Animation {
    pub fn new(object: &'static str) -> Self {
        Self {
            keyframes: Vec::new(),
            time: std::time::Instant::now(),
            target: 0,
            difference_translate: (0f32, 0f32, 0f32),
            progressed_translation: (0f32, 0f32, 0f32),
            difference_rotation: (0f32, 0f32, 0f32),
            current_frame: 0,
            object,
        }
    }

    pub fn animate(
        &mut self,
        objects: &mut std::collections::HashMap<&'static str, crate::Object>,
    ) {
        let elapsed = self.time.elapsed().as_millis();
        if elapsed <= self.target {
            objects.get_mut(self.object).unwrap().position(
                self.progressed_translation.0 +(self.difference_translate.0 * elapsed as f32),
                (self.difference_translate.1 + self.progressed_translation.1)  * elapsed as f32,
                (self.difference_translate.2 + self.progressed_translation.2)  * elapsed as f32,
            );
        } else {
            {
                if self.current_frame != 0 {
                    let target_translation = self.keyframes[self.current_frame - 1].1.translation;
                    objects.get_mut(self.object).unwrap().position(
                        target_translation.0 + self.progressed_translation.0,
                        target_translation.1 + self.progressed_translation.1,
                        target_translation.2 + self.progressed_translation.2,
                    );

                    println!("{:?} - keyframe: {}", target_translation.0 + self.progressed_translation.0, self.current_frame);

                    if self.current_frame < self.keyframes.len() {
                        self.progressed_translation = (
                            target_translation.0,
                            target_translation.1,
                            target_translation.2,
                        );
                    }

                    
                }
            }

            if self.current_frame < self.keyframes.len() {
                let next_frame = self.keyframes[self.current_frame];
                self.target = std::time::Duration::from_secs(next_frame.0).as_millis();
                self.difference_translate = (
                    (next_frame.1.translation.0 - self.difference_translate.0) / self.target as f32,
                    (next_frame.1.translation.1 - self.difference_translate.1) / self.target as f32,
                    (next_frame.1.translation.2 - self.difference_translate.2) / self.target as f32,
                );
                self.time = std::time::Instant::now();
                self.current_frame += 1;
            }
        }
    }
}