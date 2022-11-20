import pika
import json

HOST_NAME = 'localhost'
QUEUE_NAME = 'to_rust'

data = {
    'answer_id': 42,
    'language': 'C',
    'code': '#include<stdio.h>\n  int main()\n  {\n     int a, b;\n    scanf(\"%d %d\", &a, &b);\n    printf(\"%d\", a+b);\n    return 0;\n }\n',
    'testcases': [
        {
            'input': '2 3',
            'output': '5'
        },
        {
            'input': '3 4',
            'output': '7'
        }
    ]
}

payload = json.dumps(data)

connection = pika.BlockingConnection(
    pika.ConnectionParameters(host=HOST_NAME))
channel = connection.channel()
channel.queue_declare(queue=QUEUE_NAME)
channel.basic_publish(exchange='', routing_key=QUEUE_NAME, body=payload)
print('Sent: ', payload)
connection.close()
