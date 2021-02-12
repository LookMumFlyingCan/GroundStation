from PyQt5 import QtGui
from PyQt5.QtWidgets import QApplication, QMainWindow
from PyQt5.QtGui import QPainter, QBrush, QPen, QFont
from PyQt5.QtCore import Qt, QPoint
from pyModeS import adsb
import os

import sys
import packets
import threading


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


    def InitWindow(self):
        self.setWindowTitle(self.title)
        self.setGeometry(self.top, self.left, self.width, self.height-30)
        self.show()

    def resizeEvent(self, e):
        self.width = self.frameGeometry().width()
        self.height = self.frameGeometry().height()-30
        self.repaint()

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
        #global PLANES
        print(self.PLANES)

        painter = QPainter(self)

        fn = QFont('Dogica Pixel', 7)

        painter.setFont(fn)

        self.drawBox(painter)
        self.drawATCScale(painter, self.vsdeg, self.vedeg, self.hsdeg, self.hedeg, 10, 15)

        if self.station:
            self.drawStation(painter)
        #self.drawPlane(painter, 19, 50, 213, 234, 234, len(self.PLANES), 123)
        for idx in self.PLANES:
            plane = self.PLANES[idx]
            if plane[0][0] == -1:
                painter.setPen(QPen(Qt.red, 3, Qt.SolidLine))
                painter.drawText(self.boxoffset + 10, self.height-self.boxoffset-10, str(plane[6]))
            else:
                #print(plane)
                self.drawPlane(painter, plane[0][1], plane[0][0], plane[2], plane[3][1], plane[3][0], plane[1], plane[6])


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


    def drawPlane(self, painter, x, y, alt, bear, vel, sqwk, callsign):
        print('drawing ', x, ' ', y, ' ', alt, ' ', callsign)

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
        painter.setPen(QPen(Qt.red, 3, Qt.SolidLine))
        painter.drawPoint(x, y)
        painter.drawPoint(x, y+1)
        painter.drawPoint(x+1, y)
        painter.drawPoint(x+1, y+1)

        painter.setPen(QPen(Qt.green, 2, Qt.SolidLine))
        painter.drawLine(x,y,x - 15, y - 15)
        painter.drawLine(x-15,y-15,x -50, y - 15)

        painter.setPen(QPen(Qt.magenta, 1, Qt.SolidLine))
        painter.drawText(x-50, y-20, str(alt) + 'ft')
        painter.drawText(x-53, y-35, str(round(bear)) + '/' + str(vel))
        painter.drawText(x-50, y+20, str(sqwk))
        painter.drawText(x-5, y+20, str(callsign.strip('_')))



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


#global stop

PLANES = dict()

App = QApplication(sys.argv)
window = Window(PLANES)

packet_handler = threading.Thread(target=packets.thread, args=[window, PLANES])
packet_handler.start()

#stop = True
App.exec()
os._exit(0)
