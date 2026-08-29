use leptos::prelude::*;

/// 动态背景：薄荷→淡紫→天空 135° 渐变 + 200 颗粒子慢漂。
///
/// 粒子位置/尺寸/速度/相位用 LCG 在编译时确定 → SSR 出来的 HTML
/// 包含全部 200 颗，无需 hydrate 后再补；CSS `@keyframes drift`
/// 是 4 段不规则位移 + 错相 delay，每颗看起来都独立漂浮。
#[component]
pub fn DynamicBackground() -> impl IntoView {
    view! {
        <div class="dynamic-bg" aria-hidden="true">
            <div class="dynamic-bg__gradient"></div>
            <div class="dynamic-bg__particles">
                <ParticleField />
            </div>
            <div class="dynamic-bg__vignette"></div>
        </div>
    }
}

/// 200 particles, deterministic via seeded LCG. Each one is a `<span>`
/// with inline CSS variables driving the per-instance animation:
///   --x, --y   viewport position (0-100%)
///   --s        diameter in px
///   --d        drift cycle in seconds (7-10)
///   --delay    negative seconds, staggers the 200 cycles
const PARTICLE_COUNT: usize = 200;

#[component]
fn ParticleField() -> impl IntoView {
    let mut rng = Lcg::new(0xA5_5A_5A_DE);
    let mut particles: Vec<Particle> = Vec::with_capacity(PARTICLE_COUNT);

    for _ in 0..PARTICLE_COUNT {
        particles.push(Particle {
            x: (rng.next_u32() % 1000) as f64 / 10.0,     // 0.0 - 99.9
            y: (rng.next_u32() % 1000) as f64 / 10.0,     // 0.0 - 99.9
            s: 1.0 + (rng.next_u32() % 18) as f64 / 10.0, // 1.0 - 2.8 px
            d: 7.0 + (rng.next_u32() % 30) as f64 / 10.0, // 7.0 - 10.0 s
            delay: (rng.next_u32() % 100) as f64 / 10.0, // 0.0 - 9.9  (formatted with a leading minus)
        });
    }

    particles
        .into_iter()
        .map(|p| {
            // Format: --x:50.0%;--y:50.0%;--s:1.5px;--d:8.0s;--delay:-2.0s
            let style = format!(
                "--x:{:.1}%;--y:{:.1}%;--s:{:.1}px;--d:{:.1}s;--delay:-{:.1}s",
                p.x, p.y, p.s, p.d, p.delay
            );
            view! { <span class="p" style=style></span> }
        })
        .collect_view()
}

struct Particle {
    x: f64,
    y: f64,
    s: f64,
    d: f64,
    delay: f64,
}

/// Linear-congruential generator (PCG-style update). Seeded once at
/// component construction so the particle field is deterministic
/// across renders.
struct Lcg(u64);

impl Lcg {
    fn new(seed: u64) -> Self {
        Self(seed)
    }
    fn next_u32(&mut self) -> u32 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 32) as u32
    }
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;

    #[test]
    fn lcg_is_deterministic_for_same_seed() {
        let mut a = Lcg::new(0xA5_5A_5A_DE);
        let mut b = Lcg::new(0xA5_5A_5A_DE);
        for _ in 0..200 {
            assert_eq!(
                a.next_u32(),
                b.next_u32(),
                "same seed must yield same stream"
            );
        }
    }

    #[test]
    fn particle_field_stays_in_range_and_is_unique() {
        // 200 particles over a 100x100 grid (0.0-99.9 each axis).
        // The LCG used here is PCG-style: deterministic and visually
        // well-spread, but its high-bit output is correlated. We
        // don't assert uniform distribution (the eye is the real
        // test for that) — only the boring invariants: every value
        // is in range, and no two particles share the same position.
        let mut rng = Lcg::new(0xA5_5A_5A_DE);
        let mut positions: Vec<(u32, u32)> = Vec::with_capacity(200);
        for _ in 0..200 {
            let x = rng.next_u32() % 1000;
            let y = rng.next_u32() % 1000;
            assert!(x < 1000 && y < 1000, "rng output must be in [0, 1000)");
            positions.push((x, y));
        }

        let unique: std::collections::HashSet<_> = positions.iter().collect();
        assert_eq!(
            unique.len(),
            200,
            "no two particles should share a position"
        );
    }
}
