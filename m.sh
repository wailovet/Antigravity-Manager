#!/bin/bash

# 设置上游仓库并合并上游更新的脚本

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 打印带颜色的信息
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查是否在git仓库中
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    print_error "当前目录不是一个git仓库！"
    exit 1
fi

# 获取当前分支名称
current_branch=$(git branch --show-current)
print_info "当前分支: $current_branch"

# 检查是否提供了上游仓库URL
if [ $# -eq 0 ]; then
    print_warning "未提供上游仓库URL，将尝试使用已配置的upstream或当前origin作为上游"
    
    # 检查是否已经设置了upstream远程
    if git remote get-url upstream > /dev/null 2>&1; then
        upstream_url=$(git remote get-url upstream)
        print_info "使用已配置的upstream远程仓库URL: $upstream_url"
    else
        # 获取当前的远程仓库URL作为upstream
        origin_url=$(git remote get-url origin 2>/dev/null)
        if [ -z "$origin_url" ]; then
            print_error "无法获取origin远程仓库URL，也未配置upstream"
            exit 1
        fi
        print_info "当前origin仓库URL: $origin_url"
        
        # 询问用户是否设置origin作为upstream
        read -p "是否要将origin设置为upstream? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            upstream_url=$origin_url
        else
            read -p "请输入上游仓库URL: " upstream_url
            if [ -z "$upstream_url" ]; then
                print_error "上游仓库URL不能为空"
                exit 1
            fi
        fi
    fi
else
    upstream_url=$1
fi

# 检查上游仓库URL是否为空
if [ -z "$upstream_url" ]; then
    print_error "上游仓库URL不能为空"
    exit 1
fi

# 检查是否已经设置了upstream远程，如果没有则添加
if git remote get-url upstream > /dev/null 2>&1; then
    current_upstream_url=$(git remote get-url upstream)
    print_info "当前upstream远程仓库URL: $current_upstream_url"
    
    # 如果upstream URL不同，则更新
    if [ "$current_upstream_url" != "$upstream_url" ]; then
        read -p "是否要更新upstream远程仓库URL? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            git remote set-url upstream "$upstream_url"
            print_success "已更新upstream远程仓库URL为: $upstream_url"
        else
            print_info "继续使用当前upstream远程仓库URL"
            upstream_url=$current_upstream_url
        fi
    else
        print_info "upstream远程仓库URL已正确配置"
    fi
else
    # 添加upstream远程仓库
    git remote add upstream "$upstream_url"
    print_success "已添加upstream远程仓库: $upstream_url"
fi

# 获取远程仓库的更新
print_info "正在获取upstream仓库的更新..."
if git fetch upstream; then
    print_success "成功获取upstream仓库的更新"
else
    print_error "获取upstream仓库更新失败"
    exit 1
fi

# 显示upstream仓库的分支信息
print_info "upstream仓库的分支信息:"
git remote show upstream

# 询问是否要合并upstream的更新
read -p "是否要将upstream的更新合并到当前分支? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    # 确保当前分支是最新的
    git fetch origin
    
    # 尝试合并upstream的默认分支
    upstream_default_branch=$(git remote show upstream | grep "HEAD branch" | cut -d ":" -f 2 | xargs)
    if [ -z "$upstream_default_branch" ]; then
        # 如果无法获取默认分支，则使用main或master
        if git ls-remote --heads upstream | grep -q "refs/heads/main"; then
            upstream_default_branch="main"
        elif git ls-remote --heads upstream | grep -q "refs/heads/master"; then
            upstream_default_branch="master"
        else
            print_error "无法确定upstream仓库的默认分支"
            exit 1
        fi
    fi
    
    print_info "将要合并upstream/$upstream_default_branch到当前分支($current_branch)"
    
    # 检查是否有未提交的更改
    if ! git diff-index --quiet HEAD --; then
        print_warning "当前分支有未提交的更改，建议先提交或暂存更改"
        read -p "是否继续合并? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_info "已取消合并操作"
            exit 0
        fi
    fi
    
    # 执行合并操作
    if git merge "upstream/$upstream_default_branch"; then
        print_success "成功将upstream/$upstream_default_branch合并到当前分支($current_branch)"
        
        # 检查是否有合并冲突
        if git status --porcelain | grep -q "^UU\|^AA\|^DD"; then
            print_warning "存在合并冲突，请手动解决冲突后提交更改"
        else
            print_info "合并完成，没有冲突"
        fi
    else
        print_error "合并失败，请检查是否有冲突需要解决"
        print_info "您可以使用 'git status' 查看当前状态，使用 'git merge --abort' 取消合并"
        exit 1
    fi
else
    print_info "跳过合并操作"
fi

# 显示当前状态
print_info "当前仓库状态:"
git status --short

print_success "脚本执行完成"