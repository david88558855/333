#!/bin/bash
# KatelyaTV Rust 构建脚本
# 用于在 Linux 环境下编译 musl 静态二进制文件

set -e

echo "🦀 KatelyaTV Rust 构建脚本"
echo "=========================="

# 检查 Rust 是否安装
if ! command -v cargo &> /dev/null; then
    echo "❌ 错误：未找到 Rust/Cargo，请先安装 Rust"
    echo "   访问 https://rustup.rs/ 获取安装说明"
    exit 1
fi

# 检查 musl 工具链
if ! command -v musl-gcc &> /dev/null; then
    echo "⚠️  警告：未找到 musl-gcc"
    echo "   请安装 musl 工具链:"
    echo "   Ubuntu/Debian: sudo apt-get install musl-tools"
    echo "   CentOS/RHEL: sudo yum install musl-devel"
    exit 1
fi

# 添加 musl 目标（如果尚未添加）
echo "📦 检查 Rust 目标..."
if ! rustup target list --installed | grep -q "x86_64-unknown-linux-musl"; then
    echo "   添加 x86_64-unknown-linux-musl 目标..."
    rustup target add x86_64-unknown-linux-musl
fi

# 清理之前的构建
echo "🧹 清理之前的构建..."
cargo clean

# 编译 release 版本
echo "🔨 编译 release 版本..."
export RUSTFLAGS="-C target-feature=+crt-static"
cargo build --release --target x86_64-unknown-linux-musl

# 验证二进制文件
echo "✅ 验证二进制文件..."
BINARY_PATH="target/x86_64-unknown-linux-musl/release/katelyatv-rust"

if [ -f "$BINARY_PATH" ]; then
    echo "📊 二进制文件信息:"
    file "$BINARY_PATH"
    
    echo ""
    echo "📦 文件大小:"
    ls -lh "$BINARY_PATH" | awk '{print $5}'
    
    echo ""
    echo "🔗 链接检查 (应该是 statically linked):"
    ldd "$BINARY_PATH" 2>&1 || echo "   ✓ 静态链接，无外部依赖"
    
    echo ""
    echo "✅ 构建成功！"
    echo "   二进制文件位置：$BINARY_PATH"
    echo ""
    echo "运行方式:"
    echo "   ./$BINARY_PATH --help"
else
    echo "❌ 构建失败：未找到二进制文件"
    exit 1
fi
