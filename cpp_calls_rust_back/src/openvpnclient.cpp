#include <exception>
#include <iostream>

struct Callbacks {
  void *user_data;
  int (*on_read)(uint8_t *buffer, int size_t, void *user_data);
  int (*on_read_allocate)(uint8_t **buffer, size_t* size, void *user_data);
  int (*on_write)(const uint8_t *buffer, int size_t, void *user_data);
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
      uint8_t* b;
      size_t size;
      std::cout << "gonna on_read_allocate" << std::endl;
      int r = client->callbacks.on_read_allocate(&b, &size, client->callbacks.user_data);
      std::cout << "DID on_read_allocate" << std::endl;
      if (r!=0) {
        throw std::invalid_argument("error on on_read_allocate");
      }
      std::cout << "received array: " << std::endl;
      for (size_t i=0; i<size; i++) {
        std::cout << (int) b[i];
      }
      std::cout << std::endl;
      delete[] b;
      return 0;
    } catch (std::exception &e) {
      return -1;
    }
  }

  void openvpn_client_free(OpenVpnClient *client) { 
    delete client; 
  }

  uint8_t* openvpn_client_allocate(size_t size) {
    std::cout << "c++ openvpn_client_allocate for size " << size << std::endl;
    return new uint8_t[size]();
  }
}