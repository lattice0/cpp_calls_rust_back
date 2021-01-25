#include <exception>
#include <iostream>

struct Callbacks {
  void *user_data;
  int (*on_read)(char *buffer, int len, void *user_data);
  int (*on_write)(const char *buffer, int len, void *user_data);
  void (*destroy)(void *user_data);
};

class OpenVpnClient {
public:
  OpenVpnClient(Callbacks cb) : callbacks(cb) {}
  OpenVpnClient(OpenVpnClient &&client) {
    callbacks = client.callbacks;
    client.callbacks = Callbacks{};
  }
  OpenVpnClient &operator=(OpenVpnClient &&client) {
    callbacks = client.callbacks;
    client.callbacks = Callbacks{};
    return *this;
  }

  OpenVpnClient(OpenVpnClient &) = delete;
  OpenVpnClient &operator=(OpenVpnClient &) = delete;

  ~OpenVpnClient() {
    if (callbacks.destroy) {
      callbacks.destroy(callbacks.user_data);
    }
  }

public:
  Callbacks callbacks;
};

extern "C" {
OpenVpnClient *openvpn_client_new(Callbacks callbacks) {
  return new OpenVpnClient(callbacks);
}

int openvpn_client_run(OpenVpnClient *client) {
  try {
    // do stuff, calling client.callbacks.on_read and friends when
    // things happen.
    char* b;
    int r = client->callbacks.on_read(b, 5, client->callbacks.user_data);
    return 0;
  } catch (std::exception &e) {
    return -1;
  }
}

void openvpn_client_free(OpenVpnClient *client) { delete client; }
}

//int main() { return 0; }
