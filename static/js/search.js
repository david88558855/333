// 搜索功能 JavaScript

let currentQuery = '';
let activeSourceId = null;

document.addEventListener('DOMContentLoaded', () => {
    // 从 URL 获取查询参数
    const urlParams = new URLSearchParams(window.location.search);
    currentQuery = urlParams.get('q') || '';
    
    if (currentQuery) {
        document.getElementById('searchInput').value = currentQuery;
        performSearch(currentQuery);
    }
    
    loadSources();
});

// 搜索功能
async function search() {
    const query = document.getElementById('searchInput').value.trim();
    if (!query) return;
    
    window.location.href = `/search?q=${encodeURIComponent(query)}`;
}

// 执行搜索
async function performSearch(query, sourceId = null) {
    const resultsContainer = document.getElementById('searchResults');
    const noResults = document.getElementById('noResults');
    
    if (!resultsContainer) return;
    
    resultsContainer.innerHTML = '<p style="text-align: center; padding: 40px;">搜索中...</p>';
    noResults.style.display = 'none';
    
    try {
        // TODO: 实现搜索 API
        // const response = await fetch(`/api/search?q=${encodeURIComponent(query)}&source=${sourceId || ''}`);
        
        // 模拟搜索结果
        await new Promise(resolve => setTimeout(resolve, 500));
        
        const mockResults = [
            { id: 1, title: `${query} - 电影版`, year: 2024, type: '电影', poster: 'https://picsum.photos/200/300' },
            { id: 2, title: `${query} - 剧集版`, year: 2023, type: '剧集', poster: 'https://picsum.photos/200/301' },
            { id: 3, title: `${query} 特别篇`, year: 2024, type: '特别篇', poster: 'https://picsum.photos/200/302' },
        ];
        
        if (mockResults.length > 0) {
            renderSearchResults(mockResults);
            noResults.style.display = 'none';
        } else {
            resultsContainer.innerHTML = '';
            noResults.style.display = 'block';
        }
    } catch (error) {
        console.error('Search error:', error);
        resultsContainer.innerHTML = '<p style="text-align: center; padding: 40px; color: #e74c3c;">搜索失败，请稍后重试</p>';
    }
}

// 渲染搜索结果
function renderSearchResults(results) {
    const container = document.getElementById('searchResults');
    if (!container) return;
    
    container.innerHTML = results.map(video => `
        <div class="video-card" onclick="playVideo(${video.id})">
            <img src="${video.poster}" alt="${escapeHtml(video.title)}" class="video-poster">
            <div class="video-info">
                <h3 class="video-title">${escapeHtml(video.title)}</h3>
                <p class="video-meta">${video.year} · ${video.type}</p>
            </div>
        </div>
    `).join('');
}

// 加载播放源
async function loadSources() {
    try {
        const response = await fetch('/api/sources');
        const data = await response.json();
        
        if (data.success) {
            renderSourceTabs(data.data);
        }
    } catch (error) {
        console.error('Failed to load sources:', error);
    }
}

// 渲染播放源标签
function renderSourceTabs(sources) {
    const container = document.getElementById('sourceTabs');
    if (!container) return;
    
    container.innerHTML = `
        <button class="source-tab ${activeSourceId === null ? 'active' : ''}" 
                onclick="filterBySource(null)">
            全部
        </button>
        ${sources.filter(s => s.is_active).map(source => `
            <button class="source-tab ${source.id == activeSourceId ? 'active' : ''}" 
                    onclick="filterBySource(${source.id})">
                ${escapeHtml(source.name)}
            </button>
        `).join('')}
    `;
}

// 按播放源过滤
function filterBySource(sourceId) {
    activeSourceId = sourceId;
    if (currentQuery) {
        performSearch(currentQuery, sourceId);
    }
}

// 播放视频
function playVideo(videoId) {
    window.location.href = `/play?id=${videoId}`;
}

// HTML 转义
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// 搜索回车键支持
const searchInput = document.getElementById('searchInput');
if (searchInput) {
    searchInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
            search();
        }
    });
}
