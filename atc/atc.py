from PyQt5 import QtGui
from PyQt5.QtWidgets import QApplication, QMainWindow
from PyQt5.QtGui import QPainter, QBrush, QPen, QFont
from PyQt5.QtCore import Qt, QPoint
from pyModeS import adsb
import os
import time
from plane import Plane

import sys
import packets
import threading

_FONT_ = 'Dogica Pixel'

class Window(QMainWindow):
    def __init__(self, PLANES):
        super().__init__()
        self.title = "Air Traffic Control"
        self.top = 0
        self.left = 0
        self.width = 680
        self.height = 500
        self.boxoffset = 20
        self.dash = 5
        self.PLANES = PLANES

        self.vsdeg = 50
        self.vedeg = 51
        self.hsdeg = 19
        self.hedeg = 24

        self.station = True
        self.stationy = 50.1179528
        self.stationx = 19.920111

        self.setStyleSheet("background-color: black;")
        self.InitWindow()

        self.w = PlaneWindow(self)
        self.w.show()


    def InitWindow(self):
        self.setWindowTitle(self.title)
        self.setGeometry(self.top, self.left, self.width, self.height-30)
        self.show()

    def resizeEvent(self, e):
        self.width = self.frameGeometry().width()
        self.height = self.frameGeometry().height()-30

    def keyPressEvent(self, event):
        if event.key() == 16777234 and self.hedeg < 180:
            self.hedeg -= 1
            self.hsdeg -= 1
        elif event.key() == 16777236:
            self.hedeg += 1
            self.hsdeg += 1
        elif event.key() == 16777235 and self.vsdeg > 0:
            self.vedeg -= 1
            self.vsdeg -= 1
        elif event.key() == 16777237 and self.vedeg < 90:
            self.vedeg += 1
            self.vsdeg += 1
        elif event.key() == 45:
            self.hedeg += 1
            self.hsdeg -= 1
        elif event.key() == 47 and self.vedeg - self.vsdeg > 1:
            self.vedeg -= 1
            self.vsdeg += 1
        elif event.key() == 42:
            self.vedeg += 1
            self.vsdeg -= 1
        elif event.key() == 43 and self.hedeg - self.hsdeg > 1:
            self.hedeg -= 1
            self.hsdeg += 1
        elif event.key() == 83:
            self.station = not self.station
        elif event.key() == 81:
            self.close()

        self.repaint()
        event.accept()

    def paintEvent(self, e):
        print("repaint!")
        print(self.PLANES)

        painter = QPainter(self)

        fn = QFont(_FONT_, 7)

        painter.setFont(fn)

        self.drawBox(painter)
        self.drawATCScale(painter, self.vsdeg, self.vedeg, self.hsdeg, self.hedeg, 10, 15)

        if self.station:
            self.drawStation(painter)

        index = 0
        for idx in self.PLANES:
            plane = self.PLANES[idx]
            if plane.position == None:
                painter.setPen(QPen(Qt.red, 3, Qt.SolidLine))
                painter.drawText(self.boxoffset + 10, self.height-self.boxoffset-10, str(plane.callsign))
            else:
                #print(plane)
                self.drawPlane(painter, plane, self.w.select == index)

            index += 1

        self.w.PLANES = self.PLANES
        self.w.repaint()


    def drawStation(self, painter):
        x = self.stationx
        y = self.stationy
        x -= self.hsdeg
        y -= self.vsdeg

        x /= self.hedeg - self.hsdeg
        y /= self.vedeg - self.vsdeg
        x *= self.width - 2*self.boxoffset
        y *= self.height - 2*self.boxoffset
        x += self.boxoffset
        y += self.boxoffset
        x = round(x)
        y = round(y)
        y = (self.height - y)


        painter.setPen(QPen(Qt.yellow, 1, Qt.SolidLine))
        painter.drawText(x, y, 'S')

        painter.setPen(QPen(Qt.yellow, 1, Qt.DashDotLine))
        painter.drawEllipse(QPoint(x, y), ((self.width - 2*self.boxoffset) / (self.hedeg - self.hsdeg) * 0.5), ((self.height - 2*self.boxoffset) / (self.vedeg - self.vsdeg) * 0.5) )

        painter.setPen(QPen(Qt.red, 1, Qt.DashLine))
        painter.drawEllipse(QPoint(x, y), ((self.width - 2*self.boxoffset) / (self.hedeg - self.hsdeg) * 2), ((self.height - 2*self.boxoffset) / (self.vedeg - self.vsdeg) * 2) )
        painter.drawEllipse(QPoint(x, y), ((self.width - 2*self.boxoffset) / (self.hedeg - self.hsdeg)), ((self.height - 2*self.boxoffset) / (self.vedeg - self.vsdeg)) )

    def drawBox(self, painter):
        painter.setPen(QPen(Qt.blue, 1, Qt.SolidLine))
        for i in range(self.boxoffset, self.width - self.boxoffset, self.dash*2):
            painter.drawLine(i, self.height - self.boxoffset, i+self.dash, self.height - self.boxoffset)
        for i in range(self.boxoffset, self.width - self.boxoffset, self.dash*2):
            painter.drawLine(i, self.boxoffset, i+self.dash, self.boxoffset)
        for i in range(self.boxoffset, self.height - self.boxoffset, self.dash*2):
            painter.drawLine(self.width - self.boxoffset, i, self.width - self.boxoffset, i+self.dash)
        for i in range(self.boxoffset, self.height - self.boxoffset, self.dash*2):
            painter.drawLine(self.boxoffset, i, self.boxoffset, i+self.dash)


    def drawPlane(self, painter, aero, logic):
        x = aero.position[1]
        y = aero.position[0]

        print('drawing ', x, ' ', y)
        x -= self.hsdeg
        y -= self.vsdeg

        x /= self.hedeg - self.hsdeg
        y /= self.vedeg - self.vsdeg
        x *= self.width - 2*self.boxoffset
        y *= self.height - 2*self.boxoffset
        x += self.boxoffset
        y += self.boxoffset
        x = round(x)
        y = round(y)

        y = (self.height - y)

        if logic:
            painter.setPen(QPen(Qt.green, 3, Qt.SolidLine))
        else:
            painter.setPen(QPen(Qt.gray, 3, Qt.SolidLine))

        painter.drawPoint(x, y)
        painter.drawPoint(x, y+1)
        painter.drawPoint(x+1, y)
        painter.drawPoint(x+1, y+1)



    def drawATCScale(self, painter, starty, stopy, startx, stopx, partsy, partsx):
        painter.setPen(QPen(Qt.blue, 1, Qt.SolidLine))

        rangey = stopy - starty
        rangex = stopx - startx
        rangey *= 60
        rangex *= 60

        for i in range(0, partsy+1, 1):
            painter.drawLine(self.width, i * ((self.height-2*self.boxoffset) // partsy) + self.boxoffset, self.width - self.boxoffset - 10, i * ((self.height-2*self.boxoffset) // partsy) + self.boxoffset)
        for i in range(0, partsx+1, 1):
            painter.drawLine(i * ((self.width-2*self.boxoffset) // partsx) + self.boxoffset, 0, i * ((self.width-2*self.boxoffset) // partsx) + self.boxoffset, self.boxoffset + 10)



        for i in range(stopy, starty-1, -1):
            painter.drawText(self.width - self.boxoffset - 30, (stopy - i) * ((self.height - 2*self.boxoffset) // (stopy - starty)) + self.boxoffset + 10, str(i) + '°')
        for i in range(startx, stopx+1, 1):
            painter.drawText((i - startx) * ((self.width - 2*self.boxoffset) // (stopx - startx)) + self.boxoffset + 5, self.boxoffset + 25, str(i) + '°')

class PlaneWindow(QMainWindow):
    def __init__(self, up):
        super().__init__()
        self.title = "Aeroplanes"
        self.top = 0
        self.left = 0
        self.width = 680
        self.height = 500
        self.dash = 10
        self.index = 0
        self.select = 0
        self.PLANES = dict()
        self.up = up

        self.setStyleSheet("background-color: black;")
        self.InitWindow()


    def InitWindow(self):
        self.setWindowTitle(self.title)
        self.setGeometry(self.top, self.left, self.width, self.height)
        self.show()

    def resizeEvent(self, e):
        self.width = self.frameGeometry().width()
        self.height = self.frameGeometry().height()

    def keyPressEvent(self, event):
        if event.key() == 16777235:
            self.select -= 1
        elif event.key() == 16777237:
            self.select += 1

        self.up.repaint()
        self.repaint()
        event.accept()

    def paintEvent(self, e):
        print('defa ')
        print(PLANES)
        self.index = 0
        painter = QPainter(self)

        fn = QFont(_FONT_, 7)

        painter.setFont(fn)

        for idx in self.PLANES:
            plane = self.PLANES[idx]
            self.drawPlane(painter, idx, plane)


    def drawPlane(self, painter, icao, aero):
        print('drawing ', aero.altitude, ' ', aero.callsign)

        if self.index == self.select:
            painter.setPen(QPen(Qt.green, 3, Qt.SolidLine))
        else:
            painter.setPen(QPen(Qt.white, 3, Qt.SolidLine))


        painter.drawText(5, self.dash*self.index + 20, str(icao) + ' |*| ' + str(aero.callsign.strip('_')) + ' |-| ' + str(aero.altitude) + 'ft |-| ' + str(round(aero.velocity[1])) + '/' + str(aero.velocity[0]) + 'kt |_| ' + str(aero.squawk) + ' time since last frame: ' + str(round(time.time() - aero.timed)))
        self.index += 1



#global stop

PLANES = dict()

App = QApplication(sys.argv)
window = Window(PLANES)

packet_handler = threading.Thread(target=packets.thread, args=[PLANES])
painter = threading.Thread(target=packets.painter, args=[window])
packet_handler.start()
painter.start()

#stop = True
App.exec()
os._exit(0)
