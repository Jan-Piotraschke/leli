import os
from dotenv import load_dotenv
from huggingface_hub import login
import transformers
import torch
from bs4 import BeautifulSoup

# Load the .env file
load_dotenv()

# Get the token from the environment variables
hf_token = os.getenv("HUGGINGFACE_API_KEY")

# Log in using the token
login(token=hf_token)

# Load and parse the local HTML file
with open("example.html", "r", encoding="utf-8") as file:
    html_content = file.read()

soup = BeautifulSoup(html_content, "html.parser")
text_content = soup.get_text()

# Truncate the text content to the first 1000 characters
truncated_text_content = text_content[:1000]

# Choose the model to use
model_id = "distilgpt2"
# model_id = "/Users/jan-piotraschke/git_repos/leli/llama-models/models/llama3_1/Meta-Llama-3.1-8B-Instruct"

transformers.set_seed(42)
chat_model = transformers.pipeline(
    "text-generation",
    model=model_id,
    model_kwargs={"torch_dtype": torch.bfloat16},
    device="mps",
)


terminators = [
    chat_model.tokenizer.eos_token_id,
    chat_model.tokenizer.convert_tokens_to_ids("<|eot_id|>"),
]


# Define a function to interact with the model
def chat_with_model(prompt, max_new_tokens=50):
    response = chat_model(
        prompt,
        max_new_tokens=max_new_tokens,
        num_return_sequences=1,
        eos_token_id=terminators,
        do_sample=True,
        temperature=0.6,
        top_p=0.9,
    )
    return response[0]["generated_text"]


# Interactive chat loop
while True:
    user_input = input("You: ")
    if user_input.lower() in ["exit", "quit"]:
        print("Exiting chat.")
        break

    prompt = user_input + "\nHTML content:\n" + truncated_text_content
    # messages = [
    #     {
    #         "role": "system",
    #         "content": "You are a pirate chatbot who always responds in pirate speak!",
    #     },
    #     {"role": "user", "content": "Who are you?"},
    # ]

    # prompt = chat_model.tokenizer.apply_chat_template(
    #     messages, tokenize=False, add_generation_prompt=True
    # )

    response = chat_with_model(prompt)
    print("Model:", response)
