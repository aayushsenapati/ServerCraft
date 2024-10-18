
import socket

def main():
    try:
        # Connect to the server
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.connect(("localhost", 9191))

        # Send the status request
        sock.sendall(b'\xfe')

        # Read the response
        response = bytearray()
        while True:
            byte = sock.recv(1)
            if not byte:
                break
            if byte[0] != 0 and byte[0] > 16 and byte[0] != 255 and byte[0] != 23 and byte[0] != 24:
                response.append(byte[0])

        # Decode the response as UTF-16
        data = response.decode('utf-16be')
        
        # Split the data by the section character
        info = data.split('§')
        
        # Parse the MOTD and player counts
        server_motd = info[0]
        online_players = int(info[1])
        max_players = int(info[2])

        # Display the information nicely
        print("=== Server Information ===")
        print(f"MOTD: \"{server_motd}\"")
        print(f"Online Players: {online_players}/{max_players}")
        
    except Exception as e:
        print(f"An error occurred: {e}")
    finally:
        sock.close()

if __name__ == "__main__":
    main()
