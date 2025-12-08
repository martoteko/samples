int open_wmx();
int close_wmx();
double get_pos(int axis);

int start_memlog(int axis);
int stop_memlog();
int get_memlog(double pos[1000], long long cycle_counter[1000], size_t* pCount);
