#pragma once

#define DEBUG

#include <string>
#include <wpi/Twine.h>

namespace team114
{
namespace c2019
{
namespace vision
{

const std::string CAM_FORWARD_ID("/dev/v4l/by-path/platform-70090000.xusb-usb-0:4:1.0-video-index0");
const std::string CAM_REVERSE_ID("/dev/v4l/by-path/platform-70090000.xusb-usb-0:3.3:1.0-video-index0");

const int MJPEG_FORWARD_PORT = 5808;
const int MJPEG_REVERSE_PORT = 5809;
const int MJPEG_WIDTH = 320;
const int MJPEG_HEIGHT = 240;
const int MJPEG_FPS = 30;

const std::string RIO_VISION_ADDR("0.0.0.0");
const std::string RIO_VISION_PORT("5808");

} // namespace vision
} // namespace c2019
} // namespace team114
