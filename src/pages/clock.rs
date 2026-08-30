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
/// 把这里换成 leptos 闭包方案（见 `rules/leptos-ssr-hydrate.md` 写法 C/D）。
#[component]
pub fn ClockPage() -> impl IntoView {
    view! {
        <section class="mx-auto my-8 max-w-4xl px-4 flex justify-center">
            <div class="clock-wrap page-panel">
                <div class="clock-mode">
                    <button type="button" class="is-active" id="clock-mode-work">"工作"</button>
                    <button type="button" id="clock-mode-break">"休息"</button>
                </div>

                <div class="clock-custom" aria-label="自定义时长">
                    <label class="clock-custom-field">
                        <span class="clock-custom-label">"工作"</span>
                        <input
                            type="number"
                            id="clock-work-min"
                            class="clock-custom-input"
                            min="1"
                            max="120"
                            value="25"
                            inputmode="numeric"
                            aria-label="工作时长（分钟）"
                        />
                        <span class="clock-custom-unit">"分"</span>
                    </label>
                    <label class="clock-custom-field">
                        <span class="clock-custom-label">"休息"</span>
                        <input
                            type="number"
                            id="clock-break-min"
                            class="clock-custom-input"
                            min="1"
                            max="60"
                            value="5"
                            inputmode="numeric"
                            aria-label="休息时长（分钟）"
                        />
                        <span class="clock-custom-unit">"分"</span>
                    </label>
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
                var TIMES = { work: 25 * 60, break: 5 * 60 };
                var LIMITS = { work: [1, 120], break: [1, 60] };
                var STORAGE = { work: 'rcrwhyg.clock.work', break: 'rcrwhyg.clock.break' };
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
                var workInput = document.getElementById('clock-work-min');
                var breakInput = document.getElementById('clock-break-min');
                var startBtn = document.getElementById('clock-start');
                var pauseBtn = document.getElementById('clock-pause');
                var resetBtn = document.getElementById('clock-reset');

                if (!timeText || !ring || !workInput || !breakInput) return;

                function clamp(n, lo, hi) {
                    return Math.max(lo, Math.min(hi, n));
                }

                function fmt(secs) {
                    var m = Math.floor(secs / 60);
                    var s = secs % 60;
                    return (m < 10 ? '0' : '') + m + ':' + (s < 10 ? '0' : '') + s;
                }

                function updateModeLabels() {
                    workBtn.textContent = '工作 ' + TIMES.work / 60;
                    breakBtn.textContent = '休息 ' + TIMES.break / 60;
                }

                function syncInputsFromTimes() {
                    workInput.value = TIMES.work / 60;
                    breakInput.value = TIMES.break / 60;
                }

                function setCustomInputsEnabled(enabled) {
                    workInput.disabled = !enabled;
                    breakInput.disabled = !enabled;
                }

                function loadSettings() {
                    try {
                        var w = parseInt(localStorage.getItem(STORAGE.work), 10);
                        var b = parseInt(localStorage.getItem(STORAGE.break), 10);
                        if (!isNaN(w)) TIMES.work = clamp(w, LIMITS.work[0], LIMITS.work[1]) * 60;
                        if (!isNaN(b)) TIMES.break = clamp(b, LIMITS.break[0], LIMITS.break[1]) * 60;
                    } catch (e) {}
                    syncInputsFromTimes();
                    updateModeLabels();
                }

                function saveSettings() {
                    try {
                        localStorage.setItem(STORAGE.work, String(TIMES.work / 60));
                        localStorage.setItem(STORAGE.break, String(TIMES.break / 60));
                    } catch (e) {}
                }

                function applyCustomTimes() {
                    var w = clamp(parseInt(workInput.value, 10) || 25, LIMITS.work[0], LIMITS.work[1]);
                    var b = clamp(parseInt(breakInput.value, 10) || 5, LIMITS.break[0], LIMITS.break[1]);
                    workInput.value = w;
                    breakInput.value = b;
                    TIMES.work = w * 60;
                    TIMES.break = b * 60;
                    saveSettings();
                    updateModeLabels();
                    if (!running) {
                        remaining = TIMES[mode];
                        render();
                    }
                }

                function render() {
                    timeText.textContent = fmt(remaining);
                    var total = TIMES[mode];
                    var frac = total > 0 ? 1 - remaining / total : 0;
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
                    setCustomInputsEnabled(true);
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
                        setCustomInputsEnabled(true);
                    }
                    render();
                }

                workBtn.addEventListener('click', function() { setMode('work'); });
                breakBtn.addEventListener('click', function() { setMode('break'); });

                workInput.addEventListener('change', applyCustomTimes);
                breakInput.addEventListener('change', applyCustomTimes);
                workInput.addEventListener('blur', applyCustomTimes);
                breakInput.addEventListener('blur', applyCustomTimes);

                startBtn.addEventListener('click', function() {
                    if (running) return;
                    applyCustomTimes();
                    running = true;
                    startBtn.disabled = true;
                    pauseBtn.disabled = false;
                    startBtn.textContent = '进行中';
                    setCustomInputsEnabled(false);
                    interval = setInterval(tick, 1000);
                });

                pauseBtn.addEventListener('click', function() {
                    if (interval) { clearInterval(interval); interval = null; }
                    running = false;
                    startBtn.disabled = false;
                    pauseBtn.disabled = true;
                    startBtn.textContent = '开始';
                    setCustomInputsEnabled(true);
                });

                resetBtn.addEventListener('click', function() {
                    if (interval) { clearInterval(interval); interval = null; }
                    running = false;
                    startBtn.disabled = false;
                    pauseBtn.disabled = true;
                    startBtn.textContent = '开始';
                    setCustomInputsEnabled(true);
                    remaining = TIMES[mode];
                    render();
                });

                loadSettings();
                remaining = TIMES[mode];
                render();
            })();
        "#></script>
    }
}
