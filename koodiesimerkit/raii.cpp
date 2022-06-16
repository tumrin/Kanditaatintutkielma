class widget
{
private:
    int* data;
public:
    widget(const int size) { data = new int[size]; } // acquire
    ~widget() { delete[] data; } // release
    void do_something() {}
};
void functionUsingWidget() {
    // lifetime automatically tied to enclosing scope
    widget w(1000000);
    // constructs w, including the w.data member
    w.do_something();
} // automatic destruction and deallocation for w and w.data