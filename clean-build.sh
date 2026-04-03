#!/bin/bash
# 清理构建缓存和锁文件

echo "Cleaning build artifacts..."
rm -rf target/
rm -f Cargo.lock

echo "Build cache cleaned successfully!"
echo "Now you can commit and push to trigger GitHub Actions"
