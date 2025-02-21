# Hugging Face Mirror Proxy

## 介绍

本项目是一个 Hugging Face 镜像代理服务，基于 `axum` 框架构建，支持 API 请求和文件分发。

## 功能

- 代理 Hugging Face API 请求，支持 `tree/main` 结构查询。
- 代理数据集和模型文件的下载请求。
- 通过环境变量配置监听地址和本地存储路径。

## 环境变量

| 变量名 | 默认值 | 说明 |
|--------|--------|------|
| `HF_MIRROR_PATH` | `/hf_mirror` | 本地 Hugging Face 镜像存储路径 |
| `HF_MIRROR_HOST` | `127.0.0.1` | 监听地址 |
| `HF_MIRROR_PORT` | `8080` | 监听端口 |

## 运行方式

```sh
cargo run
# or
docker run -d -p 8080:8080 ghcr.io/lisoboss/hf-mirror:latest
```

## 访问示例

```sh
# 安装依赖
pip install -U huggingface_hub
# 设置环境变量
export HF_ENDPOINT=http://127.0.0.1:8000
# 下载模型
huggingface-cli download --resume-download gpt2 --local-dir gpt2
```

## huggingface 工具链访问示例

```sh
# 设置环境变量
export HF_ENDPOINT=http://127.0.0.1:8000
python your_script.py
```
