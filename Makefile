all: $(patsubst %.cc,%,$(wildcard [0-9][0-9].cc))

%: %.cc
	c++ -std=c++14 -O3 -o $@ $^

.PHONY: clean

clean:
	rm -f [0-9][0-9]