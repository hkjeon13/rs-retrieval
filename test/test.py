import json

import requests

import argparse


def test_indexing(args: argparse.Namespace):
    with open(args.file_path, "r", encoding="utf-8") as f:
        docs = json.load(f)

    data = {
        "group_id": "001",
        "user_id": "001",
        "session_id": "001",
        "documents": docs
    }

    with requests.post(f"http://{args.host}:{args.port}/indexing", json=data) as response:
        print(response.status_code)
        assert response.status_code == 200
        response = response.json()
    print(response)

def test_predict(args: argparse.Namespace):
    test_indexing(args)
    data = {
        "group_id": "001",
        "user_id": "001",
        "session_id": "001",
        "query": {"query_id": "0001", "text": "바클레이즈로부터 투자의견-하향으로 평가 받은 기업은?"},
        "top_k": 3,
    }

    with requests.post(f"http://{args.host}:{args.port}/search", json=data) as response:
        print(response.text)
        assert response.status_code == 200, "Response Error!"
        output = response.json()

    print("##### Prediction Output #####")
    print(json.dumps(output, indent=2, ensure_ascii=False))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--host", type=str, default="0.0.0.0")
    parser.add_argument("--port", type=int, default=3000)
    parser.add_argument("--file_path", type=str, default="test/sample_document.json")
    test_indexing(parser.parse_args())
    test_predict(parser.parse_args())
