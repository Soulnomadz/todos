// static/js/todos.js
document.addEventListener('DOMContentLoaded', function() {
    const addTodoBtn = document.getElementById('addTodoBtn');
    const newTodoInputContainer = document.getElementById('newTodoInputContainer');
    const newTodoInput = document.getElementById('newTodoInput');
    const todoList = document.getElementById('todoList');
    const statusMessage = document.getElementById('status-message');
    const loading = document.getElementById('loading');
    const completedSeparator = document.getElementById('completedSeparator');
    
    // 视图控制按钮
    const showAllBtn = document.getElementById('showAllBtn');
    const hideCompletedBtn = document.getElementById('hideCompletedBtn');
    const showOnlyCompletedBtn = document.getElementById('showOnlyCompletedBtn');
    
    let isAddingNew = false;
    let currentView = 'all'; // 'all', 'active', 'completed'
    let currentlyEditing = null; // 当前正在编辑的todo ID
    
    // 初始化：将已完成事项移动到列表底部
    organizeTodosByCompletion();
    
    // 视图控制按钮事件
    showAllBtn.addEventListener('click', () => {
        setActiveView('all');
        updateView();
    });
    
    hideCompletedBtn.addEventListener('click', () => {
        setActiveView('active');
        updateView();
    });
    
    showOnlyCompletedBtn.addEventListener('click', () => {
        setActiveView('completed');
        updateView();
    });
    
    // 设置活动视图
    function setActiveView(view) {
        currentView = view;
        
        // 更新按钮状态
        [showAllBtn, hideCompletedBtn, showOnlyCompletedBtn].forEach(btn => {
            btn.classList.remove('active');
        });
        
        if (view === 'all') showAllBtn.classList.add('active');
        if (view === 'active') hideCompletedBtn.classList.add('active');
        if (view === 'completed') showOnlyCompletedBtn.classList.add('active');
    }
    
    // 将已完成事项移动到列表底部
    function organizeTodosByCompletion() {
        const todoItems = Array.from(todoList.querySelectorAll('.todo-item'));
        
        // 分离未完成和已完成的事项
        const activeItems = todoItems.filter(item => 
            item.getAttribute('data-completed') === 'false' || 
            !item.classList.contains('completed')
        );
        
        const completedItems = todoItems.filter(item => 
            item.getAttribute('data-completed') === 'true' || 
            item.classList.contains('completed')
        );
        
        // 清空列表
        todoList.innerHTML = '';
        
        // 先添加未完成事项
        activeItems.forEach(item => {
            todoList.appendChild(item);
        });
        
        // 如果有已完成事项，显示分隔线并添加它们
        if (completedItems.length > 0 && activeItems.length > 0) {
            completedSeparator.classList.add('show');
            
            // 添加分隔线
            todoList.appendChild(completedSeparator);
            
            // 添加已完成事项
            completedItems.forEach(item => {
                todoList.appendChild(item);
            });
        } else if (completedItems.length > 0) {
            // 只有已完成事项
            completedSeparator.classList.remove('show');
            completedItems.forEach(item => {
                todoList.appendChild(item);
            });
        } else {
            // 没有已完成事项
            completedSeparator.classList.remove('show');
        }
    }
    
    // 更新视图显示
    function updateView() {
        const todoItems = document.querySelectorAll('.todo-item');
        
        // 根据当前视图显示/隐藏项目
        todoItems.forEach(item => {
            const isCompleted = item.classList.contains('completed') || 
                               item.getAttribute('data-completed') === 'true';
            
            switch(currentView) {
                case 'all':
                    item.style.display = 'flex';
                    break;
                case 'active':
                    item.style.display = isCompleted ? 'none' : 'flex';
                    break;
                case 'completed':
                    item.style.display = isCompleted ? 'flex' : 'none';
                    break;
            }
        });
        
        // 更新已完成分隔线
        const completedItems = document.querySelectorAll('.todo-item.completed, [data-completed="true"]');
        const activeItems = document.querySelectorAll('.todo-item:not(.completed):not([data-completed="true"])');
        
        if (completedItems.length > 0 && activeItems.length > 0 && currentView === 'all') {
            completedSeparator.style.display = 'block';
        } else {
            completedSeparator.style.display = 'none';
        }
        
        // 更新统计
        updateStats();
    }
    
    // 新增按钮点击事件
    addTodoBtn.addEventListener('click', function() {
        if (!isAddingNew) {
            newTodoInputContainer.style.display = 'block';
            newTodoInput.focus();
            isAddingNew = true;
            
            this.innerHTML = '<span class="add-icon">×</span><span>取消新增</span>';
            this.style.backgroundColor = '#868e96';
        } else {
            newTodoInputContainer.style.display = 'none';
            newTodoInput.value = '';
            isAddingNew = false;
            
            this.innerHTML = '<span class="add-icon">+</span><span>新增待办</span>';
            this.style.backgroundColor = '#4dabf7';
        }
    });
    
    // 输入框键盘事件（新增待办）
    newTodoInput.addEventListener('keydown', function(e) {
        if (e.key === 'Enter') {
            const todoText = this.value.trim();
            
            if (todoText === '') {
                showStatusMessage('请输入待办事项内容', 'error');
                return;
            }
            
            loading.style.display = 'block';
            statusMessage.style.display = 'none';
            
            const requestData = {
                text: todoText
            };
            
            fetch('/todos', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(requestData)
            })
            .then(response => {
                loading.style.display = 'none';
                
                if (response.ok) {
                    showStatusMessage('待办事项添加成功！', 'success');
                    setTimeout(() => {
                        location.reload();
                    }, 1500);
                } else {
                    showStatusMessage('添加失败，请重试', 'error');
                }
            })
            .catch(error => {
                loading.style.display = 'none';
                showStatusMessage('网络错误，请检查连接', 'error');
                console.error('Error:', error);
            });
        }
        
        if (e.key === 'Escape') {
            newTodoInputContainer.style.display = 'none';
            newTodoInput.value = '';
            isAddingNew = false;
            
            addTodoBtn.innerHTML = '<span class="add-icon">+</span><span>新增待办</span>';
            addTodoBtn.style.backgroundColor = '#4dabf7';
        }
    });
    
    // 处理待办文本点击事件（开始编辑）
    todoList.addEventListener('click', function(e) {
        const todoTextContainer = e.target.closest('.todo-text-container');
        if (todoTextContainer && !currentlyEditing) {
            const todoId = todoTextContainer.getAttribute('data-id');
            startEditing(todoId);
        }
    });
    
    // 开始编辑待办事项
    function startEditing(todoId) {
        // 如果已经在编辑其他事项，先取消编辑
        if (currentlyEditing && currentlyEditing !== todoId) {
            cancelEditing(currentlyEditing);
        }
        
        const todoItem = document.querySelector(`.todo-item[data-id="${todoId}"]`);
        const todoTextElement = todoItem.querySelector('.todo-text');
        const editInput = todoItem.querySelector('.todo-edit-input');
        const editHint = todoItem.querySelector('.edit-hint');
        
        // 保存原始文本
        editInput.setAttribute('data-original', todoTextElement.textContent);
        
        // 切换到编辑模式
        todoItem.classList.add('editing');
        todoTextElement.style.display = 'none';
        editInput.style.display = 'block';
        editHint.classList.add('show');
        
        // 聚焦输入框并选中所有文本
        editInput.focus();
        editInput.select();
        
        currentlyEditing = todoId;
        
        // 添加键盘事件监听
        editInput.addEventListener('keydown', handleEditKeydown);
        // 添加失去焦点事件监听
        editInput.addEventListener('blur', handleEditBlur);
    }
    
    // 处理编辑输入框键盘事件
    function handleEditKeydown(e) {
        if (e.key === 'Enter') {
            e.preventDefault();
            saveEditing();
        } else if (e.key === 'Escape') {
            e.preventDefault();
            cancelEditing(currentlyEditing);
        }
    }
    
    // 处理编辑输入框失去焦点事件
    function handleEditBlur() {
        // 延迟执行，以便点击其他元素时有时间处理
        setTimeout(() => {
            if (currentlyEditing) {
                saveEditing();
            }
        }, 200);
    }
    
    // 保存编辑
    function saveEditing() {
        if (!currentlyEditing) return;
        
        const todoId = currentlyEditing;
        const todoItem = document.querySelector(`.todo-item[data-id="${todoId}"]`);
        const editInput = todoItem.querySelector('.todo-edit-input');
        const todoTextElement = todoItem.querySelector('.todo-text');
        const editHint = todoItem.querySelector('.edit-hint');
        
        const newText = editInput.value.trim();
        const originalText = editInput.getAttribute('data-original');
        
        // 如果文本没有改变，直接取消编辑
        if (newText === originalText) {
            cancelEditing(todoId);
            return;
        }
        
        // 验证文本
        if (newText === '') {
            showStatusMessage('待办事项内容不能为空', 'error');
            editInput.focus();
            return;
        }
        
        // 显示加载状态
        const loadingSpinner = todoItem.querySelector('.loading-spinner');
        if (loadingSpinner) loadingSpinner.style.display = 'inline-block';
        
        // 准备PUT请求数据
        const requestData = {
            id: parseInt(todoId),
            text: newText,
            completed: todoItem.classList.contains('completed')
        };
        
        // 发送PUT请求到后端
        fetch(`/todos/${todoId}`, {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(requestData)
        })
        .then(response => {
            if (loadingSpinner) loadingSpinner.style.display = 'none';
            
            if (response.ok) {
                // 更新显示文本
                todoTextElement.textContent = newText;
                
                // 退出编辑模式
                todoItem.classList.remove('editing');
                todoTextElement.style.display = 'inline';
                editInput.style.display = 'none';
                editHint.classList.remove('show');
                
                // 移除事件监听
                editInput.removeEventListener('keydown', handleEditKeydown);
                editInput.removeEventListener('blur', handleEditBlur);
                
                currentlyEditing = null;
                
                showStatusMessage('待办事项已更新', 'success');
            } else {
                showStatusMessage('更新失败，请重试', 'error');
                editInput.focus();
            }
        })
        .catch(error => {
            if (loadingSpinner) loadingSpinner.style.display = 'none';
            showStatusMessage('网络错误，请检查连接', 'error');
            editInput.focus();
            console.error('Error:', error);
        });
    }
    
    // 取消编辑
    function cancelEditing(todoId) {
        const todoItem = document.querySelector(`.todo-item[data-id="${todoId}"]`);
        if (!todoItem) return;
        
        const todoTextElement = todoItem.querySelector('.todo-text');
        const editInput = todoItem.querySelector('.todo-edit-input');
        const editHint = todoItem.querySelector('.edit-hint');
        
        // 恢复原始文本
        const originalText = editInput.getAttribute('data-original');
        editInput.value = originalText;
        
        // 退出编辑模式
        todoItem.classList.remove('editing');
        todoTextElement.style.display = 'inline';
        editInput.style.display = 'none';
        editHint.classList.remove('show');
        
        // 移除事件监听
        editInput.removeEventListener('keydown', handleEditKeydown);
        editInput.removeEventListener('blur', handleEditBlur);
        
        currentlyEditing = null;
    }
    
    // 处理复选框点击事件（事件委托）
    todoList.addEventListener('change', function(e) {
        if (e.target.classList.contains('todo-checkbox')) {
            const checkbox = e.target;
            const todoItem = checkbox.closest('.todo-item');
            const todoId = todoItem.getAttribute('data-id');
            const todoText = todoItem.querySelector('.todo-text').textContent;
            
            // 如果正在编辑，先保存
            if (currentlyEditing === todoId) {
                saveEditing();
                // 等待保存完成再处理复选框
                setTimeout(() => {
                    handleCheckboxChange(checkbox, todoItem, todoId, todoText);
                }, 100);
            } else {
                handleCheckboxChange(checkbox, todoItem, todoId, todoText);
            }
        }
    });
    
    function handleCheckboxChange(checkbox, todoItem, todoId, todoText) {
        // 如果取消勾选，不执行操作
        if (!checkbox.checked) {
            checkbox.checked = true; // 保持选中状态
            return;
        }
        
        loading.style.display = 'block';
        statusMessage.style.display = 'none';
        
        const requestData = {
            id: parseInt(todoId),
            text: todoText,
            completed: true
        };
        
        fetch(`/todos/${todoId}`, {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(requestData)
        })
        .then(response => {
            loading.style.display = 'none';
            
            if (response.ok) {
                // 标记为已完成
                todoItem.classList.add('completed');
                todoItem.setAttribute('data-completed', 'true');
                checkbox.checked = true;
                
                // 重新组织列表，将已完成事项移到底部
                organizeTodosByCompletion();
                
                // 更新视图
                updateView();
                showStatusMessage('任务已完成！', 'success');
            } else {
                checkbox.checked = false;
                showStatusMessage('更新失败，请重试', 'error');
            }
        })
        .catch(error => {
            loading.style.display = 'none';
            checkbox.checked = false;
            showStatusMessage('网络错误，请检查连接', 'error');
            console.error('Error:', error);
        });
    }
    
    // 处理删除按钮点击事件（事件委托）
    todoList.addEventListener('click', function(e) {
        if (e.target.classList.contains('delete-btn') || 
            e.target.closest('.delete-btn')) {
            
            const deleteBtn = e.target.classList.contains('delete-btn') ? 
                e.target : e.target.closest('.delete-btn');
            
            const todoId = deleteBtn.getAttribute('data-id');
            const todoItem = deleteBtn.closest('.todo-item');
            
            // 如果正在编辑，先取消编辑
            if (currentlyEditing === todoId) {
                cancelEditing(todoId);
            }
            
            if (!confirm('确定要删除这个待办事项吗？')) {
                return;
            }
            
            deleteBtn.disabled = true;
            const btnText = deleteBtn.querySelector('.btn-text');
            const spinner = deleteBtn.querySelector('.loading-spinner');
            
            if (btnText) btnText.style.display = 'none';
            if (spinner) spinner.style.display = 'inline-block';
            
            fetch(`/todos/${todoId}`, {
                method: 'DELETE',
                headers: {
                    'Content-Type': 'application/json',
                }
            })
            .then(response => {
                if (response.ok) {
                    showStatusMessage('任务删除成功！', 'success');
                    
                    // 平滑移除动画
                    todoItem.style.opacity = '0';
                    todoItem.style.transform = 'translateX(100px)';
                    todoItem.style.height = '0';
                    todoItem.style.margin = '0';
                    todoItem.style.padding = '0';
                    todoItem.style.overflow = 'hidden';
                    
                    setTimeout(() => {
                        todoItem.remove();
                        organizeTodosByCompletion();
                        updateView();
                    }, 300);
                } else {
                    deleteBtn.disabled = false;
                    if (btnText) btnText.style.display = 'inline';
                    if (spinner) spinner.style.display = 'none';
                    showStatusMessage('删除失败，请重试', 'error');
                }
            })
            .catch(error => {
                deleteBtn.disabled = false;
                if (btnText) btnText.style.display = 'inline';
                if (spinner) spinner.style.display = 'none';
                showStatusMessage('网络错误，请检查连接', 'error');
                console.error('Error:', error);
            });
        }
    });
    
    function showStatusMessage(message, type) {
        statusMessage.textContent = message;
        statusMessage.className = 'status-message';
        statusMessage.classList.add(`status-${type}`);
        statusMessage.style.display = 'block';
        
        setTimeout(() => {
            statusMessage.style.display = 'none';
        }, 3000);
    }
    
    function updateStats() {
        const todoItems = document.querySelectorAll('.todo-item');
        const completedItems = document.querySelectorAll('.todo-item.completed, [data-completed="true"]');
        const activeItems = todoItems.length - completedItems.length;
        
        document.getElementById('totalCount').textContent = todoItems.length;
        document.getElementById('completedCount').textContent = completedItems.length;
        document.getElementById('activeCount').textContent = activeItems;
    }
    
    // 初始更新
    updateView();
});
