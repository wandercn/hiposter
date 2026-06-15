// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="INTRO.html"><strong aria-hidden="true">1.</strong> 序言：为什么选择 GPUI？</a></li><li class="chapter-item expanded "><a href="CHAPTER1_ARCHITECTURE.html"><strong aria-hidden="true">2.</strong> 第一章：项目架构与模块规划</a></li><li class="chapter-item expanded "><a href="CHAPTER2_LIFECYCLE.html"><strong aria-hidden="true">3.</strong> 第二章：应用入口与生命周期管理</a></li><li class="chapter-item expanded "><a href="CHAPTER3_UI_LAYOUT.html"><strong aria-hidden="true">4.</strong> 第三章：声明式 UI 与 Flexbox 布局</a></li><li class="chapter-item expanded "><a href="CHAPTER4_STATE_MANAGEMENT.html"><strong aria-hidden="true">5.</strong> 第四章：状态管理：Context、Entity 与模式</a></li><li class="chapter-item expanded "><a href="CHAPTER5_ACTION_SYSTEM.html"><strong aria-hidden="true">6.</strong> 第五章：交互进阶：Action 系统与快捷键</a></li><li class="chapter-item expanded "><a href="CHAPTER6_ASYNC.html"><strong aria-hidden="true">7.</strong> 第六章：异步并发：在 UI 应用中处理网络请求</a></li><li class="chapter-item expanded "><a href="CHAPTER7_PERFORMANCE.html"><strong aria-hidden="true">8.</strong> 第七章：性能之巅：脏标记模式与渲染优化</a></li><li class="chapter-item expanded "><a href="CHAPTER8_PERSISTENCE.html"><strong aria-hidden="true">9.</strong> 第八章：数据持久化：标准化存储与迁移</a></li><li class="chapter-item expanded "><a href="CHAPTER9_TESTING.html"><strong aria-hidden="true">10.</strong> 第九章：测试策略：逻辑与 UI 的平衡</a></li><li class="chapter-item expanded "><a href="CHAPTER10_BUILD.html"><strong aria-hidden="true">11.</strong> 第十章：跨平台构建与发布流水线</a></li><li class="chapter-item expanded affix "><li class="spacer"></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString();
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
