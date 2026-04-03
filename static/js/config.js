// TVBox 配置页面 JavaScript

let currentFormat = 'json';
let configBaseUrl = '';

document.addEventListener('DOMContentLoaded', () => {
    // 获取当前域名
    const protocol = window.location.protocol;
    const host = window.location.host;
    configBaseUrl = `${protocol}//${host}/api/tvbox`;
    
    updateConfigUrl();
});

// 切换格式
function switchFormat(format) {
    currentFormat = format;
    
    // 更新按钮状态
    document.querySelectorAll('.format-btn').forEach(btn => {
        btn.classList.toggle('active', btn.textContent.toLowerCase().includes(format));
    });
    
    updateConfigUrl();
}

// 更新配置 URL 显示
function updateConfigUrl() {
    const urlElement = document.getElementById('configUrl');
    if (urlElement) {
        urlElement.textContent = `${configBaseUrl}?format=${currentFormat}`;
    }
}

// 复制链接
async function copyConfigUrl() {
    const url = `${configBaseUrl}?format=${currentFormat}`;
    
    try {
        await navigator.clipboard.writeText(url);
        showSuccessToast('复制成功！');
    } catch (error) {
        console.error('Failed to copy:', error);
        
        // 降级方案
        const textArea = document.createElement('textarea');
        textArea.value = url;
        textArea.style.position = 'fixed';
        textArea.style.left = '-9999px';
        document.body.appendChild(textArea);
        textArea.select();
        
        try {
            document.execCommand('copy');
            showSuccessToast('复制成功！');
        } catch (err) {
            showSuccessToast('复制失败，请手动复制');
        }
        
        document.body.removeChild(textArea);
    }
}

// 新窗口打开
function openNewTab() {
    const url = `${configBaseUrl}?format=${currentFormat}`;
    window.open(url, '_blank');
}

// 显示成功提示
function showSuccessToast(message) {
    const toast = document.getElementById('successToast');
    if (toast) {
        toast.textContent = `✅ ${message}`;
        toast.style.display = 'block';
        
        setTimeout(() => {
            toast.style.display = 'none';
        }, 2000);
    }
}

// 退出登录
document.getElementById('logoutBtn')?.addEventListener('click', () => {
    localStorage.removeItem('token');
    localStorage.removeItem('user');
    window.location.href = '/';
});
