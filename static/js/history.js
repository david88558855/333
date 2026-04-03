// 历史记录页面 JavaScript

document.addEventListener('DOMContentLoaded', () => {
    loadHistory();
});

// 加载历史记录
async function loadHistory() {
    const token = localStorage.getItem('token');
    
    if (!token) {
        document.getElementById('needLogin').style.display = 'block';
        return;
    }
    
    try {
        // TODO: 实现历史 API
        // const response = await fetch('/api/history', {
        //     headers: {
        //         'Authorization': `Bearer ${token}`
        //     }
        // });
        
        // 模拟数据
        await new Promise(resolve => setTimeout(resolve, 300));
        
        const mockHistory = []; // 空数组用于演示
        
        if (mockHistory.length > 0) {
            document.getElementById('clearAllSection').style.display = 'block';
            renderHistory(mockHistory);
        } else {
            document.getElementById('noHistory').style.display = 'block';
        }
    } catch (error) {
        console.error('Failed to load history:', error);
        document.getElementById('historyContent').innerHTML = 
            '<p style="text-align: center; padding: 40px; color: #e74c3c;">加载失败，请稍后重试</p>';
    }
}

// 渲染历史记录
function renderHistory(historyList) {
    const container = document.getElementById('historyContent');
    if (!container) return;
    
    container.innerHTML = historyList.map(item => `
        <div class="history-item">
            <img src="${item.poster || 'https://picsum.photos/120/180'}" 
                 alt="${escapeHtml(item.title)}" class="history-poster">
            <div class="history-info">
                <h3 class="history-title">${escapeHtml(item.title)}</h3>
                <p class="history-meta">
                    ${item.sourceName || '未知源'} · 
                    ${item.episode ? `第${item.episode}集` : ''} · 
                    ${formatTime(item.updatedAt)}
                </p>
                <div class="progress-bar">
                    <div class="progress-fill" style="width: ${item.progressPercent || 0}%"></div>
                </div>
                <p class="progress-text">
                    已观看 ${formatDuration(item.progress)} / 总时长 ${formatDuration(item.duration)}
                </p>
                <div class="history-actions">
                    <button class="btn-continue" onclick="continueWatch(${item.id})">▶️ 继续观看</button>
                    <button class="btn-delete" onclick="deleteHistory(${item.id})">删除</button>
                </div>
            </div>
        </div>
    `).join('');
}

// 继续观看
function continueWatch(id) {
    window.location.href = `/play?id=${id}`;
}

// 删除单条历史
async function deleteHistory(id) {
    const token = localStorage.getItem('token');
    if (!token) return;
    
    try {
        // TODO: 实现删除历史 API
        // await fetch(`/api/history/${id}`, {
        //     method: 'DELETE',
        //     headers: {
        //         'Authorization': `Bearer ${token}`
        //     }
        // });
        
        // 重新加载
        await loadHistory();
    } catch (error) {
        console.error('Failed to delete history:', error);
    }
}

// 清空全部历史
async function clearAllHistory() {
    const token = localStorage.getItem('token');
    if (!token) return;
    
    if (!confirm('确定要清空所有观看历史吗？此操作不可恢复。')) return;
    
    try {
        // TODO: 实现清空历史 API
        // await fetch('/api/history/clear', {
        //     method: 'DELETE',
        //     headers: {
        //         'Authorization': `Bearer ${token}`
        //     }
        // });
        
        // 重新加载
        await loadHistory();
    } catch (error) {
        console.error('Failed to clear history:', error);
    }
}

// 格式化时间
function formatTime(dateString) {
    const date = new Date(dateString);
    const now = new Date();
    const diff = now - date;
    
    const minutes = Math.floor(diff / 60000);
    const hours = Math.floor(diff / 3600000);
    const days = Math.floor(diff / 86400000);
    
    if (minutes < 1) return '刚刚';
    if (minutes < 60) return `${minutes}分钟前`;
    if (hours < 24) return `${hours}小时前`;
    if (days < 7) return `${days}天前`;
    
    return date.toLocaleDateString('zh-CN');
}

// 格式化时长
function formatDuration(seconds) {
    if (!seconds) return '00:00';
    
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = seconds % 60;
    
    if (h > 0) {
        return `${h}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
    }
    return `${m}:${s.toString().padStart(2, '0')}`;
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
