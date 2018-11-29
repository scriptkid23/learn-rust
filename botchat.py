# -*- coding: utf-8 -*-
from fbchat import Client, log
from fbchat.models import *
import apiai, codecs, json


class Jarvis(Client):

    # Connect to dialogflow
    def apiai(self):
        self.ClientAccessToken = '9427bc91e4f741098311f5722951a878'
        self.ai = apiai.ApiAI(self.ClientAccessToken)
        self.request = self.ai.text_request()
        self.request.lang = 'vi'
        self.request.session_id = "<SESSION ID, UNIQUE FOR EACH USER>"


    def onMessage(self, author_id=None, message=None,thread_id=None, thread_type=ThreadType.USER, **kwargs):
        # marking the message as read
        self.markAsRead(author_id)
        # printing to terminal as a message is received.
        # printing message text
        print("The received message - ", message)
        try:
            # setting up connection with apiai
            self.apiai()
            # sending the query (message received)
            self.request.query = message
            # getting the json response
            api_response = self.request.getresponse()
            json_reply = api_response.read()
            # decoding to utf-8 (converting byte object to json format)
            decoded_data = json_reply.decode("utf-8")
            # loading it into json
            response = json.loads(decoded_data)
            # taking out the reply from json
            reply = response['result']['fulfillment']['speech']
        except Exception as e:
            print(e)
            reply = "sorry, ĐỖ HOÀN ĐANG BẬN HÃY GỌI 0975164536."

        # if we are not the sender of the message
        if author_id != self.uid:
            # sending the message i.e. the reply string.
            self.sendMessage(reply, thread_id=thread_id, thread_type=thread_type)

        self.markAsDelivered(author_id, thread_id)


# Create an object of our class, enter your email and password for facebook.
client = Jarvis("teamhacker1999@gmail.com", "sIeunhannhen")

# Listen for new message
client.listen()