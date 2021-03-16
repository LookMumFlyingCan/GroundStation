import socket
import time
from pyModeS import adsb
from pyModeS.decoder.bds.bds08 import category, callsign
from pyModeS.decoder.bds.bds61 import emergency_squawk

import plane
import time

IP = '127.0.0.1'
TXPORT = 2137
RXPORT = 2138
BUFFER = 128

global LAST_FRAME

def register():
    global LAST_FRAME
    LAST_FRAME = -1

    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.connect((IP, TXPORT))
    s.send(b"\a\a")
    s.close()

def listen(PLANES):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.bind((IP, RXPORT))

    s.listen(1)

    while 1:
        conn, addr = s.accept()
        data = conn.recv(BUFFER)

        if not data:
            break


        data = data.decode('utf-8').strip('\x00')
        if True:
            typecode = adsb.typecode(data)

            icao = adsb.icao(data)

            if icao == None:
                continue

            icao = icao.lower()

            print('got message from ', icao, ' with typecode ', typecode, ' ', data)
            if not icao in PLANES:
                #               position  squawk  alt    velocity    even frame  odd frame callsign
                PLANES[icao] = plane.Plane()

            PLANES[icao].time = time.time()

            if typecode == 19 or 5 <= typecode <= 8:
                PLANES[icao].velocity = adsb.velocity(data)

            if 9 <= typecode <= 18 or 20 <= typecode <= 22:
                if adsb.oe_flag(data) == 0:
                    PLANES[icao].even = (data, time.time())

                print(PLANES[icao][4], ' ', PLANES[icao][5])
                if PLANES[icao].even != None and PLANES[icao].even != None:
                    PLANES[icao].position = adsb.position(PLANES[icao].even[0], PLANES[icao].odd[0], PLANES[icao].even[1], PLANES[icao].odd[1])

                PLANES[icao].altitude = adsb.altitude(data)

            if 1 <= typecode <= 4:
                PLANES[icao].callsign = callsign(data)

            if typecode == 28:
                PLANES[icao].squawk = emergency_squawk(data)


            print(PLANES)


    conn.close()


def thread(PLANES):
    #register()

    listen(PLANES)

def painter(window):
    while True:
        time.sleep(1)
        window.update()
