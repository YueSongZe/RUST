const API_BASE = 'http://localhost:3000';

let token = localStorage.getItem('token');

document.addEventListener('DOMContentLoaded', () => {
    if (token) {
        showApp();
    } else {
        showLogin();
    }

    // 登录表单
    document.getElementById('login-form').addEventListener('submit', async (e) => {
        e.preventDefault();
        const account = document.getElementById('account').value;
        const password = document.getElementById('password').value;

        try {
            const response = await fetch(`${API_BASE}/api/login`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ account, password })
            });

            if (response.ok) {
                const data = await response.json();
                token = data.token;
                localStorage.setItem('token', token);
                showApp();
            } else {
                document.getElementById('login-message').textContent = '登录失败';
            }
        } catch (error) {
            document.getElementById('login-message').textContent = '网络错误';
        }
    });

    // 导航按钮
    document.getElementById('users-btn').addEventListener('click', () => loadUsers());
    document.getElementById('devices-btn').addEventListener('click', () => loadDevices());
    document.getElementById('keypacks-btn').addEventListener('click', () => loadKeypacks());
    document.getElementById('status-btn').addEventListener('click', () => loadStatus());
    document.getElementById('logout-btn').addEventListener('click', () => logout());
});

function showLogin() {
    document.getElementById('login-container').style.display = 'block';
    document.getElementById('app').style.display = 'none';
}

function showApp() {
    document.getElementById('login-container').style.display = 'none';
    document.getElementById('app').style.display = 'block';
    loadUsers(); // 默认加载用户
}

async function apiCall(endpoint, options = {}) {
    const headers = { ...options.headers };
    if (token) {
        headers['Authorization'] = token;
    }

    const response = await fetch(`${API_BASE}${endpoint}`, {
        ...options,
        headers
    });

    if (response.status === 401) {
        logout();
        throw new Error('Unauthorized');
    }

    return response;
}

async function loadUsers() {
    try {
        const response = await apiCall('/api/users');
        const users = await response.json();
        displayUsers(users);
    } catch (error) {
        console.error('加载用户失败:', error);
    }
}

function displayUsers(users) {
    const content = document.getElementById('content');
    content.innerHTML = `
        <h2>用户管理</h2>
        <button id="create-user-btn">创建用户</button>
        <table>
            <thead>
                <tr>
                    <th>ID</th>
                    <th>账号</th>
                    <th>角色</th>
                    <th>创建时间</th>
                </tr>
            </thead>
            <tbody>
                ${users.map(user => `
                    <tr>
                        <td>${user.id}</td>
                        <td>${user.account}</td>
                        <td>${user.role}</td>
                        <td>${user.created_at}</td>
                    </tr>
                `).join('')}
            </tbody>
        </table>
    `;

    document.getElementById('create-user-btn').addEventListener('click', () => showCreateUserForm());
}

function showCreateUserForm() {
    const content = document.getElementById('content');
    content.innerHTML = `
        <h2>创建用户</h2>
        <form id="create-user-form">
            <input type="text" id="new-account" placeholder="账号" required>
            <input type="text" id="new-role" placeholder="角色" required>
            <input type="password" id="new-password" placeholder="密码" required>
            <button type="submit">创建</button>
            <button type="button" id="cancel-btn">取消</button>
        </form>
    `;

    document.getElementById('create-user-form').addEventListener('submit', async (e) => {
        e.preventDefault();
        const account = document.getElementById('new-account').value;
        const role = document.getElementById('new-role').value;
        const password = document.getElementById('new-password').value;

        try {
            const response = await apiCall('/api/users', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ account, role, password })
            });

            if (response.ok) {
                loadUsers();
            } else {
                alert('创建用户失败');
            }
        } catch (error) {
            console.error('创建用户失败:', error);
        }
    });

    document.getElementById('cancel-btn').addEventListener('click', () => loadUsers());
}

async function loadDevices() {
    try {
        const response = await apiCall('/api/devices');
        const devices = await response.json();
        displayDevices(devices);
    } catch (error) {
        console.error('加载设备失败:', error);
    }
}

function displayDevices(devices) {
    const content = document.getElementById('content');
    content.innerHTML = `
        <h2>设备管理</h2>
        <table>
            <thead>
                <tr>
                    <th>ID</th>
                    <th>名称</th>
                    <th>状态</th>
                    <th>创建时间</th>
                </tr>
            </thead>
            <tbody>
                ${devices.map(device => `
                    <tr>
                        <td>${device.id}</td>
                        <td>${device.name}</td>
                        <td>${device.status}</td>
                        <td>${device.created_at}</td>
                    </tr>
                `).join('')}
            </tbody>
        </table>
    `;
}

async function loadKeypacks() {
    try {
        const response = await apiCall('/api/keypacks');
        const keypacks = await response.json();
        displayKeypacks(keypacks);
    } catch (error) {
        console.error('加载密钥包失败:', error);
    }
}

function displayKeypacks(keypacks) {
    const content = document.getElementById('content');
    content.innerHTML = `
        <h2>密钥包管理</h2>
        <table>
            <thead>
                <tr>
                    <th>ID</th>
                    <th>名称</th>
                    <th>版本</th>
                    <th>创建时间</th>
                </tr>
            </thead>
            <tbody>
                ${keypacks.map(keypack => `
                    <tr>
                        <td>${keypack.id}</td>
                        <td>${keypack.name}</td>
                        <td>${keypack.version}</td>
                        <td>${keypack.created_at}</td>
                    </tr>
                `).join('')}
            </tbody>
        </table>
    `;
}

async function loadStatus() {
    try {
        const response = await apiCall('/api/status');
        const status = await response.json();
        displayStatus(status);
    } catch (error) {
        console.error('加载状态失败:', error);
    }
}

function displayStatus(status) {
    const content = document.getElementById('content');
    content.innerHTML = `
        <h2>系统状态</h2>
        <pre>${JSON.stringify(status, null, 2)}</pre>
    `;
}

function logout() {
    token = null;
    localStorage.removeItem('token');
    showLogin();
}