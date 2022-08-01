import matlab.engine
eng = matlab.engine.start_matlab()
eng.celldetect("../iimg/Test_550nm.jpg",nargout=0)