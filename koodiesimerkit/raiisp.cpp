class widget
{
private:
    std::unique_ptr<int> data;
public:
    widget(const int size) { data = std::make_unique<int>(size); }
    void do_something() {}
};
void functionUsingWidget() {
    // lifetime automatically tied to enclosing scope
    widget w(1000000);
    // constructs w, including the w.data gadget member
    w.do_something();
} // automatic destruction and deallocation for w and w.data