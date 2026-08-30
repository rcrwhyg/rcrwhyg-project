use leptos::prelude::*;

/// 番茄钟。当前采用 vanilla JS 实现（纯前端、无 server fn）。
///
/// ⚠️ 为什么是 vanilla JS（临时替代，非终态）：
/// 这一版重写前，leptos 0.8 的 `on:click` 闭包在 SSR/hydrate 边界反复
/// 绑定不上（按钮没反应）。已确认是闭包序列化在 SSR/hydrate 不一致导致，
/// 规则见 `rules/leptos-ssr-hydrate.md`。为不再阻塞，先回退到
/// 验证过的 vanilla JS 方案让站点能上线。
///
/// TODO(leptos): 等对 leptos 0.8 的 SSR/hydrate 事件绑定吃透后，
/// 把这里换成 leptos 闭包方案（见 ThemeControls 的写法参考）。
#[component]
pub fn ClockPage() -> impl IntoView {
    view! {
        <section class="mx-auto my-8 max-w-4xl px-4 flex justify-center">
            <div class="clock-wrap">
                <div class="clock-mode">
                    <button type="button" class="is-active" id="clock-mode-work">"工作 25"</button>
                    <button type="button" id="clock-mode-break">"休息 5"</button>
                </div>

                <div class="clock-ring">
                    <svg viewBox="0 0 240 240" class="h-full w-full -rotate-90">
                        <circle
                            cx="120"
                            cy="120"
                            r="110"
                            fill="none"
                            stroke="var(--glass-border)"
                            stroke-width="10"
                        ></circle>
                        <circle
                            id="clock-ring-progress"
                            cx="120"
                            cy="120"
                            r="110"
                            fill="none"
                            stroke="var(--accent-2)"
                            stroke-width="10"
                            stroke-linecap="round"
                        ></circle>
                    </svg>
                    <div class="clock-time" id="clock-time-text">"25:00"</div>
                </div>

                <div class="flex gap-3">
                    <button type="button" class="btn" id="clock-start">"开始"</button>
                    <button type="button" class="btn secondary" id="clock-pause" disabled>"暂停"</button>
                    <button type="button" class="btn ghost" id="clock-reset">"重置"</button>
                </div>
            </div>
        </section>

        <script inner_html=r#"
            (function() {
                var TIMES = { work: 25 * 60, 'break': 5 * 60 };
                var RADIUS = 110;
                var CIRC = 2 * Math.PI * RADIUS;

                var mode = 'work';
                var remaining = TIMES.work;
                var running = false;
                var interval = null;

                var timeText = document.getElementById('clock-time-text');
                var ring = document.getElementById('clock-ring-progress');
                var workBtn = document.getElementById('clock-mode-work');
                var breakBtn = document.getElementById('clock-mode-break');
                var startBtn = document.getElementById('clock-start');
                var pauseBtn = document.getElementById('clock-pause');
                var resetBtn = document.getElementById('clock-reset');

                if (!timeText || !ring) return;

                function fmt(secs) {
                    var m = Math.floor(secs / 60);
                    var s = secs % 60;
                    return (m < 10 ? '0' : '') + m + ':' + (s < 10 ? '0' : '') + s;
                }

                function render() {
                    timeText.textContent = fmt(remaining);
                    var total = TIMES[mode];
                    var frac = 1 - remaining / total;
                    var offset = CIRC * Math.max(0, Math.min(1, frac));
                    ring.style.strokeDasharray = CIRC.toFixed(2);
                    ring.style.strokeDashoffset = offset.toFixed(2);
                }

                function setMode(next) {
                    if (interval) { clearInterval(interval); interval = null; }
                    running = false;
                    mode = next;
                    remaining = TIMES[next];
                    workBtn.classList.toggle('is-active', next === 'work');
                    breakBtn.classList.toggle('is-active', next === 'break');
                    startBtn.disabled = false;
                    pauseBtn.disabled = true;
                    startBtn.textContent = '开始';
                    render();
                }

                function tick() {
                    remaining = Math.max(0, remaining - 1);
                    if (remaining === 0) {
                        if (interval) { clearInterval(interval); interval = null; }
                        running = false;
                        startBtn.disabled = false;
                        pauseBtn.disabled = true;
                        startBtn.textContent = '开始';
                    }
                    render();
                }

                workBtn.addEventListener('click', function() { setMode('work'); });
                breakBtn.addEventListener('click', function() { setMode('break'); });

                startBtn.addEventListener('click', function() {
                    if (running) return;
                    running = true;
                    startBtn.disabled = true;
                    pauseBtn.disabled = false;
                    startBtn.textContent = '进行中';
                    interval = setInterval(tick, 1000);
                });

                pauseBtn.addEventListener('click', function() {
                    if (interval) { clearInterval(interval); interval = null; }
                    running = false;
                    startBtn.disabled = false;
                    pauseBtn.disabled = true;
                    startBtn.textContent = '开始';
                });

                resetBtn.addEventListener('click', function() {
                    if (interval) { clearInterval(interval); interval = null; }
                    running = false;
                    startBtn.disabled = false;
                    pauseBtn.disabled = true;
                    startBtn.textContent = '开始';
                    remaining = TIMES[mode];
                    render();
                });

                render();
            })();
        "#></script>
    }
}
