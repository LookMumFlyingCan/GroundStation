import socket
from pyModeS import adsb
from pyModeS.decoder.bds.bds08 import category, callsign
from pyModeS.decoder.bds.bds61 import emergency_squawk

import time

IP = '127.0.0.1'
TXPORT = 2137
RXPORT = 2138
BUFFER = 60

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
                PLANES[icao] = [(-1, -1),   'unk'  , -1, (-1,-1,-1,''),  (None, 0),    (None, 0),     ''  ]

            if typecode == 19 or 5 <= typecode <= 8:
                PLANES[icao][3] = adsb.velocity(data)

            if 9 <= typecode <= 18 or 20 <= typecode <= 22:
                PLANES[icao][4+adsb.oe_flag(data)] = (data, time.time())

                print(PLANES[icao][4], ' ', PLANES[icao][5])
                if PLANES[icao][4][0] != None and PLANES[icao][5][0] != None:
                    PLANES[icao][0] = adsb.position(PLANES[icao][4][0], PLANES[icao][5][0], PLANES[icao][4][1], PLANES[icao][5][1])

                PLANES[icao][2] = adsb.altitude(data)

            if 1 <= typecode <= 4:
                PLANES[icao][6] = callsign(data)

            if typecode == 28:
                PLANES[icao][1] = emergency_squawk(data)


            print(PLANES)
        #except:
        #    print('dupsztyl')


    conn.close()


def thread(window, PLANES):
    #register()

    while 1:
        window.repaint()
        listen(PLANES)
