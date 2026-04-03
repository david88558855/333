// 主应用 JavaScript

// 页面加载时检查登录状态并加载内容
document.addEventListener('DOMContentLoaded', () => {
    checkLoginStatus();
    loadVideos();
});

// 检查登录状态
function checkLoginStatus() {
    const token = localStorage.getItem('token');
    const user = localStorage.getItem('user');
    
    if (token && user) {
        const userData = JSON.parse(user);
        updateUserInfo(userData);
    }
}

// 更新用户信息显示
function updateUserInfo(user) {
    // TODO: 更新 UI 显示用户信息
    console.log('Logged in as:', user.username);
}

// 搜索功能
async function search() {
    const query = document.getElementById('searchInput').value.trim();
    if (!query) return;
    
    try {
        // TODO: 实现搜索 API
        console.log('Searching for:', query);
        window.location.href = `/search?q=${encodeURIComponent(query)}`;
    } catch (error) {
        console.error('Search error:', error);
    }
}

// 加载视频列表
async function loadVideos() {
    try {
        // TODO: 实现视频列表 API
        const doubanGrid = document.getElementById('doubanGrid');
        const recentGrid = document.getElementById('recentGrid');
        
        if (doubanGrid) {
            doubanGrid.innerHTML = `
                <div class="video-card">
                    <img src="https://picsum.photos/200/300" alt="Video" class="video-poster">
                    <div class="video-info">
                        <h3 class="video-title">示例视频 1</h3>
                        <p class="video-meta">2024 · 电影</p>
                    </div>
                </div>
                <div class="video-card">
                    <img src="https://picsum.photos/200/301" alt="Video" class="video-poster">
                    <div class="video-info">
                        <h3 class="video-title">示例视频 2</h3>
                        <p class="video-meta">2024 · 剧集</p>
                    </div>
                </div>
            `;
        }
        
        if (recentGrid) {
            recentGrid.innerHTML = `
                <div class="video-card">
                    <img src="https://picsum.photos/200/302" alt="Video" class="video-poster">
                    <div class="video-info">
                        <h3 class="video-title">最近更新 1</h3>
                        <p class="video-meta">更新至 第 10 集</p>
                    </div>
                </div>
                <div class="video-card">
                    <img src="https://picsum.photos/200/303" alt="Video" class="video-poster">
                    <div class="video-info">
                        <h3 class="video-title">最近更新 2</h3>
                        <p class="video-meta">更新至 第 5 集</p>
                    </div>
                </div>
            `;
        }
    } catch (error) {
        console.error('Failed to load videos:', error);
    }
}

// 播放回车键搜索
const searchInput = document.getElementById('searchInput');
if (searchInput) {
    searchInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
            search();
        }
    });
}
