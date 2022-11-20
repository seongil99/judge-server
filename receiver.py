import pika
import json

HOST_NAME = 'localhost'
QUEUE_NAME = 'to_spring'

connection = pika.BlockingConnection(
    pika.ConnectionParameters(host=HOST_NAME))
channel = connection.channel()
channel.queue_declare(queue=QUEUE_NAME)

def callback(ch, method, properties, body):
    print(" [x] Received %r" % body.decode())
    ch.basic_ack(delivery_tag=method.delivery_tag)


channel.basic_qos(prefetch_count=1)
channel.basic_consume(queue=QUEUE_NAME, on_message_callback=callback)

channel.start_consuming()
