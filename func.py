import argparse

argp = argparse.ArgumentParser()
argp.add_argument('--city_id', default="bar")
args = argp.parse_args()
print(args)

def func_with_default_arg(city_id="12345"):
  print(city_id)

func_with_default_arg()
func_with_default_arg(None)
func_with_default_arg("98765")
func_with_default_arg(args.city_id)