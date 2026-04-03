// 收藏页面 JavaScript

document.addEventListener('DOMContentLoaded', () => {
    loadFavorites();
});

// 加载收藏列表
async function loadFavorites() {
    const token = localStorage.getItem('token');
    
    if (!token) {
        document.getElementById('needLogin').style.display = 'block';
        return;
    }
    
    try {
        // TODO: 实现收藏 API
        // const response = await fetch('/api/favorites', {
        //     headers: {
        //         'Authorization': `Bearer ${token}`
        //     }
        // });
        
        // 模拟数据
        await new Promise(resolve => setTimeout(resolve, 300));
        
        const mockFavorites = []; // 空数组用于演示
        
        if (mockFavorites.length > 0) {
            renderFavorites(mockFavorites);
        } else {
            document.getElementById('noFavorites').style.display = 'block';
        }
    } catch (error) {
        console.error('Failed to load favorites:', error);
        document.getElementById('favoritesContent').innerHTML = 
            '<p style="text-align: center; padding: 40px; color: #e74c3c;">加载失败，请稍后重试</p>';
    }
}

// 渲染收藏列表
function renderFavorites(favorites) {
    const container = document.getElementById('favoritesContent');
    if (!container) return;
    
    container.innerHTML = `
        <div class="video-grid">
            ${favorites.map(fav => `
                <div class="video-card" onclick="playVideo(${fav.id})">
                    <img src="${fav.poster || 'https://picsum.photos/200/300'}" 
                         alt="${escapeHtml(fav.title)}" class="video-poster">
                    <div class="video-info">
                        <h3 class="video-title">${escapeHtml(fav.title)}</h3>
                        <p class="video-meta">${fav.year || ''} · ${fav.type || ''}</p>
                    </div>
                </div>
            `).join('')}
        </div>
    `;
}

// 播放视频
function playVideo(videoId) {
    window.location.href = `/play?id=${videoId}`;
}

// 删除收藏
async function removeFavorite(id) {
    const token = localStorage.getItem('token');
    if (!token) return;
    
    if (!confirm('确定要取消收藏吗？')) return;
    
    try {
        // TODO: 实现删除收藏 API
        // await fetch(`/api/favorites/${id}`, {
        //     method: 'DELETE',
        //     headers: {
        //         'Authorization': `Bearer ${token}`
        //     }
        // });
        
        // 重新加载
        await loadFavorites();
    } catch (error) {
        console.error('Failed to remove favorite:', error);
    }
}

// HTML 转义
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// 退出登录
document.getElementById('logoutBtn')?.addEventListener('click', () => {
    localStorage.removeItem('token');
    localStorage.removeItem('user');
    window.location.href = '/';
});
