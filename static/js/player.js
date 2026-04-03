// 播放器相关 JavaScript

let currentVideoId = null;
let currentSourceId = null;
let favorites = [];

document.addEventListener('DOMContentLoaded', () => {
    // 从 URL 获取视频 ID
    const urlParams = new URLSearchParams(window.location.search);
    currentVideoId = urlParams.get('id');
    currentSourceId = urlParams.get('source');
    
    if (currentVideoId) {
        loadVideoInfo(currentVideoId, currentSourceId);
        loadSources();
        loadFavorites();
    }
});

// 加载视频信息
async function loadVideoInfo(videoId, sourceId) {
    try {
        // TODO: 实现视频详情 API
        console.log('Loading video:', videoId, 'from source:', sourceId);
        
        // 模拟数据
        document.title = '示例视频 - KatelyaTV';
        document.getElementById('videoDescription').textContent = '这是一个示例视频简介...';
        
        // 加载剧集列表
        loadEpisodes();
    } catch (error) {
        console.error('Failed to load video info:', error);
    }
}

// 加载播放源
async function loadSources() {
    try {
        const response = await fetch('/api/sources');
        const data = await response.json();
        
        if (data.success) {
            renderSourceButtons(data.data);
        }
    } catch (error) {
        console.error('Failed to load sources:', error);
    }
}

// 渲染播放源按钮
function renderSourceButtons(sources) {
    const container = document.getElementById('sourceButtons');
    if (!container) return;
    
    container.innerHTML = sources
        .filter(s => s.is_active)
        .map(source => `
            <button class="source-btn ${source.id == currentSourceId ? 'active' : ''}" 
                    onclick="switchSource(${source.id})">
                ${escapeHtml(source.name)}
            </button>
        `).join('');
}

// 切换播放源
function switchSource(sourceId) {
    const url = new URL(window.location);
    url.searchParams.set('source', sourceId);
    window.location.href = url.toString();
}

// 加载剧集列表
function loadEpisodes() {
    const container = document.getElementById('episodeList');
    if (!container) return;
    
    // 模拟剧集数据
    const episodes = Array.from({length: 20}, (_, i) => i + 1);
    
    container.innerHTML = episodes.map((ep, index) => `
        <button class="episode-btn ${index === 0 ? 'active' : ''}" 
                onclick="playEpisode(${ep})">
            ${ep}
        </button>
    `).join('');
}

// 播放剧集
function playEpisode(episode) {
    // TODO: 实现播放逻辑
    console.log('Playing episode:', episode);
    
    // 更新活动状态
    document.querySelectorAll('.episode-btn').forEach((btn, index) => {
        btn.classList.toggle('active', index + 1 === episode);
    });
}

// 加载收藏列表
function loadFavorites() {
    const token = localStorage.getItem('token');
    if (!token) return;
    
    // TODO: 实现收藏 API
    console.log('Loading favorites...');
}

// 切换收藏状态
async function toggleFavorite() {
    const token = localStorage.getItem('token');
    if (!token) {
        alert('请先登录');
        window.location.href = '/login';
        return;
    }
    
    try {
        // TODO: 实现收藏 API
        console.log('Toggling favorite for video:', currentVideoId);
        alert('收藏功能开发中...');
    } catch (error) {
        console.error('Failed to toggle favorite:', error);
    }
}

// 报错
function reportIssue() {
    const issue = prompt('请描述您遇到的问题：');
    if (issue) {
        // TODO: 实现报错 API
        console.log('Reported issue:', issue);
        alert('感谢您的反馈！');
    }
}

// HTML 转义
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}
