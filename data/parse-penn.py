lines = None
with open('data/penn.csv', 'r') as f:
    lines = f.readlines()

sentences = []
curr_sentence = []
for line in lines:
    if line == ',",",",",O\n':
        sentence_num = ''
        word = ','
        pos = ','
    else:
        sentence_num, line = line[:-1].split(',', 1)
        word, pos, _ = line.rsplit(',', 2)
    
    if sentence_num and curr_sentence:
        sentences.append(' '.join(curr_sentence) + '\n')
        curr_sentence.clear()
    
    curr_sentence.append(f"{word}={pos}")

if curr_sentence:
    sentences.append(' '.join(curr_sentence))

train_lines = []
dev_lines = []

import random
for i, line in enumerate(sentences):
    if random.random() < 0.2:
        dev_lines.append(line)
    else:
        train_lines.append(line)

with open('data/penn-dev.txt', 'w') as f:
    f.writelines(dev_lines)

with open('data/penn-training.txt', 'w') as f:
    f.writelines(train_lines)