set -x
bin/opencv_interactive-calibration -v=/dev/video1 -of=${CAM_SERIAL_NO}.xml "$(< circle-board-params.sh)"
set +x
