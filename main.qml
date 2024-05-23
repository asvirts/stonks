import QtQuick
import QtQuick.Controls.Basic

ApplicationWindow {
    visible: true
    width: 100
    height: 500
    title: "HelloApp"
    Text {
        anchors.centerIn: parent
        text: "Hi mom!"
        font.pixelSize: 24
    }
}