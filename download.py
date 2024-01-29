from transformers import AutoTokenizer, AutoModel

tokenizer = AutoTokenizer.from_pretrained("psyche/klue-single-embedding-25000", token="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")
model = AutoModel.from_pretrained("psyche/klue-single-embedding-25000", token="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")

model.save_pretrained("model/")
tokenizer.save_pretrained("model/")