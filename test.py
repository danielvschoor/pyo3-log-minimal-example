import logging
import module

FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.INFO)
logging.info("Logging from Python")
s = module.WrappingPyStruct()
p = s.run() ## I usually start this in a multiprocessing.Process
