// 管理面板 JavaScript

let currentEditingId = null;

// 页面加载时获取视频源列表
document.addEventListener('DOMContentLoaded', async () => {
    await loadSources();
    await loadRegistrationSetting();
});

// 加载视频源列表
async function loadSources() {
    try {
        const response = await fetch('/api/sources');
        const data = await response.json();
        
        if (data.success) {
            renderSourcesTable(data.data);
        }
    } catch (error) {
        console.error('Failed to load sources:', error);
    }
}

// 渲染视频源表格
function renderSourcesTable(sources) {
    const tbody = document.getElementById('sourcesTableBody');
    if (!tbody) return;
    
    tbody.innerHTML = sources.map(source => `
        <tr>
            <td>${escapeHtml(source.name)}</td>
            <td>${escapeHtml(source.url)}</td>
            <td>${source.api_type}</td>
            <td>${source.is_active ? '启用' : '禁用'}</td>
            <td>${source.sort_order}</td>
            <td>
                <button class="btn-primary" onclick="editSource(${source.id})">编辑</button>
                <button class="btn-danger" onclick="deleteSource(${source.id})">删除</button>
            </td>
        </tr>
    `).join('');
}

// 加载注册设置
async function loadRegistrationSetting() {
    try {
        // TODO: 添加 API 端点获取注册设置
        const toggle = document.getElementById('enableRegisterToggle');
        if (toggle) {
            // 默认关闭，实际应从 API 获取
            toggle.checked = false;
            
            toggle.addEventListener('change', async () => {
                await updateRegistrationSetting(toggle.checked);
            });
        }
    } catch (error) {
        console.error('Failed to load registration setting:', error);
    }
}

// 更新注册设置
async function updateRegistrationSetting(enable) {
    try {
        const response = await fetch('/api/settings/register', {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ enable }),
        });
        
        const data = await response.json();
        
        if (!response.ok || !data.success) {
            alert('设置失败：' + (data.message || '未知错误'));
            // 恢复开关状态
            document.getElementById('enableRegisterToggle').checked = !enable;
        }
    } catch (error) {
        console.error('Failed to update registration setting:', error);
        alert('网络错误，请稍后重试');
        document.getElementById('enableRegisterToggle').checked = !enable;
    }
}

// 显示添加视频源模态框
function showAddSourceModal() {
    currentEditingId = null;
    document.getElementById('modalTitle').textContent = '添加视频源';
    document.getElementById('sourceForm').reset();
    document.getElementById('sourceModal').style.display = 'block';
}

// 关闭模态框
function closeSourceModal() {
    document.getElementById('sourceModal').style.display = 'none';
}

// 编辑视频源
async function editSource(id) {
    try {
        const response = await fetch(`/api/sources/${id}`);
        const data = await response.json();
        
        if (data.success) {
            const source = data.data;
            currentEditingId = id;
            
            document.getElementById('modalTitle').textContent = '编辑视频源';
            document.getElementById('sourceId').value = source.id;
            document.getElementById('sourceName').value = source.name;
            document.getElementById('sourceUrl').value = source.url;
            document.getElementById('sourceType').value = source.api_type;
            document.getElementById('sourceActive').value = source.is_active.toString();
            document.getElementById('sourceOrder').value = source.sort_order;
            
            document.getElementById('sourceModal').style.display = 'block';
        }
    } catch (error) {
        console.error('Failed to load source:', error);
        alert('加载失败');
    }
}

// 保存视频源
const sourceForm = document.getElementById('sourceForm');
if (sourceForm) {
    sourceForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        
        const sourceData = {
            name: document.getElementById('sourceName').value,
            url: document.getElementById('sourceUrl').value,
            api_type: document.getElementById('sourceType').value,
            is_active: document.getElementById('sourceActive').value === 'true',
            sort_order: parseInt(document.getElementById('sourceOrder').value),
        };
        
        try {
            let response;
            if (currentEditingId) {
                // 更新
                response = await fetch(`/api/sources/${currentEditingId}`, {
                    method: 'PUT',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify(sourceData),
                });
            } else {
                // 新增
                response = await fetch('/api/sources', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify(sourceData),
                });
            }
            
            const data = await response.json();
            
            if (response.ok && data.success) {
                closeSourceModal();
                await loadSources();
            } else {
                alert('保存失败：' + (data.message || '未知错误'));
            }
        } catch (error) {
            console.error('Failed to save source:', error);
            alert('网络错误，请稍后重试');
        }
    });
}

// 删除视频源
async function deleteSource(id) {
    if (!confirm('确定要删除这个视频源吗？')) return;
    
    try {
        const response = await fetch(`/api/sources/${id}`, {
            method: 'DELETE',
        });
        
        const data = await response.json();
        
        if (response.ok && data.success) {
            await loadSources();
        } else {
            alert('删除失败：' + (data.message || '未知错误'));
        }
    } catch (error) {
        console.error('Failed to delete source:', error);
        alert('网络错误，请稍后重试');
    }
}

// HTML 转义防止 XSS
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// 点击模态框外部关闭
window.onclick = function(event) {
    const modal = document.getElementById('sourceModal');
    if (event.target === modal) {
        closeSourceModal();
    }
}
